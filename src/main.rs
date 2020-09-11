#[allow(non_snake_case)]

use rand::seq::SliceRandom;
use tokio::time::delay_for;
use discord_rpc_client::Client;

#[tokio::main]
async fn main() {
    let rpc = Client::new(753767549237461102);
    if rpc.is_err() { panic!("Failed to RPC client!") }
    let mut rpc = rpc.unwrap();

    rpc.start();

    rpc.set_activity(|act| {
        act.state("my own rpc client tm")

        .assets(|assets|
            assets.large_image("chi").large_text("https://github.com/BlueSlimee")
                .small_image("clion").small_text("playing jetbrains clion"))
    });

    tokio::spawn(async move {
        loop {
            set_status(&mut rpc);

            delay_for(tokio::time::Duration::from_secs(30)).await;
        };
    }).await;
}

fn set_status (rpc: &mut Client) {
    let messages = vec![
        "its going down bitch",
        "have you watched jenna marbles today?",
        "stan azealia banks",
        "I'm a better president than Trump.",
        "Juliana is a state of mind",
        "look at your window...",
        "they're coming",
        "they're after you",
        "perereca até o chão",
        "Grey's Anatomy <3",
        "watching station 19",
        "using jetbrains products",
        "i like rust",
        "i like js",
        "rainbows!!",
        "powered by tokio"
    ];

    rpc.set_activity(|act| {
        act.state(*messages.choose(&mut rand::thread_rng()).unwrap()).assets(|assets|
        assets.large_image("chi").large_text("https://github.com/BlueSlimee")
            .small_image("clion").small_text("playing jetbrains clion"))
            .details("uwu")
    });
}