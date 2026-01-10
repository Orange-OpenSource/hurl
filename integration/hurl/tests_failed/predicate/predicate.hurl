GET http://localhost:8000/predicate/error/type
HTTP 200
[Asserts]
jsonpath "$.status" == "true"
jsonpath "$.count" == 0
jsonpath "$.message" == 0
jsonpath "$.empty" == 0
jsonpath "$.number" == 1.1
jsonpath "$.count" startsWith "0"
jsonpath "$.count" endsWith "0"
jsonpath "$.count" matches "hi"
jsonpath "$.count" isEmpty
jsonpath "$.count" contains "foo"
jsonpath "$.count" isIsoDate
jsonpath "$.count" isIpv4
jsonpath "$.count" isIpv6
jsonpath "$.count" isUuid
jsonpath "$.message" startsWith "hi"
jsonpath "$.message" endsWith "hi"
jsonpath "$.message" contains "hi"
jsonpath "$.message" matches "hi"
jsonpath "$.message" matches "hi{"
jsonpath "$.message" isEmpty
jsonpath "$.message" count == 1
jsonpath "$.message" isIsoDate
jsonpath "$.message" isIpv4
jsonpath "$.message" isIpv6
jsonpath "$.message" isUuid
jsonpath "$.toto" exists
jsonpath "$.message" not exists
jsonpath "$.list" count == 2
jsonpath "$.not_exist" == 2
jsonpath "$.not_exist" > 3
jsonpath "$.not_exist" >= 3
jsonpath "$.not_exist" < 1
jsonpath "$.not_exist" <= 1
jsonpath "$.not_exist" count == 1
jsonpath "$.not_exist" startsWith "foo"
jsonpath "$.not_exist" startsWith hex,efbbbf;
jsonpath "$.not_exist" endsWith "foo"
jsonpath "$.not_exist" endsWith hex,00;
jsonpath "$.not_exist" contains 1
jsonpath "$.not_exist" matches /foo/
jsonpath "$.not_exist" isInteger
jsonpath "$.not_exist" isFloat
jsonpath "$.not_exist" isBoolean
jsonpath "$.not_exist" isString
jsonpath "$.not_exist" isCollection
jsonpath "$.not_exist" isList
jsonpath "$.not_exist" isObject
jsonpath "$.not_exist" isDate
jsonpath "$.not_exist" exists
jsonpath "$.not_exist" isEmpty
jsonpath "$.not_exist" isIpv4
jsonpath "$.not_exist" isIpv6
jsonpath "$.not_exist" isUuid
jsonpath "$.not_a_date" isIsoDate
jsonpath "$.is_a_date" not isIsoDate
jsonpath "$.not_a_date" isNumber
jsonpath "$.is_a_date" isIpv4
jsonpath "$.is_a_date" isIpv6
jsonpath "$.ipv4" isIpv6
jsonpath "$.ipv4" not isIpv4
jsonpath "$.ipv6" isIpv4
jsonpath "$.ipv6" not isIpv6
