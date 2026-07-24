GET http://localhost:8000/json/store
HTTP 200
[Asserts]
jsonpath "$.store.guitar" not exists
jsonpath "$.store.book[2].title" == "Moby Dick"


GET http://localhost:8000/json/store
[Options]
no-jsonpath-coercion: true
HTTP 200
[Asserts]
jsonpath "$.store.guitar" count == 0
jsonpath "$.store.book[2].title" first == "Moby Dick"


GET http://localhost:8000/json/store
[Options]
no-jsonpath-coercion: false
HTTP 200
[Asserts]
jsonpath "$.store.guitar" not exists
jsonpath "$.store.book[2].title" == "Moby Dick"
