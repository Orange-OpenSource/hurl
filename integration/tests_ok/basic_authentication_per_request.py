from app import app
from flask import request


@app.route("/basic-authentication-per-request")
def basic_authentication_per_request():
    print(request.headers["Authorization"])
    assert request.headers["Authorization"] == "Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ="
    return "You are authenticated"
