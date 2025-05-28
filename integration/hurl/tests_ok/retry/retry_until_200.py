# return 500 by default
# or 200 if previous request has been executed less than 5 second ago
import time

from app import app

last = 0


@app.route("/retry/until-200")
def retry_until_200():
    global last
    current = time.time()
    interval = current - last
    last = current
    if interval < 5:
        return "OK", 200
    return "", 500
