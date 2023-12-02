from app import app
from flask import Response, request


@app.route("/post_large", methods=["POST"])
def post_large():
    data = request.data
    assert len(data) == 15728640
    return Response(f"{len(data)}", status=200)
