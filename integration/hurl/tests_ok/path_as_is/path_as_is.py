from app import app
from flask import request


@app.route("/path-as-is/../resource")
def path_as_is():
    assert request.path == "/path-as-is/../resource"
    return ""
