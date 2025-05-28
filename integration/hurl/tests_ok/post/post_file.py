from app import app
from flask import request


@app.route("/post-file", methods=["POST"])
def post_file():
    assert request.data == b"Hello World!"
    return ""
