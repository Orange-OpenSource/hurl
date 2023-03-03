GET https://localhost:8001/hello

HTTP 200
[Asserts]
certificate "Subject" == "C=US, ST=Denial, L=Springfield, O=Dis, CN=localhost"
certificate "Issuer" == "C=US, ST=Denial, L=Springfield, O=Dis, CN=localhost"
certificate "Start-Date" format "%Y-%M-%d %H:%m:%S UTC" == "2023-29-10 08:01:52 UTC"
certificate "Expire-Date" format "%Y-%M-%d %H:%m:%S UTC" == "2025-29-30 08:10:52 UTC"
certificate "Serial-Number" == "1e:e8:b1:7f:1b:64:d8:d6:b3:de:87:01:03:d2:a4:f5:33:53:5a:b0"

`Hello World!`