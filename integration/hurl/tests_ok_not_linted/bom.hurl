# This Hurl file begins with a BOM marker.
# You can check it with 'hexdump -C bom.hurl'
GET http://localhost:8000/utf8_bom

HTTP 200
`Hello World!`

# We send the content of this file and get back the same
# bytes from /mirror to test the presence of the BOM marker.
POST http://localhost:8000/mirror
file,bom.hurl;

HTTP 200
[Asserts]
bytes startsWith hex,efbbbf;
