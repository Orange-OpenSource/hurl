from app import app
from flask import Response, redirect, request


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
    assert request.headers["Accept"] == "text/plain"
    return "Followed redirect POST!"


@app.route("/follow-redirect-308", methods=["POST"])
def follow_redirect_308():
    return redirect("http://localhost:8000/followed-redirect-post", code=308)


@app.route("/follow-redirect-basic-auth")
def follow_redirect_basic_auth():
    assert "Authorization" in request.headers
    change_host = request.args.get("change_host") == "true"
    if change_host:
        return redirect("http://127.0.0.1:8000/followed-redirect-basic-auth")
    else:
        return redirect("http://localhost:8000/followed-redirect-basic-auth")


@app.route("/followed-redirect-basic-auth")
def followed_redirect_basic_auth():
    # When host changes, authorization should be filtered
    if request.headers["Host"] == "localhost:8000":
        assert "Authorization" in request.headers
        return "Followed redirect with Authorization header!"
    else:
        assert "Authorization" not in request.headers
        return "Followed redirect without Authorization header!"


@app.route("/follow-redirect-basic-auth-trusted")
def follow_redirect_basic_auth_trusted():
    return redirect("http://127.0.0.1:8000/followed-redirect-basic-auth-trusted")


@app.route("/followed-redirect-basic-auth-trusted")
def followed_redirect_basic_auth_trusted():
    assert request.headers["Authorization"] == "Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ="
    return "Followed redirect Basic Auth!"


@app.route("/follow-redirect/relative/foo")
def follow_redirect_relative():
    return redirect("../bar")


@app.route("/follow-redirect/bar")
def followed_redirect_bar():
    assert request.headers["Accept"] == "text/plain"
    return "Followed relative redirect!"
