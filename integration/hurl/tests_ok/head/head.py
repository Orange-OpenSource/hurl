from app import app


@app.route("/head")
def head():
    return "Hello Head"
