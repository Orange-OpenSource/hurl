GET https://localhost:8003/hello
[Options]
cacert: ssl/server/cert.selfsigned.pem
cert: ssl/client/cert.pem:foobar
key: ssl/client/encrypted.key.pem
HTTP 200

# Using variables in filename and password
GET https://localhost:8003/hello
[Options]
variable: filename=ssl/client/cert.pem
variable: password=foobar
cacert: ssl/server/cert.selfsigned.pem
cert: {{filename}}:{{password}}
key: ssl/client/encrypted.key.pem
HTTP 200

# Using variable for combined filename and password
GET https://localhost:8003/hello
[Options]
variable: filename_with_password=ssl/client/cert.pem:foobar
cacert: ssl/server/cert.selfsigned.pem
cert: {{filename_with_password}}
key: ssl/client/encrypted.key.pem
HTTP 200


