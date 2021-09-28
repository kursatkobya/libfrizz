#!/usr/bin/env bash
FRIZZEXEC=./frizz.exe
unameOut="$(uname -s)"
case "${unameOut}" in
    Linux*)     FRIZZEXEC=./frizz;;
    Darwin*)    FRIZZEXEC=./frizz;;
    *)          machine="UNKNOWN:${unameOut}"
esac
# install caddy
curl -sS https://webinstall.dev/caddy | bash

caddy run &

cd ../
cargo --version
cargo fmt -- --check
cargo clippy -- -Dwarnings
cargo test
cargo run
cd target/debug/
set -x

# running post data from CLI
$FRIZZEXEC -t http://httpbin.org/post?test=2 -X POST -d "hallo" -f
$FRIZZEXEC -t http://httpbin.org/posta?test=2 -X POST -d "hallo" -f
$FRIZZEXEC -t http://httpbin.org/post?test=2 -X POST -d "hallo"
# testing different http methods
$FRIZZEXEC -t http://httpbin.org/get?test=2 -X GET
$FRIZZEXEC -t http://httpbin.org/post?test=2 -X POST
$FRIZZEXEC -t http://httpbin.org/trace?test=2 -X TRACE
$FRIZZEXEC -v -t http://httpbin.org/head?test=2 -X HEAD
$FRIZZEXEC -t http://httpbin.org/put?test=2 -X PUT
#writing to file
$FRIZZEXEC -v -t http://httpbin.org/get?test=2 -o test.txt

# caddy local tests
$FRIZZEXEC -t https://localhost/static_response -X GET
$FRIZZEXEC -v -t https://localhost/static_response -o test.txt
$FRIZZEXEC -k -t https://localhost/static_response

# TODO after https://github.com/kursatkobya/libfrizz/issues/21 basic authentication imlpementation we can enable tests below
#curl -uBob:hiccup -kv https://localhost/secret/test
#$FRIZZEXEC -uBob:hiccup -k -t https://localhost/secret/test

# kill caddy
kill %-1
exit 0