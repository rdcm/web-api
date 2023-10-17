pub struct AppConf {
    pub api_host: String,
    pub api_port: i32,
    pub db_uri: String,
    pub db_name: String,
}

impl AppConf {
    pub fn new() -> AppConf {
        Self {
            db_name: std::env::var("DB_NAME").unwrap_or_else(|_| "test".into()),
            api_host: std::env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            api_port: std::env::var("API_PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse::<i32>()
                .expect("wrong api port"),
            db_uri: std::env::var("DB_URI")
                .unwrap_or_else(|_| "mongodb://0.0.0.0:27017/test".into()),
        }
    }
}

impl AppConf {
    pub fn get_api_address(&self) -> String {
        format!("{}:{}", self.api_host, self.api_port)
    }
}

impl Default for AppConf {
    fn default() -> Self {
        Self::new()
    }
}
