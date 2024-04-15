# Tests all runner errors

# AssertBodyValueError
GET http://localhost:8000/runner_errors
HTTP 200
`Hello World`

# AssertFailure
GET http://localhost:8000/runner_errors
HTTP 200
[Asserts]
body == "Hello World"

# AssertHeaderValueError
GET http://localhost:8000/runner_errors
HTTP 200
Content-Length: 200

# AssertStatus
GET http://localhost:8000/undefined
HTTP 200

# AssertVersion
GET http://localhost:8000/undefined
HTTP/3 *

# CouldNotUncompressResponse
GET http://localhost:8000/runner_errors/could_not_uncompress
HTTP 200
[Asserts]
body startsWith "Hello"

# FileReadAccess
GET http://localhost:8000/undefined
file,undefined.txt;

# FileWriteAccess
GET http://localhost:8000/runner_errors
[Options]
output: undefined/output

# FilterDecode
GET http://localhost:8000/runner_errors
HTTP 200
[Asserts]
md5 decode "utf-8" == "café"

# FilterInvalidEncoding
GET http://localhost:8000/runner_errors
HTTP 200
[Asserts]
bytes decode "toto" == "café"

# FilterInvalidInput
GET http://localhost:8000/runner_errors
HTTP 200
[Asserts]
body toInt == 1

# FilterMissingInput
GET http://localhost:8000/runner_errors
HTTP 200
[Asserts]
header "count" toInt == 1

# HttpConnection
GET http://unknown

# InvalidCharset
# TODO: remove enum variant if not used

# InvalidDecoding
# TODO: remove enum variant if not used

# InvalidJson
GET http://localhost:8000/runner_errors
[Options]
variable: x=a
{ "a": {{x}} }

# InvalidRegex
GET http://localhost:8000/runner_errors
HTTP 200
[Asserts]
body regex "{}" == "a"

# InvalidUrl
# TODO: remove enum variant if not used

# InvalidUrlPrefix
# TODO: remove enum variant if not used

# NoQueryResult
GET http://localhost:8000/runner_errors
HTTP 200
[Captures]
count: header "count"

# QueryHeaderNotFound
GET http://localhost:8000/runner_errors
HTTP 200
count: 10

# QueryInvalidJson
GET http://localhost:8000/runner_errors
HTTP 200
[Asserts]
jsonpath "$.count" == 10

# QueryInvalidJsonpathExpression
GET http://localhost:8000/runner_errors
HTTP 200
[Asserts]
jsonpath "xxx" == 10

# QueryInvalidXml
GET http://localhost:8000/runner_errors/invalid-xml
HTTP 200
[Asserts]
xpath "//a" == 10

# QueryInvalidXpathEval
GET http://localhost:8000/runner_errors
HTTP 200
[Asserts]
xpath "//" == 10

# TemplateVariableInvalidType
GET http://localhost:8000/runner_errors
[Options]
variable: verbose=1
verbose: {{verbose}}

# TemplateVariableNotDefined
GET http://localhost:8000/runner_errors
[QueryStringParams]
param: {{value}}

# TooManyRedirect
# TODO: remove enum variant if not used

# UnauthorizedFileAccess
GET http://localhost:8000/runner_errors
file,/root/file;

# UnrenderableVariable
GET http://localhost:8000/runner_errors/json-list
HTTP 200
[Captures]
list: jsonpath "$"
GET http://localhost:8000/runner_errors
`{{list}}`

# UnsupportedContentEncoding
# TODO: remove enum variant if not used

# UnsupportedHttpVersion
# TODO: remove enum variant if not used
