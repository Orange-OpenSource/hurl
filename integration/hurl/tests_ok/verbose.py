from app import app


@app.route("/verbose")
def verbose():
    return "Hello World!"
