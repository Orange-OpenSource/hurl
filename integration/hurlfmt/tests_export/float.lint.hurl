GET http://localhost:8000/float
HTTP 200
[Asserts]
jsonpath "$[1]" == 0.0
jsonpath "$[2]" != 0.0
jsonpath "$[2]" == 0.0000000000000001
jsonpath "$[3]" == 0.000000000000001
jsonpath "$[4]" == 0.333
jsonpath "$[4]" != 0.3333333333333333
jsonpath "$[5]" == 0.3333333333333333
jsonpath "$[5]" == 0.333333333333333333  # exceeding floating-point precision in expected value
jsonpath "$[6]" == 0.333333333333333333  # exceeding floating-point precision in json response
jsonpath "$[7]" == 1.0
jsonpath "$[7]" == 1                     # using an expected integer
jsonpath "$[8]" == 1.001
jsonpath "$[9]" == 1.07
jsonpath "$[9]" == 1.070                 # additional trailing zero in expected value
jsonpath "$[10]" == 1.07                 # additional trailing zero in json response
jsonpath "$[11]" == 1.1
jsonpath "$[12]" == 1.5
[ -2.2, 0.0, 0.0000000000000001, 0.000000000000001, 0.333, 0.3333333333333333, 0.333333333333333333, 1.0, 1.001, 1.07, 1.070, 1.1, 1.5 ]
