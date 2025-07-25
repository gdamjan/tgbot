# A telegram bot

This bot is designed to be hosted as a reactive WASM application.
Primarly for [Spinframework](https://spinframework.dev)
Framework/Cloud, but conceptualy it would be able to work on any WASI
platform. It'll get telegram message updates via a
[webhook](https://core.telegram.org/bots/webhooks), since it can't poll
(or establish a web-socket connection).

## Quickstart

```
SPIN_CONFIG_TELEGRAM_TOKEN="123" spin up --build
```


## Deploy to the cloud

```
spin cloud deploy --variable telegram-token=123
```



## References
- https://core.telegram.org/bots/webhooks
- https://developer.fermyon.com/spin/http-trigger
- https://developer.fermyon.com/spin/http-outbound
- https://developer.fermyon.com/spin/kv-store-api-guide
- https://fermyon.github.io/rust-docs/spin/main/spin_core/index.html
