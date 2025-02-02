# Notes

## v0.2.0

### New Features

- allow to specify minimum used TLS version  
  new options: `-1`, `--tlvs1`, `--tlsv1.0`. `--tlsv1.1`, `--tlsv1.2`, `--tlsv1.3`  
  (Note that rustls is used, when `--tlsv1.3` is specified, otherwise native TLS is used.

## v0.1.1

### Bugfixes

- allow HTTP PUT request method

## v0.1.0

Initial version.
