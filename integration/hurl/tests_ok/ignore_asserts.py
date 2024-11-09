from app import app


@app.route("/ignore_asserts")
def ignore_asserts():
    return "Hello"
