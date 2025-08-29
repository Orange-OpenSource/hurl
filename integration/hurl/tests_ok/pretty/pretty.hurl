GET http://localhost:8000/pretty/demo
HTTP 200
Content-Type: application/json
[Asserts]
jsonpath "$.strings.chinese" == "你好，世界"
jsonpath "$.numbers.large_int" == 1234567890123456789
jsonpath "$.booleans.isActive" == true
jsonpath "$.emojis.fire" == "🔥"
