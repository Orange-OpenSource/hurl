GET https://localhost:8003/hello
[Options]
cacert: ssl/server/cert.selfsigned.pem
cert: ssl/client/cert.pem:foobar
key: ssl/client/encrypted.key.pem
HTTP 200

# Using variables in filename and passphrase
GET https://localhost:8003/hello
[Options]
variable: filename=ssl/client/cert.pem
variable: passphrase=foobar
cacert: ssl/server/cert.selfsigned.pem
cert: {{filename}}:{{passphrase}}
key: ssl/client/encrypted.key.pem
HTTP 200

# Using variable for combined filename and passphrase
GET https://localhost:8003/hello
[Options]
variable: filename_with_passphrase=ssl/client/cert.pem:foobar
cacert: ssl/server/cert.selfsigned.pem
cert: {{filename_with_passphrase}}
key: ssl/client/encrypted.key.pem
HTTP 200


