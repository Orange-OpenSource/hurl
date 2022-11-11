GET http://localhost:8000/assert-match
HTTP 200
[Asserts]
jsonpath "$.date1" matches "\\d{4}-\\d{2}-\\d{2}"
jsonpath "$.date1" matches /\d{4}-\d{2}-\d{2}/
jsonpath "$.date2" matches /\d{4}-\d{2}-\d{2}/
jsonpath "$.date1" matches /^\d{4}-\d{2}-\d{2}$/
jsonpath "$.date2" not matches /^\d{4}-\d{2}-\d{2}$/
jsonpath "$.path1" matches /aa\/bb/
jsonpath "$.path2" matches /aa\\bb/
