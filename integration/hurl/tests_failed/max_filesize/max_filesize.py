from app import app
from flask import Response, redirect


@app.route("/return-256-bytes")
def return_256_bytes():
    data = b"x" * 256
    return Response(data, mimetype="application/octet-stream")


@app.route("/return-100-bytes")
def return_100_bytes():
    data = b"x" * 100
    return Response(data, mimetype="application/octet-stream")


@app.route("/redirect-to-return-256-bytes")
def redirect_to_return_256_bytes():
    return redirect("http://localhost:8000/return-256-bytes")
