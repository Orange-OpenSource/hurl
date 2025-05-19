GET http://localhost:8000/header-option
key: from-header-syntax
[Options]
header: key: from-option-1
header: key: from-option-2
header: {{my_header}}
header: {{my_key}}: {{my_value}}
header: another-key: another-from-option
HTTP 200
