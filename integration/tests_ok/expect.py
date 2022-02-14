from app import app
from flask import request


@app.route("/expect", methods=["POST"])
def expect():
    assert request.headers["Expect"] == "100-continue"
    s = request.data.decode("utf-8")
    assert s == """data"""
    return ""
