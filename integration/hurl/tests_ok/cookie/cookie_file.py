from app import app
from flask import request


@app.route("/cookie_file")
def cookie_file():
    assert request.cookies["cookie1"] == "valueA"
    return ""
