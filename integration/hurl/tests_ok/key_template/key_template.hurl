# Test template in the different Hurl "keys"

GET http://localhost:8000/key-template/header
{{key}}: value
[Options]
variable: key=name
HTTP 200


GET http://localhost:8000/key-template/querystring
[Options]
variable: key=name
[QueryStringParams]
{{key}}: value
HTTP 200


GET http://localhost:8000/key-template/querystring
[Options]
variable: key=name
[Query]
{{key}}: value
HTTP 200


POST http://localhost:8000/key-template/form
[Options]
variable: key=name
[FormParams]
{{key}}: value
HTTP 200


POST http://localhost:8000/key-template/form
[Options]
variable: key=name
[Form]
{{key}}: value
HTTP 200


POST http://localhost:8000/key-template/multipart-form-data
[Options]
variable: key1=name
variable: key2=file
[MultipartFormData]
{{key1}}: value
{{key2}}: file,data.txt;
HTTP 200


POST http://localhost:8000/key-template/multipart-form-data
[Options]
variable: key1=name
variable: key2=file
[Multipart]
{{key1}}: value
{{key2}}: file,data.txt;
HTTP 200


GET http://localhost:8000/key-template/cookie
[Options]
variable: key=name
[Cookies]
{{key}}: value
HTTP 200


GET http://localhost:8000/key-template/capture
[Options]
variable: key=name
HTTP 200
[Captures]
{{key}}: body
[Asserts]
variable "name" == "Hello"

