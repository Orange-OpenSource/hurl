GET https://localhost:8003/hello
[Options]
cacert: ssl/server/cert.selfsigned.pem
cert: ssl/client/cert.pem:foobar
key: ssl/client/encrypted.key.pem
HTTP 200

