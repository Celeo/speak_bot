# speak_bot

[![CI](https://github.com/Celeo/speak_bot/workflows/CI/badge.svg?branch=master)](https://github.com/celeo/speak_bot/actions?query=workflow%3ACI)
[![Crates.io](https://img.shields.io/crates/v/speak_bot.svg)](https://crates.io/crates/speak_bot)
[![License](https://img.shields.io/crates/l/speak_bot)](https://github.com/Celeo/speak_bot/blob/master/Cargo.toml#L9)

Simple program to run a Discord bot that responds to mentions with a randomly-selected quote from a text file.

## Installing

Download the binary from [GitHub](https://github.com/Celeo/speak_bot) or install via `cargo install speak_bot`.

## Using

You'll need 3 things:

1. [This bot](#installing)
1. A Discord bot token
1. A text file with quotes

To set up a Discord bot, head to [discord.com/developers/applications](https://discord.com/developers/applications), create an application, click on "Bot" on the left nav, and then the "Add Bot" button. Now, there will be a lot more information on the page, and a "Copy" button next to the icon. Click that, and that's your bot's token - you'll need this token when running this program. You'll also need to invite the bot to your server (or a server on which you are an admin) by clicking on the "Oauth" item on the left nav, selecting "bot" in the area with all the checkboxes, and then copying the URL into your browser and confirming the addition.

For the quotes, create a text file with one quote per line.

Once you have these items, run the bot with:

```sh
./speak_bot <path/to/quotes.txt> -t <token>
```

or

```sh
./speak_bot <path/to/quotes.txt>
```

if you have your Discord bot token in an environment variable called "DISCORD_TOKEN". This may be more useful for you, as you can configure your terminal to have that variable instead of having to remember it whenever you start the bot.

To run the bot in the background, you can look into your OS's capabilities to run programs in the background. For Linux systems, [systemd](https://linuxconfig.org/how-to-write-a-simple-systemd-service) is a popular choice.

When the bot is running, it will respond with a random quote from the quotes file whenever mentioned (like with `@`).

## Developing

### Building

#### Requirements

* Git
* A recent version of [Rust](https://www.rust-lang.org/tools/install)

#### Steps

```sh
git clone https://github.com/Celeo/speak_bot
cd speak_bot
cargo build
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Please feel free to contribute. Please open an issue first (or comment on an existing one) so that I know that you want to add/change something.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
