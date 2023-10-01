use rand::Rng;

fn main() {
    let secret_length = 32; // 256 bits
    let secret: String = (0..secret_length)
        .map(|_| rand::thread_rng().gen_range(0..=255)) // Generate values between 0 and 255
        .map(|n| format!("{:02x}", n)) // Format of two hexadecimal characters
        .collect();

    println!("Generated JWT secret: {}", secret);
}