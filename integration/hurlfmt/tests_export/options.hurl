GET http://localhost:8000/hello
[Options]
aws-sigv4: aws:amz:eu-central-1:sts
cacert: cacertfile
cert: certfile
key: keyfile
compressed: false
connect-to: example.com:443:example.net:8443
delay: 1000
location: false
http1.0: false
http1.1: false
http2: false
http3: false
insecure: false
ipv4: false
ipv6: false
max-redirs: 10
output: output.txt
path-as-is: false
proxy: http://proxy.example
resolve: example.com:443:127.0.0.1
retry: 0
retry: -1
retry: 4
retry-interval: 1000
skip: false
variable: name=Bob
verbose: false
very-verbose: false

GET http://localhost:8000/hello
[Options]
aws-sigv4: {{aws-sigv4}}
#cacert: {{cacert}}         NOT SUPPORTED YET
#cert: {{cert}}             NOT SUPPORTED YET
#key: {{key}}               NOT SUPPORTED YET
compressed: {{compressed}}
connect-to: {{connect-to}}
delay: {{delay}}
location: {{location}}
http1.0: {{http10}}
http1.1: {{http11}}
http2: {{http2}}
http3: {{http3}}
insecure: {{insecure}}
ipv4: {{ipv4}}
ipv6: {{ipv6}}
max-redirs: {{max-redirs}}
#output: {{output}}         NOT SUPPORTED YET
path-as-is: {{path-as-is}}
proxy: {{proxy}}
resolve: {{resolve}}
retry: {{retry}}
retry-interval: {{retry-interval}}
skip: {{skip}}
#variable: {{variable}}     NOT SUPPORTED YET
verbose: {{verbose}}
very-verbose: {{very-verbose}}
