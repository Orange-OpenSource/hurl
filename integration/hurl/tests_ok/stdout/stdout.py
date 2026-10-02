from app import app


@app.route("/stdout/text")
def stdout_text():
    return "Hello"
