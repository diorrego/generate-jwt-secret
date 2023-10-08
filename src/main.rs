use rand::Rng;
use rocket::tokio::sync::Mutex;
use rocket::State;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() {
    rocket::build()
        .manage(Mutex::new(generate_secret()))
        .mount("/", routes![get_secret])
        .launch()
        .await
        .expect("Rocket failed to launch");
}

fn generate_secret() -> String {
    let secret_length = 32; // 256 bits
    let secret: String = (0..secret_length)
        .map(|_| rand::thread_rng().gen_range(0..=255))
        .map(|n| format!("{:02x}", n))
        .collect();
    secret
}

#[get("/secret")]
async fn get_secret(secret: &State<Mutex<String>>) -> String {
    let secret = secret.lock().await;
    secret.clone()
}
