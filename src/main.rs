use carapax::prelude::*;
use dotenv::dotenv;
use env_logger;
use futures::Future;
use log;
use std::env;
use std::time::Duration;

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

//fn create_thread(api: Api) {
//    std::thread::spawn(move || {
//        let method = SendMessage::new(-1001369415711, "Hello");
//        let response = api.execute(&method).then(|x| {
//            log::info!("sendMessage result: {:?}\n", x);
//            futures::future::ok::<u32, u32>(7)
//        });
//    });
//}

//
//fn create_future() -> impl Future<Item = (), Error = ()> {
//    log::info!("11");
//    futures::future::ok(())
//}

fn create_thread(api: Api) {
    std::thread::spawn(move || {
        log::info!("Tokio thread starting...");
        tokio::run(
            App::new()
                .add_handler(Handler::message(handle_message))
                .run(api.clone(), UpdateMethod::poll(UpdatesStream::new(api)))
        );
    });
}

fn main() {
    dotenv().expect("Failed to read .env file");;
    env_logger::init();
    log::info!("started");

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let proxy = env::var("CARAPAX_PROXY").ok();

    let api = Api::new(token, proxy).unwrap();
    create_thread(api.clone());

    std::thread::sleep(Duration::from_secs(5));

    let method = SendMessage::new(-1001369415711, "Hello");
    let mut f = api.execute(&method);


    tokio::run(f.then(|_|Ok(())));
    //let r = f.wait();

    //log::info!("sendMessage result: {:?}\n", r);
    std::thread::sleep(Duration::from_secs(5));
}

