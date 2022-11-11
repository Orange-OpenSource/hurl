GET http://localhost:8000/hello
HTTP 200
[Asserts]
xpath "//item" equals "\d\d\d\d-\d\d-\d\d"
#xpath "//item" contains "\\d\\d\\d\\d-\\d\\d-\\d\\d"
# you need to escape twice (one for regex one for json
