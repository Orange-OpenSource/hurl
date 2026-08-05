from app import app
from flask import request


@app.route("/post-bytes-null", methods=["POST"])
def post_bytes_null():
    assert request.data == b"\x00\x01\x02\x03"
    return ""
