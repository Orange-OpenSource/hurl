from tests import app
from flask import Response


@app.route("/include")
def include():
    headers = {"Date": "DATE"}
    return Response("Hello", headers=headers)
