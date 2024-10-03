#!/bin/bash
# regenerate Cryptographic pairs: CA, client, server
set -Eeuo pipefail

rm -rf ca client server
mkdir ca client server

# CA
openssl genrsa -out certs/ca/key.pem 2048
openssl req -x509 -new -nodes -key certs/ca/key.pem -sha256 -days 1024 -out certs/ca/cert.pem -subj "/C=US/ST=Denial/L=Springfield/O=Dis/CN=myCA"

# Client
openssl genrsa -out certs/client/key.pem 2048
openssl req -new -key certs/client/key.pem -sha256 -out certs/client/csr.pem -subj "/C=US/ST=Denial/L=Springfield/O=Dis/CN=client"
openssl x509 -req -in certs/client/csr.pem -CA certs/ca/cert.pem -CAkey certs/ca/key.pem -CAcreateserial -out certs/client/cert.pem -days 825 -sha256
openssl rsa -aes256 -in tests_ssl/certs/client/key.pem -passout pass:foobar -out tests_ssl/certs/client/encrypted.key.pem

# Server
openssl genrsa -out certs/server/key.pem 2048
openssl req -x509 -new -nodes -key certs/server/key.pem -sha256 -days 1024 -out certs/server/cert.selfsigned.pem -subj "/C=US/ST=Denial/L=Springfield/O=Dis/CN=localhost"
openssl req -new -key certs/server/key.pem -sha256 -out certs/server/csr.pem -subj "/C=US/ST=Denial/L=Springfield/O=Dis/CN=localhost"
openssl x509 -req -in certs/server/csr.pem -CA certs/ca/cert.pem -CAkey certs/ca/key.pem -CAcreateserial -out certs/server/cert.pem -days 825 -sha256

