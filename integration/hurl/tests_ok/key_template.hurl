# Test template in the different Hurl "keys"

GET http://localhost:8000/key-template/header
{{key}}: value
[Options]
variable: key=name
HTTP 200

GET http://localhost:8000/key-template/querystring
[QueryStringParams]
{{key}}: value
[Options]
variable: key=name
HTTP 200

POST http://localhost:8000/key-template/form
[FormParams]
{{key}}: value
[Options]
variable: key=name
HTTP 200

POST http://localhost:8000/key-template/multipart-form-data
[MultipartFormData]
{{key1}}: value
{{key2}}: file,data.txt;
[Options]
variable: key1=name
variable: key2=file
HTTP 200

GET http://localhost:8000/key-template/cookie
[Cookies]
{{key}}: value
[Options]
variable: key=name
HTTP 200

GET http://localhost:8000/key-template/capture
[Options]
variable: key=name
HTTP 200
[Captures]
{{key}}: body
[Asserts]
variable "name" == "Hello"

