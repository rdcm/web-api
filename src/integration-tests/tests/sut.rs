use actix_web::rt;
use api::models::{CreateUserRequest, CreatedUserIdResponse, TrackActivityRequest, UserResponse};
use domain::events::ActivityEvent;
use host::composition::Composition;
use host::conf::AppConf;
use messaging::fake_kafka::FakeKafkaFactory;
use messaging::kafka::IKafkaFactory;
use reqwest::StatusCode;
use std::sync::Arc;

pub struct Sut {
    base_url: String,
    kafka_factory: Arc<dyn IKafkaFactory<ActivityEvent>>,
}

impl Sut {
    pub async fn new() -> Self {
        let conf = AppConf {
            api_host: "127.0.0.1".to_string(),
            api_port: 0,
            db_uri: "mongodb://127.0.0.1/test".to_string(),
            db_name: "test".to_string(),
        };

        let kafka_factory: Arc<dyn IKafkaFactory<ActivityEvent>> =
            Arc::new(FakeKafkaFactory::new());
        let info = Composition::new(&conf, kafka_factory.clone())
            .await
            .unwrap();

        let base_url = format!("http://{}:{}", info.addrs[0].ip(), info.addrs[0].port());

        rt::spawn(info.server);

        Self {
            base_url,
            kafka_factory: kafka_factory.clone(),
        }
    }

    pub async fn get_user(&self, id: String) -> Result<UserResponse, String> {
        let client = reqwest::Client::new();
        let uri = format!("{}/user/{}", self.base_url, id);

        let response = client.get(uri).send().await.unwrap();

        match response.status() {
            StatusCode::OK => Ok(response.json().await.unwrap()),
            StatusCode::BAD_REQUEST => Err(response.text().await.unwrap()),
            code => Err(format!("unexpected status code {}", code)),
        }
    }

    pub async fn create_user(
        &self,
        name: String,
        age: u8,
    ) -> Result<CreatedUserIdResponse, String> {
        let client = reqwest::Client::new();
        let uri = format!("{}/user", self.base_url);

        let response = client
            .post(uri)
            .json(&CreateUserRequest { name, age })
            .send()
            .await
            .unwrap();

        match response.status() {
            StatusCode::CREATED => Ok(response.json().await.unwrap()),
            code => Err(format!("unexpected status code {}", code)),
        }
    }

    pub async fn track_activity(&self, activity: &TrackActivityRequest) -> Result<(), String> {
        let client = reqwest::Client::new();
        let uri = format!("{}/track_activity", self.base_url);

        let response = client.post(uri).json(&activity).send().await.unwrap();

        match response.status() {
            StatusCode::OK => Ok(()),
            code => Err(format!("unexpected status code {}", code)),
        }
    }

    pub fn get_activity_event(&self) -> ActivityEvent {
        let consumer = self.kafka_factory.create_consumer();
        consumer.poll()
    }
}
