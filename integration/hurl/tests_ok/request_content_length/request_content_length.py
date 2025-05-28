from app import app
from flask import request


@app.route("/request-content-length-1", methods=["POST"])
def request_content_length_1():
    assert int(request.headers["Content-Length"]) == 1
    assert len(request.data) == 1
    assert request.data[0] == ord("H")
    return ""
