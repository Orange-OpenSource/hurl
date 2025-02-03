GET http://localhost:8000/hello
HTTP 200
[Asserts]
status == 200
version == "2"
url == "http://localhost8080/hello"
header "content-type" == "application/json"
certificate "Subject" replace " = " "=" replace ";" ", " == "C=US, ST=Denial, L=Springfield, O=Dis, CN=localhost"
certificate "Issuer" replace " = " "=" replace ";" ", " == "C=US, ST=Denial, L=Springfield, O=Dis, CN=localhost"
certificate "Start-Date" isDate
certificate "Start-Date" format "%Y-%m-%d %H:%M:%S UTC" == "2023-01-10 08:29:52 UTC"
certificate "Expire-Date" isDate
certificate "Expire-Date" format "%Y-%m-%d %H:%M:%S UTC" == "2025-10-30 08:29:52 UTC"
certificate "Serial-Number" == "1e:e8:b1:7f:1b:64:d8:d6:b3:de:87:01:03:d2:a4:f5:33:53:5a:b0"
cookie "JSESSIONID" exists
body == "Hello"
xpath "/users" count == 3
jsonpath "$.users" count == 3
regex "name=.*" == "Bob"
variable "name" == "Bob"
duration < 1000
sha256 == hex,7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069;
md5 == hex,ed076287532e86365e841e92bfc50d8c;
bytes startsWith hex,48656c6c6f;
