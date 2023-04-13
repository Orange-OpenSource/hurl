# coding=utf-8
from app import app
from flask import Response


@app.route("/fail-at-end")
def fail_at_end():
    return Response("", status=200)
