from app import app
from flask import request


@app.route("/post-multilines", methods=["POST"])
def post_multilines():
    s = request.data.decode("utf-8")
    assert s == "name,age\nbob,10\nbill,22\n"
    return ""


@app.route("/post-multilines-json", methods=["POST"])
def post_multilines_json():
    s = request.data.decode("utf-8")
    assert (
        s
        == """{
    "g_clef": "𝄞"
}
"""
    )
    return ""


@app.route("/get-bob-age", methods=["GET"])
def get_bob_age():
    return "10"
