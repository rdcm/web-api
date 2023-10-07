use actix_web::rt;
use web_api::root::composition_root::create_server;

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

    pub async fn get_user(&self, id: String) -> String {
        let uri = format!("{}/user/{}", self.base_url, id);
        let response = reqwest::get(uri).await;

        match response {
            Ok(r) => r.status().to_string(),
            Err(e) => e.to_string(),
        }
    }
}
