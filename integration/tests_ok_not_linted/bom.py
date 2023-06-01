from app import app
from flask import request


@app.route("/utf8_bom")
def utf8_bom():
    return "Hello World!"


@app.route("/mirror", methods=["POST"])
def mirror():
    return request.data
