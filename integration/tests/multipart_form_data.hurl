# curl -v -F key1=value1 -F upload1=@tests/hello.txt -Fupload2=@tests/hello.html -Fupload3="@tests/hello.txt;type=text/html" http://localhost:8000/multipart-form-data
POST http://localhost:8000/multipart-form-data
[MultipartFormData]
key1: value1
upload1: file,hello.txt;
upload2: file,hello.html;
upload3: file,hello.txt; text/html

HTTP/1.0 200



