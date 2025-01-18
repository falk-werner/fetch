[![build](https://github.com/falk-werner/fetch/actions/workflows/build.yaml/badge.svg)](https://github.com/falk-werner/fetch/actions/workflows/build.yaml)

# fetch

Downloads an aritfact from a URL and optionally verifies it's SHA256 or MD5 checksum.

## Usage

```bash
fetch -L $SOME_URL --sha256 $SHA256_HASH
```

## Motivation

A typical use case is to download an artifact from a URL and verify it's checksum afterwards.
Unfortunately, there is no commonly used tool which provides this in one step. There are
multiple feature requests on commonly used tools such as [curl](https://curl.se/), which 
were closed due to a lack of interest (see [here](https://github.com/curl/curl/issues/6836)
or [here](https://github.com/curl/curl/issues/1399)).

On the other hand, popular tools such as [rustup](https://rustup.rs/) and
[node.js](https://nodejs.org/en/download) propose dangerous workflows for installations,
where a direct download is piped into a shell:

```bash
url --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

It would be nice to enhance security of those workflows by verifiing a MD5 oder SHA256
checksum during download.

This can be achieved using the `fetch` utility.

## Command Line Options

```bash
fetch [OPTIONS] <URL>
```

Command line options are strongly inspired by [curl](https://curl.se/).
While `fetch` does not use all options that `curl` provides, the options `fetch` provides are
named the same as `curl`'s options. Therefore, `fetch` can be used as drop-in replacement for
`curl` in most use cases.

| Option | Type | Description |
| ------ | ---- | ----------- |
| -o, --output | Path | Write to file instead of stdout |
| -X, --request | HTTP Method | Specify the request method to use |
| -H, --header | string | Pass custom header(s) to server |
| -d, --data | string | Post data |
| -F, --form | string | Specify multipart form data as name=value pair |
| -K, --insecure | flag | Allow insecure server connections |
| -L, --location | flag | Follow redirects |
| --max-redirs | uint | Maximum number of redirects |
| --max-filesize | uint | Maximum file size to download |
| --connection-timeout | uint | Maximum time allowed for connection in seconds |
| -m, --max-time | uint | Maximum time allowed for transfer in seconds |
| --sha256 | hex-string | SHA256 checksum of the artifact to download |
| --md5 | hex-string | MD5 checksum of the artifact to download |
| -h, --help | flag | Print help |
| -V, --version | flag | Print version |

# Run tests

In order to run tests, [bats](https://github.com/bats-core/bats-core) is needed.
Please install `bats` and build `fetch` before running the tests.

```bash
bats test
```

