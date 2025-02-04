[![build](https://github.com/falk-werner/fetch/actions/workflows/build.yaml/badge.svg)](https://github.com/falk-werner/fetch/actions/workflows/build.yaml)

[![Snap Store](https://raw.githubusercontent.com/snapcore/snap-store-badges/master/EN/[EN]-snap-store-white.png)](https://snapcraft.io/fetch)

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
| --data-raw | string | Post data, '@' allowed |
| -F, --form | string | Specify multipart form data as name=value pair |
| -k, --insecure | flag | Allow insecure server connections |
| -L, --location | flag | Follow redirects |
| --max-redirs | uint | Maximum number of redirects |
| --max-filesize | uint | Maximum file size to download |
| --connection-timeout | uint | Maximum time allowed for connection in seconds |
| -m, --max-time | uint | Maximum time allowed for transfer in seconds |
| -1, --tlsv1, --tlsv1.0 | flag | Use TLSv1.0 or later |
| --tlsv1.1 | flag | Use TLSv1.1 or later |
| --tlsv1.2 | flag | Use TLSv1.2 or later |
| --tlsv1.3 | flag | Use TLSv1.3 or later |
| --proto   | string | List of enabled protocols (see below) |
| -s, --silent | flag | Silent mode |
| -S, --show-error | flag | show error messages, even in silent mode |
| -v, --verbose | flag | show additional log messages |
| -i, --include | flag | include HTTP reponse headers in the output |
| --sha256 | hex-string | SHA256 checksum of the artifact to download |
| --md5 | hex-string | MD5 checksum of the artifact to download |
| -h, --help | flag | Print help |
| -V, --version | flag | Print version |

## Protocols

The argument of the `--proto` option is a single string that contains
an expression that is evaluated from the left to the right. It contains
a list of protocols with an optional modifier. The following modifiers
are defined:

- `+`: adds a protocol; default if no modifier is specified explicitly
- `-`: removed a protocol
- `=`: sets the specified protocol only

Known protocols:

- `all`: placeholder for all known protocols
- `http`: HTTP protocol
- `https`: HTTPS protocol

Examples:

- `=https`: allow HTTPS only
- `-all,https`: allow HTTPS only
- `-http`: don't allow HTTP

Note that `fetch` uses this argument only to check, if HTTP-only mode
can be activated, `fetch` does never disable HTTPS. The `--proto`
option was added to maintain compatibility with `curl`.

## Missing Features

Fetch does not aim at full curl compatibility, since fetch focuses on
http / https protocol only. We also do not aim to support each http / https
related option, since some options are rarely used. Before reaching v1.0.0
the following feates should be supported.

- specify root certificte for peer verification  
  curl options: `--cacert`, `--capath`, `--crlfile`
- basic proxy support  
  curl options: `-x`, `--proxy`, `-U`, `--proxy-user`

There are also some useful features which may be supported after v1.0.0:

- mTLS support  
  curl options: `-E`, `--cert`, `--cert-status`, `--cert-type`
- .netrc support  
  curl options: `-n`, `--netrc`, `--netrc-file`
- show document information  
  curl options: `-I`, `--head`
- dump response headers info file  
  curl options: `-D`, `--dump-reader`
- allow to output body on http errors  
  curl option: `--fail-with-body`
- etag support  
  curl options: `--etag-compare`, `--etag-save`
- put post data in url for GET request  
  curl options: `-G`, `--get`
- convenience helpers for often used headers  
  curl options: `-u`, `--user`, `-A`, `--user-agent`, `-r`, `--range`, `-e`, `--referer`, `-b`, `--cookie`, `-c`, `--cookie-jar`
- redirect `stderr`  
  curl option: `--stderr`

## Run tests

In order to run tests, [bats](https://github.com/bats-core/bats-core) is needed.
Please install `bats` and build `fetch` before running the tests.

```bash
bats test
```

