from app import app
from flask import Response


@app.route("/filter-charset-decode")
def filter_charset_decode():
    data = """café""".encode()
    return Response(data)
