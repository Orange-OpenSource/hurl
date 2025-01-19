from app import app


@app.route("/secret-failed")
def secret_failed():
    return "Hello Bob"
