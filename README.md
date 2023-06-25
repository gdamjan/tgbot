# A telegram bot

This bot is designed to be hosted as a reactive WASM application.
Primarly for [Fermyon Spin](https://developer.fermyon.com/spin/index)
Framework/Cloud, but conceptualy it would be able to work on any WAGI
platform. It'll get telegram message updates via a
[webhook](https://core.telegram.org/bots/webhooks), since it can't poll
(or establish a web-socket connection).

## Quickstart

```
fermyon-spin build
SPIN_CONFIG_PASSWORD="123" SPIN_CONFIG_USERNAME="admin" fermyon-spin up
```


## Deploy to the cloud

```
fermyon-spin cloud deploy --variable password=123 --variable username=admin
```



## References
- https://core.telegram.org/bots/webhooks
- https://developer.fermyon.com/spin/http-trigger
- https://developer.fermyon.com/spin/http-outbound
- https://developer.fermyon.com/spin/kv-store-api-guide
