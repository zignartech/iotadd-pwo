use crate::streams_utils::random_seed::randomSeed;
use futures::executor::block_on;
use iota_streams::app::transport::tangle::client::iota_client::Client as OtherClient;
use iota_streams::app::transport::tangle::client::{Client, SendOptions};
use iota_streams::app_channels::api::tangle::{Address, Subscriber, UnwrappedMessage};
// use iota_streams::app_channels::api::ChannelType;
// use iota_streams::ddml::types::Bytes;

pub struct Subscriptor {
    subscriber: Subscriber<Client>,
}

impl Subscriptor {
    pub async fn new(seed_option: Option<String>) -> Self {
        let seed = match seed_option {
            Some(seed) => seed,
            None => randomSeed(81),
        };

        let send_options: SendOptions = SendOptions {
            url: std::env::var("NODE").unwrap(),
            local_pow: false,
        };

        let iota_client = block_on(
            OtherClient::builder()
                .with_node(&std::env::var("NODE").unwrap())
                .unwrap()
                .with_local_pow(false)
                .finish(),
        )
        .unwrap();

        let client = Client::new(send_options, iota_client);

        let subscriber = Subscriber::new(&seed, client);

        Self { subscriber }
    }

    pub async fn receive_announcement(&mut self, link: &Address) -> () {
        self.subscriber.receive_announcement(link).await.unwrap()
    }

    pub async fn export(&mut self, password: String) -> Vec<u8> {
        self.subscriber.export(&password).await.unwrap()
    }

    pub async fn import(bytes: &[u8], pwd: &str) -> Self {
        let send_options: SendOptions = SendOptions {
            url: std::env::var("NODE").unwrap(),
            local_pow: false,
        };

        let iota_client = block_on(
            OtherClient::builder()
                .with_node(&std::env::var("NODE").unwrap())
                .unwrap()
                .with_local_pow(false)
                .finish(),
        )
        .unwrap();

        let client = Client::new(send_options, iota_client);

        let subscriber = Subscriber::import(bytes, pwd, client).await.unwrap();
        Self { subscriber }
    }

    pub async fn fetch_all_next_msgs(&mut self) -> Vec<UnwrappedMessage> {
        self.subscriber.fetch_all_next_msgs().await
    }

    pub async fn fetch_next_msgs(&mut self) -> Vec<UnwrappedMessage> {
        self.subscriber.fetch_next_msgs().await
    }

}
