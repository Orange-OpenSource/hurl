from app import app
from flask import request


@app.route("/post-base64", methods=["POST"])
def post_base64():
    assert request.data == b"Hello World!"
    return ""
