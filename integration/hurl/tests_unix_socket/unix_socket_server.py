#!/usr/bin/env python3
from os import path, unlink
from socket import AF_UNIX, socket

from flask import Flask

app = Flask("Unix Domain Sockets Server")


@app.route("/hello")
def hello():
    return "Hello World!"


def main():
    sock = socket(AF_UNIX)
    socket_path = "build/unix_socket.sock"

    try:
        unlink(socket_path)
    except OSError:
        if path.exists(socket_path):
            raise

    sock.bind(socket_path)

    try:
        app.run(host="unix://" + socket_path)
    finally:
        unlink(socket_path)


if __name__ == "__main__":
    main()
