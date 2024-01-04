# Go through a Unix Domain Socket
# Requests will be sent to the Unix Domain Socket instead of over the network
GET http://example/hello
[Options]
unix-socket: build/unix_socket.sock
HTTP 200
[Asserts]
`Hello World!`
