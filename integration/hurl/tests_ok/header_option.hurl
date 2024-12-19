GET http://localhost:8000/header-option
test: from-header-syntax
[Options]
header: test: from-option-1
header: test: from-option-2
header: another-test: from-option
HTTP 200
