from app import app


@app.route("/resolve")
def resolve():
    return "Hello World!"
