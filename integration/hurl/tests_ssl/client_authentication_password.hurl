GET https://localhost:8003/hello
[Options]
cacert: tests_ssl/server/cert.selfsigned.pem
cert: tests_ssl/client/cert.pem:foobar
key: tests_ssl/client/encrypted.key.pem
HTTP 200

# Using variables in filename and password
GET https://localhost:8003/hello
[Options]
variable: filename=tests_ssl/client/cert.pem
variable: password=foobar
cacert: tests_ssl/server/cert.selfsigned.pem
cert: {{filename}}:{{password}}
key: tests_ssl/client/encrypted.key.pem
HTTP 200

# Using variable for combined filename and password
GET https://localhost:8003/hello
[Options]
variable: filename_with_password=tests_ssl/client/cert.pem:foobar
cacert: tests_ssl/server/cert.selfsigned.pem
cert: {{filename_with_password}}
key: tests_ssl/client/encrypted.key.pem
HTTP 200


