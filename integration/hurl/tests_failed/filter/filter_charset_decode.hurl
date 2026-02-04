# café is utf8-encoded by the server
GET http://localhost:8000/filter-charset-decode
HTTP 200
[Asserts]
bytes charsetDecode "utf-8" == "café"        # value is decoded with success
bytes charsetDecode "unknown" == "café"      # <unknown> encoding is not supported
bytes charsetDecode "arabic" == "café"       # value can not be decoded with <arabic> encoding
bytes charsetDecode "iso-8859-1" == "café"   # value can be decoded but to an invalid string cafÃ©
