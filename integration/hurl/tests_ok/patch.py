from flask import request, make_response
from app import app


@app.route("/patch/file.txt", methods=["PATCH"])
def patch():
    assert request.headers["Host"] == "www.example.com"
    assert request.headers["Content-Type"] == "application/example"
    assert request.headers["If-Match"] == '"e0023aa4e"'
    resp = make_response()
    resp.headers["Content-Location"] = "/file.txt"
    resp.headers["ETag"] = '"e0023aa4f"'
    return resp, 204
