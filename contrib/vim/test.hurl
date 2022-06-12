# This is a comment
GET http://{{host}}
Header1: Value1
Header2: a\#    # value with \#
Header3: GET   

HTTP/1.1 200
[Asserts]
body not contains "#"            # Other comment
body not contains "[Asserts]"
body not contains "200"
body not contains "GET"
body not contains "\"\#"


POST http://example.com
{
  "id": {{id}},
  "message": "Hello"
}


POST http://example.com
```
Hello
```



