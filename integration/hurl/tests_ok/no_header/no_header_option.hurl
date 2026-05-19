GET http://localhost:8000/no-header/foo-bar
bar: BAR
HTTP 200


GET http://localhost:8000/no-header/foo-bar
bar: BAR
baz: BAZ
[Options]
no-header: baz
no-header: qux
HTTP 200


GET http://localhost:8000/no-header/foo-bar
baz: BAZ
[Options]
header: bar:BAR
no-header: baz
HTTP 200


GET http://localhost:8000/no-header/foo-bar
bar: BAR
[Options]
header: baz:BAZ
no-header: baz
HTTP 200


GET http://localhost:8000/no-header/foo-bar-baz
bar: BAR
[Options]
header: baz:BAZ
no-header: qux
HTTP 200
