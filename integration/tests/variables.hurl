POST http://localhost:8000/variables
Name: {{name}}
Age: {{age}}
Height: {{height}}
Female: {{female}}
Id: {{my-id}}
A_Null: {{a_null}}
{
  "name": "{{name}}",
  "age": {{age}},
  "height": {{height}},
  "female": {{female}},
  "id": "{{my-id}}",
  "a_null": {{a_null}}
}

HTTP/* 200
[Asserts]
variable "name" equals "Jennifer"
variable "female" equals true
variable "age" equals 30 
variable "height" equals 1.70
variable "a_null" equals null 
variable "my-id" equals "123"
