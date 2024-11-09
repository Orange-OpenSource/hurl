# coding=utf-8
from app import app

counter = 0


@app.route("/skip/init")
def skip_init():
    global counter
    counter = 0
    return str(counter)


@app.route("/skip/increment")
def skip_increment():
    global counter
    counter = counter + 1
    return str(counter)


@app.route("/skip/get")
def skip_get():
    global counter
    return str(counter)
