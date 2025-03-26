from app import app
from flask import jsonify, make_response, request


@app.route("/secret")
def secret():
    assert request.json == {"query": "secret1"}
    resp = jsonify(value="12345678")
    resp.set_cookie("value", "secret2")
    return resp


@app.route("/get-token")
def get_token():
    resp = make_response()
    resp.headers["x-token"] = "secret3"
    return resp


@app.route("/another-secret")
def another_secret():
    assert request.headers["x-token"] == "secret3"
    resp = make_response("Hi\n")
    return resp


counter = 0


@app.route("/get-dynamic-secret")
def dynamic_secret():
    global counter
    resp = make_response()
    resp.headers["x-dynamic-token"] = f"secret-dynamic-{counter % 3}"
    counter += 1
    return resp
