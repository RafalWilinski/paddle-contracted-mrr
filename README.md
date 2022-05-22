Calculate your expected MRR from existing Paddle.com subscriptions.

### Usage

Get contracted recurring revenue in June 2022:

```sh
VENDOR_ID=xxxx VENDOR_AUTH_CODE=yyyy DATE=2022-06 cargo run
```

Get contracted recurring revenue in the next 12 months - skip `DATE` environment variable:

```sh
VENDOR_ID=xxxx VENDOR_AUTH_CODE=yyyy cargo run
```
