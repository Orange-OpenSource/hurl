# Test body response with line ending LF and CRLF.
# We receive the text body "line1\nline2\r\nline3\n"
# and not "line1\nline2\nline3\n"

#
# $ printf "line1\nline2\nline3\n" | base64
# bGluZTEKbGluZTIKbGluZTMK


GET http://localhost:8000/error-assert-base64
HTTP 200
base64,bGluZTEKbGluZTIKbGluZTMK;

