use actix_web::rt;
use app::models::{CreateUserRequest, CreatedUserIdResponse, UserResponse};
use host::composition::Composition;
use host::conf::AppConf;
use reqwest::StatusCode;

#[derive(Clone)]
pub struct Sut {
    base_url: String,
}

impl Sut {
    pub async fn new() -> Sut {
        let conf = AppConf {
            api_host: "127.0.0.1".to_string(),
            api_port: 0,
            db_uri: "mongodb://127.0.0.1/test".to_string(),
            db_name: "test".to_string(),
        };

        let info = Composition::new(&conf).await.unwrap();
        let base_url = format!("http://{}:{}", info.addrs[0].ip(), info.addrs[0].port());

        rt::spawn(info.server);

        Self { base_url }
    }

    pub async fn get_user(&self, id: String) -> Result<UserResponse, String> {
        let client = reqwest::Client::new();
        let uri = format!("{}/user/{}", self.base_url, id);

        let response = client.get(uri).send().await.unwrap();

        if response.status() == StatusCode::OK {
            let user: UserResponse = response.json().await.unwrap();

            Ok(user)
        } else {
            Err(response.text().await.unwrap())
        }
    }

    pub async fn create_user(self, name: String, age: u8) -> Result<CreatedUserIdResponse, String> {
        let client = reqwest::Client::new();
        let uri = format!("{}/user", self.base_url);

        let response = client
            .post(uri)
            .json(&CreateUserRequest { name, age })
            .send()
            .await
            .unwrap();

        if response.status() == StatusCode::CREATED {
            let user: CreatedUserIdResponse = response.json().await.unwrap();

            Ok(user)
        } else {
            Err(response.text().await.unwrap())
        }
    }
}
