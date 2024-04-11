# Variables can be set from:
# - an external properties file
# - at the command-line with the --variable option
# - with HURL_xxx environment variable
# - in the Hurl File in the [Options] section

POST http://localhost:8000/variables
Name: {{name}}
Age: {{age}}
Height: {{height}}
Female: {{female}}
Id: {{my-id}}
A-Null: {{a_null}}
Country: {{country}}
Planet: {{planet}}
Galaxy: {{galaxy}}
[Options]
variable: country=Italy
variable: planet=The Earth
{
  "name": "{{name}}",
  "age": {{age}},
  "height": {{height}},
  "female": {{female}},
  "id": "{{my-id}}",
  "a_null": {{a_null}},
  "country": "{{country}}",
  "planet": "{{planet}}",
  "galaxy": "{{galaxy}}"
}
HTTP 200
[Asserts]
variable "name" == "Jennifer"
variable "female" == true
variable "age" == 30
variable "height" == 1.70
variable "a_null" == null
variable "my-id" == "123"
variable "country" == "Italy"
variable "planet" == "The Earth"
variable "galaxy" == "Milky Way"


# `country` and `planet` variable are still available
# for the next entries, even if they've been defined in the
# previous `[Options]` request section.
GET http://localhost:8000/variable/country
HTTP 200
`{{country}}`

GET http://localhost:8000/variable/planet
HTTP 200
`{{planet}}`
