from app import app
from flask import Response


@app.route("/include")
def include():
    return Response("Hello")
