from app import app


@app.route("/color")
def color():
    return "Blue, Orange, Green!"
