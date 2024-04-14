# oxiflow
Small yet functional load testing tool written in Rust (oh yeah, "blazingly fast", of course)

## Usage
`oxiflow` tester can perform up to 255 concurrent request, with custom timeout between calls or batches and repeats.

Tester can work both with single URL and file list with URLs and methods.

For now, only following methods are supported:
- `GET`
- `POST`
- `DELETE`
- `PUT`
- `PATCH`

## Single URL vs. File
There are two main exclusive arguments to provide test targets: File or URL.

### URL
If you have only one URL to call - you can provide just a URL and method (optionally) to call and tweak other parameters.

```shell
# to call a singe URL with default method (GET)
./oxiflow https://site.test/critical-endpoint

# to call a singe URL with specific HTTP method
./oxiflow https://site.test/post-endpoint -mPOST
```


### File
But if you have a set of different URL or you want to call the same URL but use few different HTTP methods - then the file is the choice here: `-f` or `--file`.

```test
# this is sample file called url-list.txt
https://site.test/critical-endpoint
GET https://site.test/critical-endpoint
POST https://site.test/critical-endpoint
PUT https://site.test/critical-endpoint
```

Following command will call each URL defined in the file 
```shell
./oxiflow -f url-list.txt
```

Comments in file are supported, using `#` character on the beginning of the line.



## Common arguments
_At any time - refer to the help for currently available options (`-h`)._

- **method** (`-m`) – which HTTP method to use when calling single URL (GET, POST, etc.). Supported methods will change overtime, so the best way is to get then is by adding argument `-mHELP`
- **concurrent** (`-c`) - define many request to send in parallel (might be systems dependent, max. 255)
- **repeat** (`-r`) - how many times to repeat defined batch of concurrent requests (max. 255)
- **timeout** (`-t`) - response timeout in _seconds_, if server won't respond in required interval - connection will be terminated and requests will be considered as failed
- **delay** (`-d`) - delay in seconds between repeating requests batches.
Concurrent requests are performed simultaneously, without delay. Consider disabling concurrence with `-c0` if you want to have delay between each request
- **reporting** (`--per-request`) - will produce per-URL report output
- **verbosity level** (`-v`) - use to print more details during calls. This is accumulator argument, meaning more `v` you add - more verbosity it provides. Where `-v` is some verbosity and `-vvvv` is a maximal (trace output). 


## Concurrency and repeats
> **TL; DR:** concurrency argument with a single URL will multiply the very same URL C-times, while with file it will divide URLs list into C-sized pieces

Concurrency work a little bit different with single URL and file-provided URLs, however the idea is very similar - it just forms the requests batch.

After batches were formed - they will be called concurrently and will repeated it according to the `-r` parameter.

Batching logic (concurrency):
- **With a single URL** - tester _**will form the batch**_ made of single URL and then will repeat this batch N-times
- **With multiple URLs in file** - all the _**URLs will be split**_ into batches and all the batches will be repeated N-times


Basically, this command will create batch of 5 URLs and will call it in parallel twice, performing 10 requests in total (`5 * 2`)
```shell
oxiflow -c 5 -r 2 http://localhost:8080/test-url.html
```


## Sample test flow
Following command will perform 3 concurrent requests (simultaneously) to the defined site and will repeat such requests batch 4 time, with timeout of 2 seconds and will trigger delay of 2 seconds between every batch 

This will result in `3 x 4 = 12` total requests attempts. If server will fail to respond in 2 seconds – connection will be dropped and attempt will be recorded as an error.

```shell
oxiflow -t 2 -c 3 -r 4 -d 2 http://localhost:8080/test-url.html
```

With current limitation of 255 concurrent requests can be repeated 255 times, which will result in `255 ^ 2 = 65 024` requests.

There is no delay neither between requests nor between batches by default (and *at all* at the moment) – be aware. Option `-c100`, for example, will instantly perform 100 requests.


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
  - [ ] Request content/body
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
