from flask import request
from tests import app


@app.route("/post-bytes", methods=["POST"])
def post_bytes():
    assert request.data == b"\x01\x02\x03"
    return ""
