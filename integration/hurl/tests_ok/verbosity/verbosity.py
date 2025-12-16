from app import app


@app.route("/verbosity", methods=["GET","POST"])
def verbosity():
    return "Lorem ipsum dolor sit amet..."
