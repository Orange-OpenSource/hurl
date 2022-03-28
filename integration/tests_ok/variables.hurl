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
variable "name" == "Jennifer"
variable "female" == true
variable "age" == 30
variable "height" == 1.70
variable "a_null" == null
variable "my-id" == "123"
