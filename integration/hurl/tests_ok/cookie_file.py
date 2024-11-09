from flask import request
from app import app


@app.route("/cookie_file")
def cookie_file():
    assert request.cookies["cookie1"] == "valueA"
    return ""
