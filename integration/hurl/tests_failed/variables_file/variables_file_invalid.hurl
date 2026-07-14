GET http://localhost:8000/hello
[Options]
variables-file: tests_failed/variables_file/variables_file_invalid.env
HTTP 200

GET http://localhost:8000/hello
[Options]
variable: b={{a}}
HTTP 200
