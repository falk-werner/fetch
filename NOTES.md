# Notes

## v0.3.0

### New Features

- add verbosity options
  - `-s`: Silent mode (don't print any additional output)
  - `-S`: print error messages, even if `-s` is set
  - `-v`: print additional messages (not used yet)

### Fixes

- use `-k` for insecure operation to match curl's CLI API  
  (`-K` was used before)

## v0.2.0

### New Features

- allow to specify minimum used TLS version  
  new options: `-1`, `--tlvs1`, `--tlsv1.0`. `--tlsv1.1`, `--tlsv1.2`, `--tlsv1.3`  
  Note that rustls is used, when `--tlsv1.3` is specified, otherwise native TLS is used.
- allow HTTPS-only mode  
  new option: `--proto`  
  Note that `--proto` uses the same syntax as the respective `curl` option, but is used
  to determine if HTTPS-only mode can be used. It does not forbit HTTPS requests, when
  on HTTP is active.

## v0.1.1

### Bugfixes

- allow HTTP PUT request method

## v0.1.0

Initial version.
