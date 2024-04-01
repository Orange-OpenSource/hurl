from app import app
from flask import Response


@app.route("/max-filesize-fail")
def max_filesize_fail():
    data = b"x" * 256
    return Response(data, mimetype="application/octet-stream")
