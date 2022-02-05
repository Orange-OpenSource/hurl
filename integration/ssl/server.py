#!/usr/bin/python
from flask import Flask
import os

app = Flask(__name__)


@app.route("/hello")
def hello():
    return "Hello World!"


if __name__ == "__main__":
    ssl_dir = os.path.dirname(os.path.realpath(__file__))
    app.run(port=8001, ssl_context=(ssl_dir + "/cert.pem", ssl_dir + "/key.pem"))
