use deadpool_diesel::postgres::Pool; // Ou le Pool approprié

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
}
