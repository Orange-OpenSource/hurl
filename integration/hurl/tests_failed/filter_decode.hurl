# café is utf8-encoded by the server
GET http://localhost:8000/filter-decode
HTTP 200
[Asserts]
bytes decode "utf-8" == "café"        # value is decoded with success
bytes decode "unknown" == "café"      # <unknown> encoding is not supported
bytes decode "arabic" == "café"       # value can not be decoded with <arabic> encoding
bytes decode "iso-8859-1" == "café"   # value can be decoded but to an invalid string cafÃ©
