import time

from app import app
from flask import Response


@app.route("/sse")
def sse():
    def generate():
        yield "data: Hello world\n\n"
        time.sleep(1)
        yield "data: This is another message\n\n"
        time.sleep(1)
        yield 'data: {"key": "value"}\n\n'

    return Response(generate(), content_type="text/event-stream")
