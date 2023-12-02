from app import app
from flask import request


@app.route("/basic-authentication")
def basic_authentication():
    assert request.headers["Authorization"] == "Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ="
    return "You are authenticated"
