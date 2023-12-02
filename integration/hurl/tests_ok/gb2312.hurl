# In this test, the data returned by the server is encoded using GB2312.
# Meanwhile, the 'Content-Type' HTTP response header doesn't precise
# any charset.
GET http://localhost:8000/gb2312
HTTP 200
[Asserts]
header "Content-Type" == "text/html"
bytes contains hex,c4e3bac3cac0bde7; # 你好世界 encoded in GB2312
bytes decode "gb2312" contains "你好"

