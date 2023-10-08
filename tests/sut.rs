use actix_web::rt;
use reqwest::StatusCode;
use web_api::app::models::{CreateUserRequest, CreatedUserIdResponse, UserResponse};
use web_api::root::composition_root::create_server;

#[derive(Clone)]
pub struct Sut {
    base_url: String,
}

impl Sut {
    pub async fn new() -> Sut {
        let info = create_server(0).await.unwrap();
        let base_url = format!("http://localhost:{}", info.addrs[0].port());

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
            Err("error".to_string())
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

        if response.status() == StatusCode::OK {
            let user: CreatedUserIdResponse = response.json().await.unwrap();

            Ok(user)
        } else {
            Err("error".to_string())
        }
    }
}
