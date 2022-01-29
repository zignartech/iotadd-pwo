use crate::models::dtos::create_subscribe_dto::CreateSubscriberQuery;
use crate::models::dtos::fetch_all::FetchAll;
use crate::models::dtos::send_one_dto::SendOneQuery;
use crate::streams_utils::author::Publisher;
use crate::streams_utils::random_seed::randomSeed;
use crate::streams_utils::subscriber::Subscriptor;
use actix_web::{get, HttpResponse};
use actix_web::{post, web, web::Query};
use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use iota_streams::app::transport::tangle::TangleAddress;
use iota_streams::app_channels::api::tangle::ChannelType;
use iota_streams::app_channels::api::tangle::MessageContent;
use iota_streams::ddml::types::Bytes;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;

#[get("/create_author")]
pub async fn createAuthor() -> HttpResponse {
    let mut author = Publisher::new(None, ChannelType::SingleBranch).await;

    let (channel_address, msg_id) = author.send_announce().await;

    let password = randomSeed(12);
    let exported = author.export(password.clone()).await;
    let encodedExported = encode_config(exported.clone(), URL_SAFE_NO_PAD);

    HttpResponse::Ok().json(json!({
        "address": {
            "appInst": channel_address,
            "msgId": msg_id,
        },
        "author":{
          "password": password,
          "state": encodedExported,
        },

    }))
}

#[post("/address/sendOne")]
pub async fn addressSendOne(query: Query<SendOneQuery>, bytes: web::Bytes) -> HttpResponse {
    let q = query.into_inner();
    let address = q.address;
    let autor = q.author;

    let s = String::from_utf8(bytes.to_vec()).unwrap();
    let json: HashMap<String, Value> = serde_json::from_str(&s).unwrap();

    let payloadStr = serde_json::to_string(&json).unwrap();
    let payload = encode_config(&payloadStr, URL_SAFE_NO_PAD);

    let get_address = format!("{}:{}", address.appInst.clone(), address.msgId.clone());
    println!("address: {}", get_address);

    let keyLoadLink = TangleAddress::from_str(&get_address).unwrap();

    let mut author = Publisher::import(
        &decode_config(autor.state.clone(), URL_SAFE_NO_PAD).unwrap(),
        &autor.password.clone(),
    )
    .await;

    let _msgs = author.fetch_all_next_msgs().await;

    let (msg_link, _) = author
        .send_signed_packet(
            &keyLoadLink,
            &Bytes::default(),
            &Bytes(payload.as_bytes().to_vec()),
        )
        .await;

    let exported = author.export(autor.password.clone()).await;
    let encodedExported = encode_config(exported.clone(), URL_SAFE_NO_PAD);

    HttpResponse::Ok().json(json!({

        "address": {
            "appInst":msg_link.appinst.to_string(),
            "msgId": msg_link.msgid.to_string(),
        },
        "author":{
          "password": autor.password.clone(),
          "state": encodedExported,
        },

    }))
}

#[post("/create_subscriber")]
pub async fn createSubscriber(query: Query<CreateSubscriberQuery>) -> HttpResponse {
    let rst = query.into_inner();

    let address = rst.address;

    let mut subscriber = Subscriptor::new(None).await;

    let get_address = format!("{}:{}", address.appInst.clone(), address.msgId.clone());

    let announcement_link = TangleAddress::from_str(&get_address).unwrap();
    println!("address : {}", &get_address);

    subscriber.receive_announcement(&announcement_link).await;

    let password = randomSeed(12);
    let exported = subscriber.export(password.clone()).await;
    let encodedExported = encode_config(exported.clone(), URL_SAFE_NO_PAD);

    HttpResponse::Ok().json(json!({

        "subscriber":{
          "password": password.clone(),
          "state": encodedExported,

        },

    }))
}

#[post("/address/fetchAll")]
pub async fn addressFetchAll(query: Query<FetchAll>) -> HttpResponse {
    let q = query.into_inner();
    // let address = q.address;
    let subscriptor = q.subscriber;

    let mut subscriber = Subscriptor::import(
        &decode_config(subscriptor.state.clone(), URL_SAFE_NO_PAD).unwrap(),
        &subscriptor.password.clone(),
    )
    .await;

    let msgs = subscriber.fetch_all_next_msgs().await;

    let processed_msgs = msgs
        .iter()
        .map(|msg| {
            let content = &msg.body;
            match content {
                MessageContent::SignedPacket {
                    pk: _,
                    public_payload: _,
                    masked_payload,
                } => String::from_utf8(
                    decode_config(
                        &String::from_utf8(masked_payload.0.to_vec()).unwrap(),
                        URL_SAFE_NO_PAD,
                    )
                    .unwrap(),
                )
                .unwrap(),
                _ => String::default(),
            }
        })
        .filter(|s| s != &String::default())
        .collect::<Vec<String>>();

    let mut my_vec: Vec<Value> = Vec::new();

    print!("Retrieved messages: ");
    for i in 0..processed_msgs.len() {
        print!("{}, ", processed_msgs[i]);
        let jzx: Value = serde_json::from_str(&processed_msgs[i]).unwrap();
        my_vec.push(jzx);
    }
    println!();

    let exported = subscriber.export(subscriptor.password.clone()).await;
    let encodedExported = encode_config(exported.clone(), URL_SAFE_NO_PAD);
    println!("password: {}", subscriptor.password.clone());
    println!("state: {}", encodedExported);

    HttpResponse::Ok().json(my_vec)
}
