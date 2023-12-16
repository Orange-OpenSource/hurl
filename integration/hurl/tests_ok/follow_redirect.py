from app import app
from flask import request, redirect, Response


@app.route("/follow-redirect", methods=["GET", "POST"])
def follow_redirect():
    assert request.headers["Accept"] == "text/plain"
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
    assert request.headers["Accept"] == "text/plain"
    return "Followed redirect!"


@app.route("/followed-redirect-post", methods=["POST"])
def followed_redirect_post():
    return "Followed redirect POST!"


@app.route("/follow-redirect-308", methods=["POST"])
def follow_redirect_308():
    return redirect("http://localhost:8000/followed-redirect-post", code=308)
