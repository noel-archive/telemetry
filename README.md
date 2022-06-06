# 🐻‍❄️🌧️ Noelware Telemetry
> *Telemetry project for Noelware, to capture anonymous data about the running products.*

## Why? Should I even... trust you?
Because, even though that "telemetry" has a bad reputation on how it is used, this is not the purpose of this project.

> Note:
> **We will not collect IP addresses, host names, user names, container labels, etc**!

Our plan is to create a privacy-first company that will not sell anything to outside consumers, even your data will never be collected to outside consumers, since we got tired of corporations being who they truely are! I know we aren't the first "start-up" company that is trying to achive this goal, but I think we can.

This project is made to capture anonymous data about our products that you might self-host on any cloud provider or managed infrastructure! By default, all products disable telemetry input, but we recommend enabling it, if you wish!

It is useful to the Noelware team so we can collect:

- Metrics about how are products are used!
- How many errors spike up per installation?

On the trust factor, that's up to you. Do you really want a cute polar bear handling all of your data? To be honest... I think you should. He's pretty cute, you know.

## Installation
I don't think you would want to run Noelware's Telemetry server since it's catered to Noelware's specific tech stacks and such. But,
if you want to:

You are required to have **Rust v1.60** and **ClickHouse 22.1.5**, optionally **Logstash** running (if `config.logging.logstash_uri` is provided):

```shell
$ git clone git@github.com:Noelware/telemetry.git && cd telemetry
$ cargo build --release
$ psql -U <username> -f ./init.sql
$ ./target/release/telemetry-server
```

## Contributing
Thanks for considering contributing to **Noelware Telemetry**! Before you boop your heart out on your keyboard ✧ ─=≡Σ((( つ•̀ω•́)つ, we recommend you to do the following:

- Read the [Code of Conduct](./.github/CODE_OF_CONDUCT.md)
- Read the [Contributing Guide](./.github/CONTRIBUTING.md)

If you read both if you're a new time contributor, now you can do the following:

- [Fork me! ＊*♡( ⁎ᵕᴗᵕ⁎ ）](https://github.com/Noelware/telemetry/fork)
- Clone your fork on your machine: `git clone https://github.com/your-username/telemetry`
- Create a new branch: `git checkout -b some-branch-name`
- BOOP THAT KEYBOARD!!!! ♡┉ˏ͛ (❛ 〰 ❛)ˊˎ┉♡
- Commit your changes onto your branch: `git commit -am "add features （｡>‿‿<｡ ）"`
- Push it to the fork you created: `git push -u origin some-branch-name`
- Submit a Pull Request and then cry! ｡･ﾟﾟ･(థ Д థ。)･ﾟﾟ･｡

## License
**Noelware Telemetry** is released under the **Apache 2.0** License with love ( ^з^) y -☆  by Noelware.
