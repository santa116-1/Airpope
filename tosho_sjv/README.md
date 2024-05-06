# airpope-sjv

![crates.io version](https://img.shields.io/crates/v/airpope-sjv)

A minimal asynchronous client for the SJ API by V.

The following crate is used by the [`airpope`][airpope crates] app.

## Usage

Download the [`airpope`][airpope crates] app, or you can utilize this crate like any other Rust crate:

```rust
use airpope_sjv::{SJClient, SJConfig, SJMode, SJPlatform};

#[tokio::main]
async fn main() {
    let config = SJConfig {
        user_id: 123,
        token: "xyz987abc".to_string(),
        instance: "abcxyz".to_string(),
        platform: SJPlatform::Android,
    };

    let client = SJClient::new(config, SJMode::VM);
    let manga = client.get_manga(vec![777]).await.unwrap();
    println!("{:?}", manga);
}
```

## Authentication

The following sources only have one method of authentication, and that method uses your email and password.

```bash
$ airpope sj auth email password --help
```

Or, if you use the crates:

```rust
use airpope_sjv::{SJClient, SJConfig, SJMode, SJPlatform};

#[tokio::main]
async fn main() {
    let (account, instance_id) = SJClient::login("test@mail.com", "mypassword", SJMode::SJ, SJPlatform::Android).await.unwrap();

    let config = SJConfig::from_login_response(&account, instance_id, SJPlatform::Android);

    // Do stuff
    let client = SJClient::new(config, SJMode::SJ);
}
```

## Disclaimer

This project is designed as an experiment and to create a local copy for personal use. These tools will not circumvent any paywall, and you will need to purchase and own each chapter with your own account to be able to make your own local copy.

We're not responsible if your account got deactivated.

## License

This project is licensed with MIT License ([LICENSE](https://github.com/noaione/airpope-mango/blob/master/LICENSE) or http://opensource.org/licenses/MIT)

[airpope crates]: https://crates.io/crates/airpope