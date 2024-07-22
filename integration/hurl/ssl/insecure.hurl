GET https://localhost:8001/hello
HTTP 200
[Asserts]
certificate "Subject" replace " = " "=" replace ";" ", " == "C=US, ST=Denial, L=Springfield, O=Dis, CN=localhost"
certificate "Issuer" replace " = " "=" replace ";" ", " == "C=US, ST=Denial, L=Springfield, O=Dis, CN=localhost"
certificate "Start-Date" isDate
certificate "Start-Date" format "%Y-%m-%d %H:%M:%S UTC" == "2023-01-10 08:29:52 UTC"
certificate "Expire-Date" isDate
certificate "Expire-Date" format "%Y-%m-%d %H:%M:%S UTC" == "2025-10-30 08:29:52 UTC"
certificate "Serial-Number" == "1e:e8:b1:7f:1b:64:d8:d6:b3:de:87:01:03:d2:a4:f5:33:53:5a:b0"

`Hello World!`