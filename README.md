<h1 align="center">
	RedisFS
</h1>

<h3 align="center">
	Redis File Server. A CLI tool to use any Redis instance to store and transfer files.
</h3>

<p align="center">
	<img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square"/>
	<img src="https://img.shields.io/github/license/BuyMyMojo/RedisFS?color=green"/>
	<img src="https://img.shields.io/github/last-commit/BuyMyMojo/RedisFS?color=green"/>
	<img alt="Crates.io" src="https://img.shields.io/crates/v/redisfs">
</p>

<h4 align="center">
	Status: 🚧 In Construction
</h4>

<p align="center">
	<a href="#about">About</a> •
	<a href="#tech-stack">Tech Stack</a> •
	<a href="#usage">Usage</a> • 
	<a href="#contact">Contact</a> 
</p>

## About
RedisFS allows you to easily store and download files inside of a Redis instance for lost latency access.

## Tech Stack
<img src="https://img.shields.io/badge/Git-05122A?style=flat&logo=git" alt="git Badge" height="25">
<img src="https://img.shields.io/badge/Rust-05122A?style=flat&logo=rust" alt="Rust Badge" height="25">
<img src="https://img.shields.io/badge/Redis-05122A?style=flat&logo=redis" alt="Redis Badge" height="25">
&nbsp;

## Installation
To use this project, follow the steps above:
```bash
# Using crates.io
cargo install redisfs

# Build using cargo
git clone https://github.com/BuyMyMojo/RedisFS.git
cd RedisFS
cargo build --release
```

## Usage
To use this project, follow the steps above:
```bash
# Upload files
redisfs push <FILE_PATH> <FILE_KEY>

# Download file
redisfs clone <OUT_PATH> <FILE_KEY>

# Check file memory usage
redisfs usage <FILE_KEY>

# List all files
redisfs list

# Delte key
redisfs delete <FILE_KEY>

# Use a redis server with a password but no username
redisfs -r "redis://:[password]@[address]:[password]" list
# The : is required

# Use a redis server with a password and username
redisfs -r "redis://[username]:[password]@[address]:[password]" list
```

## Contact
<img align="left" src="https://avatars.githubusercontent.com/BuyMyMojo?size=100">

Made with ❤️ by [Owen Quinlan](https://github.com/BuyMyMojo), get in touch!

<a href="mailto:hello@buymymojo.net" target="_blank"><img src="https://img.shields.io/badge/Email-D14836?style=flat&logo=gmail&logoColor=white" alt="Email Badge" height="25"></a>
<img src="https://img.shields.io/badge/Discord:BuyMyMojo%230308-5865F2?style=flat&logo=discord&logoColor=white" alt="Discord Badge" height="25">&nbsp;

<br clear="left"/>
