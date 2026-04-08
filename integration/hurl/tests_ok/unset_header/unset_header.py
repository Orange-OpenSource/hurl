from app import app
from flask import request


@app.route("/unset-header")
def unset_header():
    assert "Authorization" not in request.headers
    return ""
