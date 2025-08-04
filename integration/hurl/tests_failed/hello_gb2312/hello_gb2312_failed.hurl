# In this test, the data returned by the server is encoded using GB2312.
# Meanwhile, the 'Content-Type' HTTP response header doesn't precise
# any charset. In this case, any text based assert default to UFT-8 encoding and
# will probably fail.
GET http://localhost:8000/hello_gb2312_failed
HTTP 200
[Asserts]
header "Content-Type" == "text/html"
bytes contains hex,c4e3bac3cac0bde7; # 你好世界 encoded in GB2312
xpath "string(//body)" == "你好世界"

