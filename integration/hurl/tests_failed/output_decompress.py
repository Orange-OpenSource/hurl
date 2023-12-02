from app import app
from flask import Response


@app.route("/error-output-decompress")
def error_output_decompress():
    headers = {}
    headers["Content-Encoding"] = "gzip"
    return Response("Hello", headers=headers)
