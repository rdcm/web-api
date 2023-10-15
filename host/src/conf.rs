pub struct AppConf {
    pub api_host: String,
    pub api_port: i32,
    pub connection_string: String,
}

pub fn read_app_conf() -> AppConf {
    AppConf {
        api_host: std::env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
        api_port: std::env::var("API_PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse::<i32>()
            .expect("wrong api port"),
        connection_string: std::env::var("DB_CONNECTION_STRING")
            .unwrap_or_else(|_| "mongodb://0.0.0.0:27017/test".into()),
    }
}

impl AppConf {
    pub fn get_api_address(&self) -> String {
        format!("{}:{}", self.api_host, self.api_port)
    }
}
