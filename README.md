# Shitverter Bot

## Introduction
This bot waits and processes `.webm` files in chats and converts them to `.mp4`.

## Requirements
- Rust
- FFmpeg
- Docker (optional)
- [Just](https://github.com/casey/just) (command runner)

## Installation and Running

### Using Just (recommended)
```bash
# Show available commands
just

# Rebuild the project from scratch and run in docker
just rebuild

# Just run the container (without rebuilding)
just run
```

### Local Setup (alternative)
```bash
cargo build --release
./target/release/converter-bot
```

### Using Docker directly (alternative)
```bash
docker build -t shitverter .
docker run -d -e TELOXIDE_TOKEN=$TELEGRAM_API_TOKEN --name my_shitverter_container shitverter:latest
```

## Configuration
Set the following environment variables:
- `TELOXIDE_TOKEN`: Your Telegram Bot Token.

## Serverless mode via GitHub Actions

The repository includes a scheduled workflow that runs every 10 minutes to poll Telegram for new messages and convert `.webm` uploads without keeping a long-running bot online.

- Add a repository secret `TELOXIDE_TOKEN` with your bot token.
- The workflow persists the latest processed Telegram update ID in the repository variable `TELEGRAM_UPDATE_OFFSET`. The variable is created automatically after the first successful run; you do not need to seed it manually.
- You can trigger the workflow manually (`workflow_dispatch`) or rely on the default cron schedule.
- To run the same serverless pass locally, set `SERVERLESS_RUN=true` (and optionally `TELEGRAM_UPDATE_OFFSET=<last_id>`) before executing `cargo run --release`.

## Usage
Send a `.webm` file to the chat with bot, and it will send converted `.mp4` file and delete post with webm.
Also shows tg ID's of new members.

## Contributing
Contributions are welcome. Please send pull requests.
