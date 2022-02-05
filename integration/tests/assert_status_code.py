# coding=utf-8
from tests import app
from flask import Response


@app.route("/assert-status-code")
def assert_status_code():
    return Response("", status=201)
