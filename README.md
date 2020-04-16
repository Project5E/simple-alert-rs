# simple-alert-rs

A bridge proxy from grafana alert webhook to other webhooks written in Rust.

Features:
 * simple and fast

```
                                    HTTPS  +-----------------+
                                   +------>+Enterprise WeChat|
                                   |       |   Bot WebHook   |
                  +--------------+ |       +-----------------+
                  |              +-+
+-----------+     |              |   HTTP  +-----------------+
|           |     |              +-------->+ Other WebHook 1 |
|  Grafana  +---->+ simple-alert |         +-----------------+
|           |     |              +---+
+-----------+     |              |   |HTTP +-----------------+
                  |              +-+ +---->+ Other WebHook 2 |
                  +--------------+ |       +-----------------+
                                   |              ...
                                   | HTTP  +-----------------+
                                   +------>+ Other WebHook n |
                                           +-----------------+
```
 
TODO:
 * support more webhooks
 * support config file for webhooks

## Usage

### Print usage
```bash
simple-alert-rs --help
```
### Examples
```bash
simple-alert-rs --bind 0.0.0.0:8080 --wx-hook https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=<bot-key>
```

## Install

You may download the binary executable file on
[releases page](https://github.com/Project5E/simple-alert-rs/releases).

Or compile it manually:

```bash
# Install Rust
curl https://sh.rustup.rs -sSf | sh

# Clone source code
git clone https://github.com/Project5E/simple-alert-rs
cd simple-alert-rs

# Build
cargo build --release
target/release/simple-alert-rs --help
```