from app import app
from flask import Response


@app.route("/filter-decode")
def filter_decode():
    data = """café""".encode("utf8")
    return Response(data)
