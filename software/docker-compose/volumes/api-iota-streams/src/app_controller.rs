use actix_web::web::Json;
use crate::models::dtos::create_author_dto::{CreateAuthorBody, CreateAuthorQuery};
use crate::models::dtos::fetch_all::FetchAll;
use crate::models::dtos::send_one_dto::SendOneQuery;
use actix_web::HttpResponse;
use actix_web::{post, web, web::Query};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use crate::streams_utils::random_seed::randomSeed;
use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use futures::executor::block_on;
use iota_streams::app::transport::tangle::client::iota_client::Client as OtherClient;
use iota_streams::app::transport::tangle::client::{Client, SendOptions};
use iota_streams::app::transport::tangle::{ TangleAddress};
use iota_streams::app_channels::api::tangle::Author;
use iota_streams::app_channels::api::tangle::ChannelType;
use iota_streams::app_channels::api::tangle::MessageContent;
use iota_streams::app_channels::api::tangle::Subscriber;
use iota_streams::ddml::types::Bytes;
use std::str::FromStr;

#[post("/create_author")]
pub async fn createAuthor(
  query: Query<CreateAuthorQuery>,
  body: Json<CreateAuthorBody>,
) -> HttpResponse {
  let rs = query.into_inner().sendingSeed;
  let seed = match rs {
    true => body.into_inner().seed,
    false => randomSeed(81)
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

  let mut author: Author<Client> =

   Author::new(&seed,ChannelType::SingleBranch ,client );
  let j = author.send_announce().await; //let annAddress: TangleAddress

  let annAddress: TangleAddress = match j {
    Ok(v) => v,
    _ => {
      return HttpResponse::Ok()
        .content_type("application/json")
        .json("author error");
    }
  };

  let password = randomSeed(12);
  let exported = author.export(&password.clone()).await.unwrap();
  let encodedExported = encode_config(exported.clone(), URL_SAFE_NO_PAD);

  println!("  msg => <{:x}>", annAddress.to_msg_index());
  let result = json!({
      
      "address": {
          "appInst":annAddress.appinst.to_string(),
          "msgId": annAddress.msgid.to_string(),
      },
      "author":{
        "password": password.clone(),
        "state": encodedExported,
        // "seed": seed.to_string(),
        
      },
      
  });

  HttpResponse::Ok().json(result)
}



#[post("/address/sendOne")]
pub async fn addressSendOne(
  query: Query<SendOneQuery>,
  bytes: web::Bytes,
) -> HttpResponse {
  let q = query.into_inner();
  let address = q.address;
  let autor = q.author;
  

  let s = String::from_utf8(bytes.to_vec()).unwrap();
  let json: HashMap<String, Value> = serde_json::from_str(&s).unwrap();
  // println!("s: {}",s);

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

  let payloadStr = serde_json::to_string(&json).unwrap();
  let payload = encode_config(&payloadStr, URL_SAFE_NO_PAD);

  let get_address = address.appInst.clone()+ &":".to_string() + &address.msgId.clone();
  println!("address: {}",get_address);
  let keyLoadLink =
    TangleAddress::from_str(&get_address).unwrap();

  let mut author: Author<Client> = Author::import(
    &decode_config(autor.state.clone(), URL_SAFE_NO_PAD).unwrap(),
    &autor.password.clone(),
    client.clone(),
  )
  .await
  .unwrap();
  let _msgs = author.fetch_all_next_msgs().await;
  
  print!("  Author1     : {}", author);

      let result = author.send_signed_packet(
        &keyLoadLink,
        &Bytes::default(),
        &Bytes(payload.as_bytes().to_vec()),
    ).await;

        let (msg_link, _): (TangleAddress, _)  = match result {
          Ok(v) => v,
          _ => {
            return HttpResponse::Ok()
              .content_type("application/json")
              .json("sendone error");
          }
        };
        print!("  Author3     : {}", author);
    // }

    let exported = author.export(&autor.password.clone()).await.unwrap();
    let encodedExported = encode_config(exported.clone(), URL_SAFE_NO_PAD);


  let j = json!({
      
      "address": {
          "appInst":msg_link.appinst.to_string(),
          "msgId": msg_link.msgid.to_string(),
      },
      "author":{
        "password": autor.password.clone(),
        "state": encodedExported,
        // "seed": autor.seed.to_string(),
        
      },
      
  

  });

  HttpResponse::Ok().json(j)

}

#[post("/address/fetchAll")]
pub async fn addressFetchAll(

  query: Query<FetchAll>,
) -> HttpResponse {
  let q = query.into_inner();
  // let address = q.address;
  let subscriptor = q.subscriber;

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

    let mut subscriber: Subscriber<Client> = Subscriber::import(
      &decode_config(subscriptor.state.clone(), URL_SAFE_NO_PAD).unwrap(),
      &subscriptor.password.clone(),
      client.clone(),
    )
    .await
    .unwrap();

  //   let get_address = address.appInst.clone()+ &":".to_string() + &address.msgId.clone();
  // let _importedLoadLink =
  //   TangleAddress::from_str(&get_address).unwrap();

  println!("Subscriber1: {}", subscriber);

  // subscriber.receive_announcement(&importedLoadLink).await.unwrap();

  if subscriber.is_registered() {
    println!("subscriber Ok");
  }
  println!("Subscriber2: {}", subscriber);

  let msgs = subscriber.fetch_all_next_msgs().await;
  // let msgs = subscriber.receive_msg_by_sequence_number(&importedLoadLink,3).await;
  // let msgs = subscriber.fetch_prev_msgs(&importedLoadLink,100).await.unwrap();

  println!("Subscriber3: {}", subscriber);

  let processed_msgs = msgs
  .iter()
  .map(|msg| {
      let content = &msg.body;
      match content {
          MessageContent::SignedPacket {
              pk: _,
              public_payload: _,
              masked_payload,
          } => String::from_utf8(decode_config(&String::from_utf8(masked_payload.0.to_vec()).unwrap(),URL_SAFE_NO_PAD).unwrap(),).unwrap(),
          _ => String::default(),
      }
  })
  .filter(|s| s != &String::default())
  .collect::<Vec<String>>();


let mut my_vec:Vec<Value> = Vec::new();

  print!("Retrieved messages: ");
  for i in 0..processed_msgs.len() {
      print!("{}, ", processed_msgs[i]);
      let jzx: Value = serde_json::from_str(& processed_msgs[i]).unwrap();
      my_vec.push(jzx);
  }
  println!();

  let exported = subscriber.export(&subscriptor.password.clone()).await.unwrap();
  let encodedExported = encode_config(exported.clone(), URL_SAFE_NO_PAD);
println!("password: {}", subscriptor.password.clone());
println!("state: {}", encodedExported);



  HttpResponse::Ok().json(my_vec)

}
