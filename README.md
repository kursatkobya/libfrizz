
## Running frizz cli

* Examples to specify custom request method to given target
```
# frizz --target http://httpbin.org/post\?test\=2  -X POST -d "hallo"
# frizz --target http://httpbin.org/put\?test\=2 -X PUT
# frizz -t http://httpbin.org/get?test=2 -X GET -#
# frizz.exe -t http://httpbin.org/trace?test=2 -X TRACE -#
# frizz -t http://httpbin.org/head?test=2 -X HEAD -#
# frizz -t http://httpbin.org/put?test=2 -X PUT -#

```

* Examples to scan ports for given target
```
# frizz -s --target example.net
# frizz -s -t example.net --ports 80 1024
# frizz -s -t example.net --timeout 1 -c 1024
# frizz -s -t example.net --timeout 1 -c 1024 -o output.txt
# frizz -s -t example.net --timeout 1 --sctp -o output
# frizz -s -t example.net --timeout 1 --tcp -o output
```

Check [Git workflow](https://github.com/kursatkobya/libfrizz/wiki/Git-workflow) for how to contribute.
