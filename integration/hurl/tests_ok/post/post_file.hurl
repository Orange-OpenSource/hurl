POST http://localhost:8000/post-file
file,data.bin;
HTTP 200

POST http://localhost:8000/post-file
file,{{filename}};
HTTP 200

POST http://localhost:8000/post-file
file,post_file_with\ space;
HTTP 200
