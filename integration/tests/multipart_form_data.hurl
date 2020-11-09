# curl -v -F key1=value1 -F upload1=@tests/data.txt -Fupload2=@tests/data.html -Fupload3="@tests/data.txt;type=text/html" http://localhost:8000/multipart-form-data
POST http://localhost:8000/multipart-form-data
[MultipartFormData]
key1: value1
upload1: file,data.txt;
upload2: file,data.html;
upload3: file,data.txt; text/html

HTTP/1.0 200



