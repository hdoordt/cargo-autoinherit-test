use cargo_autoinherit_test_config::Config;
use cargo_autoinherit_test_db::{connect_pool, DbPool};

/// The application's state that is available in [`crate::controllers`] and [`crate::middlewares`].
#[derive(Clone)]
pub struct AppState {
    /// The database pool that's used to get a connection to the application's database (see [`cargo_autoinherit_test_db::DbPool`]).
    pub db_pool: DbPool,
}

/// Initializes the application state.
///
/// This function creates an [`AppState`] based on the current [`cargo_autoinherit_test_config::Config`].

pub async fn init_app_state(config: Config) -> AppState {
    let db_pool = connect_pool(config.database)
        .await
        .expect("Could not connect to database!");

    AppState { db_pool }
}
