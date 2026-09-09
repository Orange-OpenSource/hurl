GET http://localhost:8000/fileroot-ko
[Options]
output: authorized.bin
HTTP 200


POST http://localhost:8000/fileroot-ko
file,authorized.bin;
HTTP 200


GET http://localhost:8000/fileroot-ko
[Options]
output: authorized_dir/output.bin
HTTP 200
