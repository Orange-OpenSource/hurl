from app import app


@app.route("/connect-to")
def connect_to():
    return "Hello World!"
