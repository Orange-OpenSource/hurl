# Issue <https://github.com/Orange-OpenSource/hurl/issues/4576>
# When the response is smaller than an existing output file, the output file is not truncated.
POST http://localhost:8000/output/endpoint1
[Options]
output: output_existing_tiny_option.bin
{ "user": "bob" }
HTTP 200
