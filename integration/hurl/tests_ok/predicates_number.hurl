# testing predicates
# on the number types: integer and float
GET http://localhost:8000/predicates-number

HTTP 200
[Asserts]

jsonpath "$.integer" exists
jsonpath "$.integer" isInteger
jsonpath "$.integer" not isFloat
jsonpath "$.integer" == 1
jsonpath "$.integer" <= 2
jsonpath "$.integer" > 0
jsonpath "$.integer" > 0.0
jsonpath "$.integer" != null

jsonpath "$.float" exists
jsonpath "$.float" isFloat
jsonpath "$.float" not isInteger
jsonpath "$.float" == 1.0
jsonpath "$.float" <= 2.0
jsonpath "$.float" > 0.0
jsonpath "$.float" > 0
jsonpath "$.float" != null

{
  "integer": 1,
  "float": 1.0
}

