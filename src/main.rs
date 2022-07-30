use std::thread;
use std::env;

use actix_web::{App, HttpResponse, HttpServer, post, Responder, web};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct Config {
    bot_oauth_token: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bot_oauth_token = env::var("SLACK_BOT_TOKEN").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Config {
                bot_oauth_token: bot_oauth_token.clone()
            }))
            .service(webhook)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[derive(Deserialize)]
struct UnknownWebhookEvent {
    r#type: String,
}

#[derive(Deserialize)]
struct Event {
    text: String,
    user: String,
}

#[derive(Deserialize)]
struct EventCallback {
    event: Event,
}

#[post("/webhook")]
async fn webhook(config: web::Data<Config>, req_body: String) -> impl Responder {
    println!("{:?}", req_body);

    let body: UnknownWebhookEvent = serde_json::from_str(req_body.as_str()).unwrap();

    println!("{:?}", body.r#type);

    match serde_json::from_str::<EventCallback>(req_body.as_str()) {
        Ok(e) => {
            thread::spawn(move || {
                println!("thread running");
                handle_event(config.bot_oauth_token.clone(), e);
            });
        }
        Err(_) => {}
    }

    HttpResponse::Ok().body(req_body)
}

#[derive(Debug, Deserialize, Serialize)]
struct ChatMessageRequest {
    text: String,
    channel: String,
}


fn handle_event(token: String, event: EventCallback) {
    let hellos = vec![
        "hello",
        "hi",
        "hey",
    ];

    if !hellos.contains(&event.event.text.as_str()) { return; }

    let client = reqwest::blocking::Client::new();

    let request_body = ChatMessageRequest {
        text: "Don't just say \"Hello\": https://www.nohello.com".to_string(),
        channel: event.event.user.clone(),
    };

    let res = client.post("https://slack.com/api/chat.postMessage")
        .header("authorization", format!("Bearer {}", token))
        .json(&request_body)
        .send();

    println!("res {:?}", res.unwrap().text());
}
