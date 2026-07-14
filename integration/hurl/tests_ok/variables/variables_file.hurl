GET http://localhost:8000/variable/country
[Options]
variable: country=France
variables-file: tests_ok/variables/variables_file.env
variable: country=Italy
HTTP 200
[Asserts]
variable "planet" == "The Earth"
variable "count" == 2
`{{country}}`

GET http://localhost:8000/variable/planet
HTTP 200
[Asserts]
variable "count" == 2
`{{planet}}`
