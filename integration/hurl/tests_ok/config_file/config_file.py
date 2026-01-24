from app import app


@app.route("/config-file")
def config_file():
    return "Hello"
