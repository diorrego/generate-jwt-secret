# JSON Web Token (JWT) Secret Key Generator

![Rust](https://img.shields.io/badge/Rust-1.55.0-orange)

## Overview

This project is a simple Rust program designed to generate a secure secret key. The generated secret key is suitable for use in various applications, including the implementation of JSON Web Tokens (JWT) and any other scenario where a random and secure secret key is required.

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

   The program will generate a 256-bit secret key and display it in hexadecimal format.

## Example Output

After running the program, you will see output similar to the following:

```
Generated JWT secret: c79b7d7d23082c9d64a98d27473b20be364400fded51ddb1a60631ab76e41164
```

## Author

- Diego Orrego
- GitHub: [diorrego](https://github.com/diorrego)
