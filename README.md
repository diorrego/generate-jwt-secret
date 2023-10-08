# JSON Web Token (JWT) Secret Key Generator with Rocket

![Rust](https://img.shields.io/badge/Rust-1.55.0-orange)
![Rocket](https://img.shields.io/badge/Rocket-0.5.0-red)

## Overview

This project is a simple Rust program transformed into a Rocket web server. It generates a secure secret key suitable for use in various applications, including the implementation of JSON Web Tokens (JWT) and any other scenario where a random and secure secret key is required.

## Usage

To use this program, follow these steps:

1. Clone this repository to your local machine:

   ```shell
   git clone https://github.com/diorrego/generate-jwt-secret
   ```

2. Navigate to the project directory:

   ```shell
   cd generate-jwt-secret
   ```

3. Build and run the Rust program:

   ```shell
   cargo run
   ```

   The program will start a web server at http://localhost:8000 that provides a secret key when you access http://localhost:8000/secret via a web browser or API request.

## Accessing the Secret Key

```
http://localhost:8000/secret
```

The server will respond with the secret key in plain text format.

## Example Output

After accessing the /secret endpoint, you will receive a response similar to the following:

```
c79b7d7d23082c9d64a98d27473b20be364400fded51ddb1a60631ab76e41164
```

## Author

- Diego Orrego
- GitHub: [diorrego](https://github.com/diorrego)
