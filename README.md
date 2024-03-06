# oxiflow
Small load testing tool written in Rust

## Usage
`oxiflow` tester can perform up to 255 concurrent request in batches up to 255 repeats.

Following command will perform 3 concurrent requests (simultaneously) to the defined site and will repeat such requests batch 4 time, with response timeout of 2 seconds.

This will result in `3 x 4 = 12` total requests attempts. If server will fail to respond in 2 seconds – connection will be dropped and attempt will be recorded as an error.

```shell
oxiflow -t 2 -c 3 -r 4 http://SITENAME/
```

With current limitation of 255 concurrent requests can be repeated 255 times, which will result in `255 ^ 2 = 65 024` requests.

There is no delay neither between requests nor between batches by default (and *at all* at the moment) – be aware. Option `-c100`, for example, will instantly perform 100 requests.

## Options
_At any time – refer to the help for currently available options._

-  `-m, --method <METHOD>` – which HTTP method to use when calling URL (GET, POST, etc.). Number of supported methods might change, so the best way is to do `-mHELP` and all the supported methods will be listed
- `-c, --concurrent <CONCURRENT>` - define many request to send in parallel (might be systems dependent)
- `-r, --repeat <REPEAT>` - how many times to repeat defined batch of concurrent requests
- `-t, --timeout <TIMEOUT>` - response timeout in seconds, if server won't respond in required interval – connection will be terminated and requests will be considered as failed
- `-d, --delay <DELAY>` - delay in seconds between repeating requests batches.
Concurrent requests are performed simultaneously, without delay. Consider disabling concurrence with `-c0` if you want to have delay between each request
- `-f, --file <FILEPATH>` - use text file content as a URLs list
- `--per-request` - flag to enable per-URL report in the output



# Roadmap
This is pure pet fun project, but we all need some sort of plan, right?

Planned features:
- [x] Base functionality (concurrent calls, repeats, timeout, per-code report)
- [x] Progress and verbosity
- [x] Delay between repeats (https://github.com/artemoliynyk/oxiflow/pull/3)
- [x] HTTP Methods support: (https://github.com/artemoliynyk/oxiflow/pull/4)
  - [x] GET
  - [x] POST
  - [x] DELETE
  - [x] PUT
  - [x] PATCH
- [ ] Testing scenarios:
  - [x] URLs file (with methods)
  - [ ] Pre-test actions (Auth)
- [ ] Reporting component
  - [ ] Toggleable coloured output
  - [X] Per-URL requests report
  - [ ] Report export
    - [ ] TXT format
    - [ ] CSV format
    - [ ] XML format
  - [ ] Visual reporting (plotting)
- [ ] Additional HTTP fields support:
  - [ ] Support passing headers
  - [ ] Support passing cookies
- [ ] Authentication support:
  - [ ] Obtain and reuse Bearer Token
  - [ ] Keep cookies between requests during the session
