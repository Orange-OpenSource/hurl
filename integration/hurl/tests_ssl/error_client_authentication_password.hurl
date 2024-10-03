GET https://localhost:8003/hello
[Options]
cacert: tests_ssl/certs/server/cert.selfsigned.pem
cert: tests_ssl/certs/client/cert.pem:toto1234
key: tests_ssl/certs/client/encrypted.key.pem
HTTP 200

