# coding=utf-8
from app import app
from flask import make_response


@app.route("/body-binary")
def body_binary():
    with open("tests_failed/cat.avif", "rb") as data:
        response = make_response(data.read())
        response.headers.set("Content-Type", "image/png")
        return response
