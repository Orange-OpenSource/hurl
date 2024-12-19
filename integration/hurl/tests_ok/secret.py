from app import app
from flask import jsonify, request


@app.route("/secret")
def secret():
    assert request.json == {"query": "secret1"}
    resp = jsonify(value="secret3")
    resp.set_cookie("value", "secret2")
    return resp
