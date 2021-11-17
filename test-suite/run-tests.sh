#!/usr/bin/env bash
FRIZZEXEC=./frizz.exe
unameOut="$(uname -s)"
case "${unameOut}" in
    Linux*)     FRIZZEXEC=./frizz;;
    Darwin*)    FRIZZEXEC=./frizz;;
    *)          machine="UNKNOWN:${unameOut}"
esac
if ! command -v caddy version &> /dev/null
then
    # install caddy
    curl -sS https://webinstall.dev/caddy | bash
fi

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
$FRIZZEXEC -k -t https://localhost/static_response -X GET
$FRIZZEXEC -kv -t https://localhost/static_response -o test.txt
$FRIZZEXEC -k -t https://localhost/static_response
$FRIZZEXEC -k -# -t https://localhost/static_response
$FRIZZEXEC -k --progress-bar -t https://localhost/static_response

# caddy port scan
$FRIZZEXEC -s -t localhost --ports 80 1024
$FRIZZEXEC -s --udp -t example.org -o output
$FRIZZEXEC -s --tcp -t example.org
$FRIZZEXEC -s --sctp -t example.org --ports 80 1024


# TODO after https://github.com/kursatkobya/libfrizz/issues/21 basic authentication imlpementation we can enable tests below
#curl -uBob:hiccup -kv https://localhost/secret/test
#$FRIZZEXEC -uBob:hiccup -k -t https://localhost/secret/test

# kill caddy
kill %-1
exit 0