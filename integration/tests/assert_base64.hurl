# Test body response with line ending LF and CRLF.
# We receive the text body "line1\nline2\r\nline3\n"
#
# $ printf "line1\nline2\r\nline3\n" | base64
# bGluZTEKbGluZTINCmxpbmUzCg==


GET http://localhost:8000/assert-base64
HTTP/1.0 200
base64,bGluZTEKbGluZTINCmxpbmUzCg==;

