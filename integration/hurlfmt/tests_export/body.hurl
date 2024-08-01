POST http://localhost:8000/json
{
  "message": "Hello",
  "count": 5,
  "success": false,
  "errors": [{"id":"error1"},{"id":"error2"}],
  "failures": [{"id":"failure1"}],
  "warnings": [],
  "duration": 1.5,
  "tags": ["test"],
  "nullable": null,
  "profile-id": "123abc",
  "empty": {},
  "name": {{name}}
}

POST http://localhost:8000/xml
<message>Hello</message>

POST http://localhost:8000/multiline-string
```
Hello
```

POST http://localhost:8000/multiline-string
```escape
Hello\tWorld!
```

POST http://localhost:8000/oneline-string
`Hello`

POST http://localhost:8000/oneline-base64
base64,bGluZTEKbGluZTINCmxpbmUzCg==;

POST http://localhost:8000/oneline-base64
base64,ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/==;

POST http://localhost:8000/oneline-file
file,data.bin;

POST http://localhost:8000/oneline-hex
hex,010203;
