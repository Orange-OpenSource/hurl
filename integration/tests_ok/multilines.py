from app import app


@app.route("/multilines")
def multilines():
    return "line1\nline2\nline3\n"
