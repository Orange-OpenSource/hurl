# coding=utf-8
from app import app
from flask import Response


@app.route("/continue-on-error")
def continue_on_error():
    return Response("", status=200)
