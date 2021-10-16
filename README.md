
## Running frizz cli

* Specify custom request method to given target
```
# frizz --target http://httpbin.org/post\?test\=2  -X POST -d "hallo"
200 OK{
  "args": {
    "test": "2"
  },
  "data": "hallo",
  "files": {},
  "form": {},
  "headers": {
    "Accept": "*/*",
    "Content-Length": "5",
    "Host": "httpbin.org",
    "User-Agent": "frizz / 0.1.0",
    "X-Amzn-Trace-Id": "Root=1-616b2fb2-73df152535de8142760d6042"
  },
  "json": null,
  "origin": "158.174.82.172",
  "url": "http://httpbin.org/post?test=2"
}

# frizz --target http://httpbin.org/put\?test\=2 -X PUT
200 OK{
  "args": {
    "test": "2"
  },
  "data": "",
  "files": {},
  "form": {},
  "headers": {
    "Accept": "*/*",
    "Host": "httpbin.org",
    "User-Agent": "frizz / 0.1.0",
    "X-Amzn-Trace-Id": "Root=1-616b305f-00c592481ea6e8121c243e9e"
  },
  "json": null,
  "origin": "158.174.82.172",
  "url": "http://httpbin.org/put?test=2"
}

# frizz -t http://httpbin.org/get?test=2 -X GET
200 OK{
  "args": {
    "test": "2"
  },
  "headers": {
    "Accept": "*/*",
    "Host": "httpbin.org",
    "User-Agent": "frizz / 0.1.0",
    "X-Amzn-Trace-Id": "Root=1-6140c58c-0f9bafac391aadaa08926665"
  },
  "origin": "82.23.157.163",
  "url": "http://httpbin.org/get?test=2"
}

# frizz.exe -t http://httpbin.org/trace?test=2 -X TRACE
405 Method Not Allowed
<html>
<head><title>405 Not Allowed</title></head>
<body>
<center><h1>405 Not Allowed</h1></center>
</body>
</html>

# frizz -t http://httpbin.org/head?test=2 -X HEAD
404 Not Found

# frizz -t http://httpbin.org/put?test=2 -X PUT
200 OK{
  "args": {
    "test": "2"
  },
  "data": "",
  "files": {},
  "form": {},
  "headers": {
    "Accept": "*/*",
    "Host": "httpbin.org",
    "User-Agent": "frizz / 0.1.0",
    "X-Amzn-Trace-Id": "Root=1-6140c5e7-0378b13c64a945840e5a2314"
  },
  "json": null,
  "origin": "82.23.157.163",
  "url": "http://httpbin.org/put?test=2"
}

```

* Scan ports for given target
```
# frizz -s --target example.net
80
443
```

* Scan ports for given port range
```
# frizz -s --target example.net -p 80 1024
80
443

```

* Specify connection timeout and concurrency level while scanning ports
```
# frizz -s --target example.net --timeout 1 -c 1024
80
443
```

Check [Git workflow](https://github.com/kursatkobya/libfrizz/wiki/Git-workflow) for how to contribute.
