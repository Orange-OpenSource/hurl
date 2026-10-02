from datetime import datetime, timezone

from app import app

last = None

counter = 0


@app.route("/delay-init")
def delay_init():
    global last, counter
    last = datetime.now(timezone.utc)
    counter = 0
    return ""


@app.route("/delay")
def delay():
    global last
    diff = (datetime.now(timezone.utc) - last).total_seconds()
    assert 1 < diff < 2
    last = datetime.now(timezone.utc)
    return ""


@app.route("/delay-and-retry")
def delay_and_retry():
    global last, counter
    counter += 1

    if counter > 5:
        diff = (datetime.now(timezone.utc) - last).total_seconds()
        assert 1 < diff < 3
        last = datetime.now(timezone.utc)

    return f"{counter}"
