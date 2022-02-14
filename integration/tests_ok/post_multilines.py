from flask import request
from app import app


@app.route("/post-multilines", methods=["POST"])
def post_multilines():
    s = request.data.decode("utf-8")
    assert s == "name,age\nbob,10\nbill,22\n"
    return ""


@app.route("/get-bob-age", methods=["GET"])
def get_bob_age():
    return "10"
