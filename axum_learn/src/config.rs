use sea_orm::DatabaseConnection;

pub struct Settings {
    pub database_url: String,
    pub server_port: u16,
}
