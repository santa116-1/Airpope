# airpope-musq

![crates.io version](https://img.shields.io/crates/v/airpope-musq)

An asynchronous client for the MU! API by SQ.

The following crate is used by the [`airpope`](airpope) app.

## Usage

Download the [`airpope`](airpope) app, or you can utilize this crate like any other Rust crate:

```rust
use airpope_musq::MUClient;
use airpope_musq::constants::get_constants;

#[tokio::main]
async fn main() {
    let client = MUClient::new("1234", get_constants(1));
    let manga = client.get_manga(240).await.unwrap();
    println!("{:?}", manga);
}
```

## Authentication

The following sources do not have any easy authentication method.

The command to authenticate is `airpope mu auth`.

It's recommended that you set up network intercepting first; please read [INTERCEPTING](https://github.com/noaione/airpope-mango/blob/master/INTERCEPTING.md).

Using the CLI, you can do this:

```bash
$ airpope mu auth secret -t android
```

Or, with Apple constants:

```bash
$ airpope mu auth secret -t ios
```

With crates, you can follow the above usages.

### Android

1. Open the source app.
2. Click on the home page or my page.
3. Observe the requests on HTTP Toolkit and find the request to the API that has `secret` as the query parameters.
4. Save that secret elsewhere and authenticate with `airpope`.

### Apple

1. Open the Stream app and click `Sniff Now`.
2. Go to the source app and open the `Home` or `My Page`.
3. Return to the Stream app and click `Sniff History`, then select the most recent item.
4. Find the request that goes to the API of the source app and locate the request that has `secret=xxxxx` in them.
5. Copy the link and save the secret value somewhere so you can authenticate with `airpope`.

## Disclaimer

This project is designed as an experiment and to create a local copy for personal use. These tools will not circumvent any paywall, and you will need to purchase and own each chapter with your own account to be able to make your own local copy.

We're not responsible if your account got deactivated.

## License

This project is licensed with MIT License ([LICENSE](https://github.com/noaione/airpope-mango/blob/master/LICENSE) or http://opensource.org/licenses/MIT)

[airpope]: https://crates.io/crates/airpope