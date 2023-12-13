use sqlx::{
	migrate::{Migration, MigrationType, Migrator},
	Pool, Postgres,
};
use std::path::{Path, PathBuf};

const MIGRATION_PATHS: &[&str] = &["../../migrations/20230227201309_create_tables.sql"];
const MIGRATIONS: &[&str] = &[include_str!("../../migrations/20230227201309_create_tables.sql")];

pub async fn migrate(db_conn: &Pool<Postgres>) -> Result<(), sqlx::Error> {
	let migrations = MIGRATIONS
		.iter()
		.zip(MIGRATION_PATHS.iter())
		.map(|(migration, path)| {
			let path = PathBuf::from(path);
			let (version, name) = {
				let file_name =
					path.file_name().expect("migration file name should be a valid file name");
				let file_name =
					file_name.to_str().expect("migration file name should be valid utf-8");
				let mut parts = file_name.splitn(2, '_');
				let version = parts.next().expect("migration file name should start with version");
				let version =
					version.parse::<i64>().expect("migration file name should start with version");
				let name = parts.next().expect("migration file name should contain a name");
				(version, name)
			};
			Migration::new(
				version,
				name.to_owned().into(),
				MigrationType::Simple,
				migration.to_owned().into(),
			)
		})
		.collect::<Vec<_>>();
	let migrator = Migrator { migrations: migrations.into(), ignore_missing: false, locking: true };
	migrator.run(db_conn).await?;
	Ok(())
}
