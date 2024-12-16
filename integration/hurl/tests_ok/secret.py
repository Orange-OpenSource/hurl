from app import app
from flask import jsonify, request


@app.route("/secret")
def secret():
    assert request.json == {"query": "foofoofoo"}
    return jsonify(value="baz")
