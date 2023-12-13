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

# 0.1 in 64-bit floating-point format is really 0.10000000000000000555111512312578270211815834045410
# both 0.1 and 0.100000000000000005 have the same 64-bit float value
jsonpath "$.float2" == 0.1
jsonpath "$.float2" == 0.10000000000000000555111512312578270211815834045410
jsonpath "$.float2" == 0.100000000000000005
jsonpath "$.float2" < 0.1000000000000001
jsonpath "$.float3" == 0.1
jsonpath "$.float3" == 0.100000000000000005

{
  "integer": 1,
  "float": 1.0,
  "float2": 0.1,
  "float3": 0.100000000000000005
}

