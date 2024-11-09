from app import app
from flask import redirect


@app.route("/redirect-infinite/<number>", methods=["GET"])
def redirect_infinite_n(number):
    n = int(number)
    if n == 0:
        return "Done!"
    return redirect("http://localhost:8000/redirect-infinite/" + str(n - 1))
