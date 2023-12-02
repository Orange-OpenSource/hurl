from app import app
from flask import request


@app.route("/output/endpoint1", methods=["POST"])
def output_endpoint1():
    assert request.headers["Content-Type"] == "application/json"
    s = request.data.decode("utf-8")
    assert s == '{ "user": "bob" }'
    return app.response_class(
        headers={"date": "DATE1"}, response="Response endpoint1\n"
    )


@app.route("/output/endpoint2")
def output_endpoint2():
    return app.response_class(
        headers={"date": "DATE2"}, response="Response endpoint2\n"
    )
