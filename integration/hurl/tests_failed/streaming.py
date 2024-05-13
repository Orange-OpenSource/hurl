# coding=utf-8
from app import app
from flask import Response
import time
import itertools


@app.route("/streaming")
def streaming():
    def stream():
        count = 0
        for idx in itertools.count():
            yield str(idx) + "\n"
            time.sleep(1)

    return Response(stream(), mimetype="text/event-stream")
