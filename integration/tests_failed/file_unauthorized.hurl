POST http://localhost:8000/post-file
file,/secret.txt;

HTTP 200

POST http://localhost:8000/post-file
file,../secret.txt;

HTTP 200

POST http://localhost:8000/post-file
[MultipartFormData]
file1: file,/secret.txt;

HTTP 200

POST http://localhost:8000/post-file
[MultipartFormData]
file1: file,../secret.txt;

HTTP 200
