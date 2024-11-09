from flask import request
from app import app


@app.route("/path-as-is/../resource")
def path_as_is():
    assert request.path == "/path-as-is/../resource"
    return ""
