from app import app
from flask import redirect


@app.route("/redirect/<number>", methods=["GET"])
def redirect_n(number):
    n = int(number)
    if n == 0:
        return ""
    return redirect("http://localhost:8000/redirect/" + str(n - 1))
