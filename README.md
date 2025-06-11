# Coco-Bot

Rust port of [KittyBot](https://github.com/olliequ/KittyBot) for the CS@unimelb Discord server.

## Getting started

Some common steps before you start the bot.

1. Install [Rust](https://www.rust-lang.org/learn/get-started) or [Docker Engine](https://docs.docker.com/engine/install/) depending on what method you prefer.
2. Make a copy of [`sample.env`](./sample.env) & fill in the variables. Rename it to `.env`
3. Follow any one method.

### Docker method (Recommended)

1. Download [docker compose file](./docker-compose.yml)
2. Follow the common step no. 2
3. Run in terminal:

```sh
docker compose up -d
```

### Use precompiled binaries

1. Head over to https://github.com/MRDGH2821/Coco-Bot/releases
2. Download the archive as per your os & architecture.
3. Follow the common step no. 2
4. Double click to execute or:

In linux terminal:

```sh
./coco-bot.bin
```

In Windows terminal

```pwsh
./coco-bot.exe
```

### Compile from source

1. Clone this repo:

```sh
git clone https://github.com/MRDGH2821/Coco-Bot/
```

2. Follow the common step no. 2
3. Run in terminal:

```sh
cargo run
```

## Licence

[Apache 2.0](./LICENCE.txt)
