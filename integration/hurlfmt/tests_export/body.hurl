POST http://localhost:8000/json
{
   "message": "Hello"
}

POST http://localhost:8000/xml
<message>Hello</message>

POST http://localhost:8000/multiline-string
```
Hello
```

POST http://localhost:8000/oneline-string
```Hello```

POST http://localhost:8000/oneline-base64
base64,bGluZTEKbGluZTINCmxpbmUzCg==;

POST http://localhost:8000/oneline-file
file,data.bin;

POST http://localhost:8000/oneline-hex
hex,010203;
