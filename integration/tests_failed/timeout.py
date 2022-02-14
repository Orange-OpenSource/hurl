from app import app
import time


@app.route("/timeout")
def timeout():
    time.sleep(2)
    return ""
