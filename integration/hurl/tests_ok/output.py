from app import app
from flask import request


@app.route("/output/endpoint1", methods=["POST"])
def output_endpoint1():
    assert request.headers["Content-Type"] == "application/json"
    s = request.data.decode("utf-8")
    assert s == '{ "user": "bob" }'
    return "Response endpoint1\n"


@app.route("/output/endpoint2")
def output_endpoint2():
    return "Response endpoint2\n"


@app.route("/output/endpoint3")
def output_endpoint3():
    return "Response endpoint3\n"
