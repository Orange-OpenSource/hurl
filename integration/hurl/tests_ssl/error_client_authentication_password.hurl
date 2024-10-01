GET https://localhost:8003/hello
[Options]
cacert: tests_ssl/server/cert.selfsigned.pem
cert: tests_ssl/client/cert.pem:toto1234
key: tests_ssl/client/encrypted.key.pem
HTTP 200

