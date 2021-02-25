POST http://localhost:8000/variables
{
  "name": "{{name}}",
  "age": {{age}},
  "height": {{height}},
  "female": {{female}},
  "id": "{{id}}",
  "a_null": {{a_null}}
}

HTTP/* 200
[Asserts]
variable "name" equals "Jennifer"
variable "female" equals true
variable "age" equals 30 
variable "height" equals 1.70
variable "a_null" equals null 
variable "id" equals "123"
