use rand::Rng;
use rocket::tokio::sync::Mutex;
use rocket::State;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() {
    // Create a Rocket web server.
    rocket::build()
        // Manage a shared Mutex that stores the generated secret.
        .manage(Mutex::new(generate_secret()))
        // Mount the `/` route with the `get_secret` handler.
        .mount("/", routes![get_secret])
        // Launch the Rocket web server.
        .launch()
        .await
        .expect("Rocket failed to launch");
}

// Generate a random secret key and return it as a hexadecimal string.
fn generate_secret() -> String {
    let secret_length = 32; // 256 bits
    let secret: String = (0..secret_length)
        // Generate random values between 0 and 255.
        .map(|_| rand::thread_rng().gen_range(0..=255))
        // Format each value as a two-digit hexadecimal string.
        .map(|n| format!("{:02x}", n))
        // Collect the formatted values into a single string.
        .collect();
    secret
}

// Define a Rocket handler that responds to a GET request with the current secret.
#[get("/secret")]
async fn get_secret(secret: &State<Mutex<String>>) -> String {
    // Lock the Mutex to access the shared secret.
    let secret = secret.lock().await;
    // Clone and return the secret as a response.
    secret.clone()
}
