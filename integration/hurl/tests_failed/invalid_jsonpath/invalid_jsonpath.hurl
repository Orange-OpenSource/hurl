# Testing invalid jsonpath expression
GET http://localhost:8000/error-invalid-jsonpath
HTTP 200
[Asserts]
jsonpath "" == false
jsonpath "$.tags[0]x" == false
jsonpath "$.tags[0,A]" == false
jsonpath "$.tags[0:A]" == false
jsonpath "$.tags[]" == false
jsonpath "$." == false
# invalid regex expression
jsonpath "$.items[?match(@.name, '[]')].name" not exists
jsonpath "$.items[?search(@.name, '[]')]" not exists
# invalid type for regex
jsonpath "$.items[?match(@.name, 1)].name" not exists
jsonpath "$.items[?search(@.name, 1)]" not exists

jsonpath "$[?@ == 1e999]" == 1  # Number too big
jsonpath "$[?@ == 1e9999999999]" == 1 # Exponent too big

# Exceeding the 64-level recursion limit
jsonpath "$[?((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((@.id == 1))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))]" == 1
jsonpath "$[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a[?@.a]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]" == 1
