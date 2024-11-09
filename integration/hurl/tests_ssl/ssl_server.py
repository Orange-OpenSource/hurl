#!/usr/bin/env python
# usage: ./ssl_server.py <port> <cert_file> <Client certificate authentication>
# Start the server with or without client certificate authentication
import ssl
import sys

import flask

app1 = flask.Flask("SSL Server")


@app1.route("/hello")
def hello():
    return "Hello World!"


def start_server(port, cert_file, use_client_certificate_authentication):
    ssl_context = get_ssl_context(cert_file, use_client_certificate_authentication)
    app1.run(port=port, ssl_context=ssl_context)


def get_ssl_context(cert_file, use_client_certificate_authentication):
    ssl_context = ssl.SSLContext(ssl.PROTOCOL_TLSv1_2)
    if use_client_certificate_authentication:
        ssl_context.verify_mode = ssl.CERT_REQUIRED
    ssl_context.load_verify_locations("tests_ssl/certs/ca/cert.pem")
    ssl_context.load_cert_chain(cert_file, "tests_ssl/certs/server/key.pem")
    return ssl_context


def print_usage_and_exit():
    print(
        "usage: ./ssl_server.py <port> <cert_file> <use_client_certificate_authentication>"
    )
    sys.exit(1)


def main():
    if len(sys.argv) < 4:
        print_usage_and_exit()

    port = int(sys.argv[1])
    cert_file = sys.argv[2]
    if sys.argv[3] == "true":
        use_client_certificate_authentication = True
    elif sys.argv[3] == "false":
        use_client_certificate_authentication = False
    else:
        print_usage_and_exit()

    print("Starting SSL Server")
    print("  port: " + str(port))
    print("  cert file: " + cert_file)
    print(
        "  using client Certificate Authentication: "
        + ("yes" if use_client_certificate_authentication else "no")
    )
    start_server(port, cert_file, use_client_certificate_authentication)


if __name__ == "__main__":
    main()
