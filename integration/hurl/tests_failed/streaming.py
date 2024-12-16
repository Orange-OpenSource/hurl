import itertools
import time

from app import app
from flask import Response


@app.route("/streaming")
def streaming():
    def stream():
        for idx in itertools.count():
            yield str(idx) + "\n"
            time.sleep(1)

    return Response(stream(), mimetype="text/event-stream")
