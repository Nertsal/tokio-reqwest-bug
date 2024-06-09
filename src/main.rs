use geng::prelude::Geng;
use reqwest::Client;

#[tokio::main]
async fn main() {
    Geng::run("test", move |geng| async move {
        geng_main(geng).await;
    });
}

async fn geng_main(_geng: Geng) {
    let client = Client::new();
    for i in 0..1000 {
        println!("loop {i}");
        let req = client.get("http://localhost:3000");
        let response = req.send().await;
        println!("response {:?}", response);
    }
}
