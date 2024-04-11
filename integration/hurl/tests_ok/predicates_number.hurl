# testing predicates
# on the number types: integer, float and big integer
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
jsonpath "$.small_float1" isFloat
jsonpath "$.small_float1" == 0.1
jsonpath "$.small_float1" == 0.10000000000000000555111512312578270211815834045410
jsonpath "$.small_float1" == 0.100000000000000005
jsonpath "$.small_float1" > 0.0999999940395355224609375000000000000000
jsonpath "$.small_float2" == 0.1

# 1000000000000000000000.5 is really 1000000000000000000000.0 if 64-bit float
jsonpath "$.big_float1" isFloat
jsonpath "$.big_float1" == 1000000000000000000000.0
jsonpath "$.big_float1" == 1000000000000000000000.5
jsonpath "$.big_float2" == 1000000000000000000000.0

jsonpath "$.big_integer" == 1000000000000000000000
jsonpath "$.big_integer" isInteger

{
  "integer": 1,
  "float": 1.0,
  "small_float1": 0.1,
  "small_float2": 0.100000000000000005,
  "big_float1": 1000000000000000000000.0,
  "big_float2": 1000000000000000000000.5,
  "big_integer": 1000000000000000000000
}

