use parking_lot::RwLock;
use rocksdb::DB;
use snafu::ResultExt;
use std::{collections::HashMap, sync::Arc, time::Duration};

use parity_processbot::{bamboo, bots, config, error, github_bot, matrix_bot};

const GITHUB_TO_MATRIX_KEY: &str = "GITHUB_TO_MATRIX";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	match run().await {
		Err(error) => panic!("{}", error),
		_ => Ok(()),
	}
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
	let config = config::MainConfig::from_env();
	env_logger::from_env(env_logger::Env::default().default_filter_or("info"))
		.init();

	let db = DB::open_default(&config.db_path)?;

	let matrix_bot = matrix_bot::MatrixBot::new_with_token(
		&config.matrix_homeserver,
		&config.matrix_access_token,
		&config.matrix_default_channel_id,
		config.matrix_silent,
	)?;
	log::info!(
		"Connected to matrix homeserver {}",
		config.matrix_homeserver,
	);

	let github_bot = github_bot::GithubBot::new(
		config.private_key.clone(),
		&config.installation_login,
	)
	.await?;
	log::info!("Connected to Github account {}", config.installation_login);

	// the bamboo queries can take a long time so only wait for it
	// if github_to_matrix is not in the db. otherwise update it
	// in the background and start the main loop
	if db
		.get_pinned(&GITHUB_TO_MATRIX_KEY)
		.context(error::Db)?
		.is_none()
	{
		// block on bamboo
		match bamboo::github_to_matrix(&config.bamboo_token) {
			Ok(github_to_matrix) => {
				db.put(
					&GITHUB_TO_MATRIX_KEY,
					serde_json::to_string(&github_to_matrix)
						.context(error::Json)?
						.as_bytes(),
				)
				.context(error::Db)?;
			}
			Err(e) => {
				log::error!("Error fetching employee data from Bamboo: {}", e)
			}
		}
	}

	let db = Arc::new(RwLock::new(db));

	let db_clone = db.clone();
	let config_clone = config.clone();

	// update github_to_matrix on another thread
	std::thread::spawn(move || loop {
		match bamboo::github_to_matrix(&config_clone.bamboo_token).and_then(
			|github_to_matrix| {
				let db = db_clone.write();
				db.delete(&GITHUB_TO_MATRIX_KEY)
					.context(error::Db)
					.map(|_| {
						serde_json::to_string(&github_to_matrix)
							.context(error::Json)
							.map(|s| {
								db.put(&GITHUB_TO_MATRIX_KEY, s.as_bytes())
									.context(error::Db)
							})
					})
			},
		) {
			Ok(_) => {}
			Err(e) => log::error!("Bamboo error: {}", e),
		}
		std::thread::sleep(Duration::from_secs(config_clone.bamboo_tick_secs));
	});

	let mut interval =
		tokio::time::interval(Duration::from_secs(config.main_tick_secs));

	let mut bot =
		bots::Bot::new(db, github_bot, matrix_bot, vec![], HashMap::new());

	loop {
		interval.tick().await;

		let core_devs = match bot.github_bot.team("core-devs").await {
			Ok(team) => bot.github_bot.team_members(team.id).await?,
			_ => vec![],
		};

		let github_to_matrix = bot
			.db
			.read()
			.get(&GITHUB_TO_MATRIX_KEY)
			.context(error::Db)?
			.and_then(|ref v| {
				serde_json::from_slice::<HashMap<String, String>>(v).ok()
			})
			.unwrap_or_else(|| {
				log::error!("Bamboo data not found in DB");
				HashMap::new()
			});

		bot.core_devs = core_devs;
		bot.github_to_matrix = github_to_matrix;
		bot.update().await?;
	}
}
