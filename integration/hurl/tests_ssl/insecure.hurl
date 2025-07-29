GET https://localhost:8001/hello
HTTP 200
[Asserts]
certificate "Subject" replace " = " "=" replace ";" ", " == "C=US, ST=Denial, L=Springfield, O=Dis, CN=localhost"
certificate "Issuer" replace " = " "=" replace ";" ", " == "C=US, ST=Denial, L=Springfield, O=Dis, CN=localhost"
certificate "Start-Date" isDate
certificate "Start-Date" dateFormat "%Y-%m-%d %H:%M:%S UTC" == "2025-03-15 18:09:11 UTC"
certificate "Expire-Date" isDate
certificate "Expire-Date" dateFormat "%Y-%m-%d %H:%M:%S UTC" == "2028-01-03 18:09:11 UTC"
certificate "Serial-Number" == "2d:bb:19:8d:b1:07:64:5a:78:47:96:e0:9a:26:9d:c8:d6:1c:0a:27"

`Hello World!`
