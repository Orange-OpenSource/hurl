from app import app


@app.route("/delete", methods=["DELETE"])
def delete():
    return ""
