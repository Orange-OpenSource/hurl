#!/bin/bash
# regenerate Cryptographic pairs: CA, client, server
set -Eeuo pipefail

rm -rf ca client server
mkdir ca client server

# CA
openssl genrsa -out ca/key.pem 2048
openssl req -x509 -new -nodes -key ca/key.pem -sha256 -days 1024 -out ca/cert.pem -subj "/C=US/ST=Denial/L=Springfield/O=Dis/CN=myCA"

# Client
openssl genrsa -out client/key.pem 2048
openssl req -new -key client/key.pem -sha256 -out client/csr.pem -subj "/C=US/ST=Denial/L=Springfield/O=Dis/CN=client"
openssl x509 -req -in client/csr.pem -CA ca/cert.pem -CAkey ca/key.pem -CAcreateserial -out client/cert.pem -days 825 -sha256
openssl rsa -aes256 -in ssl/client/key.pem -passout pass:foobar -out ssl/client/encrypted.key.pem

# Server
openssl genrsa -out server/key.pem 2048
openssl req -x509 -new -nodes -key server/key.pem -sha256 -days 1024 -out server/cert.selfsigned.pem -subj "/C=US/ST=Denial/L=Springfield/O=Dis/CN=localhost"
openssl req -new -key server/key.pem -sha256 -out server/csr.pem -subj "/C=US/ST=Denial/L=Springfield/O=Dis/CN=localhost"
openssl x509 -req -in server/csr.pem -CA ca/cert.pem -CAkey ca/key.pem -CAcreateserial -out server/cert.pem -days 825 -sha256

