# oxiflow

Small (as for now) load web testing tool written in Rust

## Usage
`oxiflow` tester can perform up to 255 concurrent requst in batches up to 255 repeats.

Following command will perform 3 concurrent requests (simultaneously) to the defined site and will repeat such requests batch 4 time, with response timeout of 2 seconds.

This will result in `3 x 4 = 12` total requests attempts. If server will fail to respond in 2 seconds – connection will be dropped and attempt will be recorded as an error.

```shell
oxiflow -t 2 -c 3 -r 4 http://SITENAME/
```

With current limitation of 255 concurent requests can be repeated 255 times, wich will result in `255 ^ 2 = 65 024` requests.

There is no delay neither between requests nor between batches by default (and *at all* at the moment) – be aware. Option `-c100`, for example, will instantly perform 100 requests.

## Options
_At any time – refer to the help for currently available options._

- `-c, --concurrent <CONCURRENT>` - define many request to send in parallel (might be systems dependent)
- `-r, --repeat <REPEAT>` - how many times to repeat defined batch of concurrent requestss
- `-t, --timeout <TIMEOUT>` - response timeout in seconds, if server won't respond in required interval – connection will be terminated and requests will be considered as failed
