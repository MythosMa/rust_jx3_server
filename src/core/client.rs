use once_cell::sync::Lazy;
use reqwest::Client;
use std::time::Duration;

pub static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(5))
        .no_proxy()
        .build()
        .expect("http client build failed")
});
