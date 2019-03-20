use carapax::prelude::*;
use dotenv::dotenv;
use env_logger;
use futures::Future;
use log;
use std::env;

fn handle_message(context: &mut Context, message: &Message) -> HandlerFuture {
    log::info!("got a message: {:?}\n", message);
    if let Some(text) = message.get_text() {
        let chat_id = message.get_chat_id();
        let method = SendMessage::new(chat_id, text.data.clone());
        let api = context.get::<Api>();
        return HandlerFuture::new(api.execute(&method).then(|x| {
            log::info!("sendMessage result: {:?}\n", x);
            Ok(HandlerResult::Continue)
        }));
    }
    HandlerResult::Continue.into()
}

fn main() {
    dotenv().expect("Failed to read .env file");;
    env_logger::init();
    log::info!("started");

    //let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    
    let proxy = env::var("CARAPAX_PROXY").ok();

    let api = Api::new(token, proxy).unwrap();
    tokio::run(
        App::new()
            .add_handler(Handler::message(handle_message))
            .run(api.clone(), UpdateMethod::poll(UpdatesStream::new(api))),
    );

    let method = SendMessage::new(-1001369415711, "Hello");
    api.clone().execute(&method).then(|x| {
        log::info!("sendMessage result: {:?}\n", x);
        Ok()
    });
}

