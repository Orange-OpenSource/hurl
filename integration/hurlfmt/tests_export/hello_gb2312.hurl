# In this test, the data returned by the server is encoded using GB2312.

# The 'Content-Type' HTTP response header precise the charset 'gb2312'
# so any text based assert are using GB2312 and can be used.
# See the sibling fail test where there is no charset => tests_failed/hello_gb2312_failed.hurl
GET http://localhost:8000/hello_gb2312
HTTP 200
[Asserts]
header "Content-Type" == "text/html; charset=gb2312"
bytes contains hex,c4e3bac3cac0bde7; # 你好世界 encoded in GB2312
xpath "string(//body)" == "你好世界"


# The 'Content-Type' HTTP response header does not precise the charset 'gb2312'
# so body must be decoded explicitly by Hurl before processing any text based assert
GET http://localhost:8000/hello_gb2312_implicit
HTTP 200
[Asserts]
header "Content-Type" == "text/html"
bytes contains hex,c4e3bac3cac0bde7; # 你好世界 encoded in GB2312
bytes decode "gb2312" xpath "string(//body)" == "你好世界"
