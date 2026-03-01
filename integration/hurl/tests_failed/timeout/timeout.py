import time

from app import app


@app.route("/timeout")
def timeout():
    time.sleep(2)
    return ""
