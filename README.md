# Rust replacetxt

[![Rust](https://github.com/diepes/rust-replace-txt-between-markers/actions/workflows/rust_amd_macos.yml/badge.svg)](https://github.com/diepes/rust-replace-txt-between-markers/actions/workflows/rust_amd_macos.yml)

* A search and replace that looks for start and end marker in file and replaces text between the markers.

```shell
./replacetxt -h

program to search for start and end marker and replace the text inbetween with new text

Usage: replacetxt [OPTIONS] --start <START> --end <END> --src <SRC> --dst <DST> --replace <REPLACE>

Options:
  -s, --start <START>      start Marker string
  -e, --end <END>          end Marker string
      --src <SRC>          src filename
      --dst <DST>          dst filename
  -r, --replace <REPLACE>  replacement string to place between markers
  -v, --verbose            
  -h, --help               Print help
  -V, --version            Print version
```
