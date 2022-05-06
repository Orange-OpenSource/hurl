from app import app
from flask import redirect, Response


@app.route("/follow-redirect")
def follow_redirect():
    return redirect("http://localhost:8000/following-redirect")


@app.route("/following-redirect")
def following_redirect():
    # For this redirection, we construct the response instead of using
    # Flask `redirect` function to make a redirection with a 'location' header (instead of 'Location').
    response = Response(
        response="<!DOCTYPE html>\n"
        "<title>Redirecting...</title>\n"
        "<h1>Redirecting...</h1>\n",
        status=302,
        mimetype="text/html",
    )
    response.headers["location"] = "http://localhost:8000/followed-redirect"
    return response


@app.route("/followed-redirect")
def followed_redirect():
    return "Followed redirect!"
