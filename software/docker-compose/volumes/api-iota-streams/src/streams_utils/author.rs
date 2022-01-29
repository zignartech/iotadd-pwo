use crate::streams_utils::random_seed::randomSeed;
use futures::executor::block_on;
use iota_streams::app::transport::tangle::client::iota_client::Client as OtherClient;
use iota_streams::app::transport::tangle::client::{Client, SendOptions};
use iota_streams::app_channels::api::tangle::{Address, Author, UnwrappedMessage};
use iota_streams::app_channels::api::ChannelType;
use iota_streams::ddml::types::Bytes;

pub struct Publisher {
    author: Author<Client>,
}

impl Publisher {
    pub async fn new(seed_option: Option<String>, channel_type: ChannelType) -> Self {
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

        let author = Author::new(&seed, channel_type, client);

        Self { author }
    }

    pub async fn send_announce(&mut self) -> (String, String) {
        let channel_address = self.author.channel_address().unwrap().to_string();

        let announcement_message = self.author.send_announce().await.unwrap();

        let announcement_id = announcement_message.msgid.to_string();

        (channel_address.clone(), announcement_id.clone())
    }

    pub async fn export(&mut self, password: String) -> Vec<u8> {
        self.author.export(&password).await.unwrap()
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

        let author = Author::import(bytes, pwd, client).await.unwrap();
        Self { author }
    }

    pub async fn fetch_all_next_msgs(&mut self) -> Vec<UnwrappedMessage> {
        self.author.fetch_all_next_msgs().await
    }

    pub async fn send_signed_packet(
        &mut self,
        link_to: &Address,
        public_payload: &Bytes,
        masked_payload: &Bytes,
    ) -> (Address, Option<Address>) {
        self.author
            .send_signed_packet(link_to, public_payload, masked_payload)
            .await
            .unwrap()
    }
}
