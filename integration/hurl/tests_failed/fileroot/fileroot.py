from app import app


@app.route("/fileroot-ko", methods=["GET", "POST"])
def fileroot_ko():
    return "Error!"
