from app import app
from datetime import datetime

init = None


@app.route("/delay-init")
def delay_init():
    global init
    init = datetime.now()
    return ""


@app.route("/delay")
def delay():
    diff = (datetime.now() - init).total_seconds()
    assert diff > 1
    return ""
