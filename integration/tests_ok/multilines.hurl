GET http://localhost:8000/multilines
HTTP/* 200
[Asserts]
body == "line1\nline2\nline3\n"
body == ```line1
line2
line3
```
body == ```
line1
line2
line3
```

