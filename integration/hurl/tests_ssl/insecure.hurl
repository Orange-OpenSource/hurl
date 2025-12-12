GET https://localhost:8001/hello
HTTP 200
[Asserts]
certificate "Subject" replace " = " "=" replace ";" ", " == "C=US, ST=Denial, L=Springfield, O=Dis, CN=localhost"
certificate "Issuer" replace " = " "=" replace ";" ", " == "C=US, ST=Denial, L=Springfield, O=Dis, CN=localhost"
certificate "Start-Date" isDate
certificate "Start-Date" dateFormat "%Y-%m-%d %H:%M:%S UTC" == "2025-12-12 05:18:48 UTC"
certificate "Expire-Date" isDate
certificate "Expire-Date" dateFormat "%Y-%m-%d %H:%M:%S UTC" == "2028-10-01 05:18:48 UTC"
certificate "Serial-Number" == "0f:4e:1d:d0:76:8f:04:77:5b:dc:50:c2:28:76:79:e8:b0:83:26:ef"

`Hello World!`
