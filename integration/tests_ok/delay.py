from app import app
from datetime import datetime

last = None


@app.route("/delay-init")
def delay_init():
    global last
    last = datetime.now()
    return ""


@app.route("/delay")
def delay():
    global last
    diff = (datetime.now() - last).total_seconds()
    assert diff > 1
    last = datetime.now()
    return ""
