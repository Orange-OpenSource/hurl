GET http://localhost:8000/predicate-value
HTTP 200
[Asserts]
jsonpath "$.null" == null
jsonpath "$.boolean" == true
jsonpath "$.integer" == 2
jsonpath "$.float" == 2.5
jsonpath "$.string" == "Hello"
jsonpath "$.template" == "Hello {{name}}!"
jsonpath "$.multiline_string" == ```
Hello
Bob!
```
jsonpath "$.hex" == hex,010203;
jsonpath "$.base64" == base64,bGluZTEKbGluZTINCmxpbmUzCg==;
jsonpath "$.file" == file,data.bin;
jsonpath "$.expression" == {{expression}}

