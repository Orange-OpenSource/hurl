from datetime import datetime

from app import app

last = None

counter = 0


@app.route("/delay-init")
def delay_init():
    global last, counter
    last = datetime.now()
    counter = 0
    return ""


@app.route("/delay")
def delay():
    global last
    diff = (datetime.now() - last).total_seconds()
    assert 1 < diff < 2
    last = datetime.now()
    return ""


@app.route("/delay-and-retry")
def delay_and_retry():
    global last, counter
    counter += 1

    if counter > 5:
        diff = (datetime.now() - last).total_seconds()
        assert 1 < diff < 3
        last = datetime.now()

    return f"{counter}"
