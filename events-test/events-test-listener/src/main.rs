use std::sync::{Arc, Mutex};

use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::{Client, Cluster, EventContext};
use flexi_logger::{AdaptiveFormat, Logger};
use log::info;

use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    Logger::try_with_str("debug")
        .unwrap()
        .adaptive_format_for_stderr(AdaptiveFormat::WithThread)
        .start()
        .unwrap();

    let url = Cluster::Custom(
        "http://localhost:8899".to_string(),
        "ws://127.0.0.1:8900".to_string(),
    );

    let payer =
        read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json")).expect("Need a payer");

    let anchor_client = Client::new_with_options(url, payer, CommitmentConfig::processed());

    let event_program_id = "Fvk1gv6DmGbnwwafWPj1LqSRiCoVhqLa2TzRrProugWP";
    let event_buf = bs58::decode(event_program_id.clone()).into_vec().unwrap();
    let event_pubkey = Pubkey::new(&event_buf[..]);
    let event_program: Arc<Mutex<anchor_client::Program>> =
        Arc::new(Mutex::new(anchor_client.program(event_pubkey.clone())));

    let handle = event_program.lock().unwrap().on(
        move |_ctx: &EventContext, order: events_test::StoplossOrderUpdate| {
            info!("received order order update: {:?}", order);
            handle(order);
        },
    );
    // TODO: remove once https://github.com/solana-labs/solana/issues/16102
    //       is addressed. Until then, drop the subscription handle in another
    //       thread so that we deadlock in the other thread as to not block
    //       self thread.
    // std::thread::spawn(move || {
    //     drop(handle);
    // });

    // stayin alive
    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        info!(".");
    }
}

fn handle(order: events_test::StoplossOrderUpdate) {
    info!("do something {:?}", order);
}
