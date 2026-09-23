from app import app
from flask import redirect, request


@app.route("/follow-redirect-leak/host-a-step-1")
def follow_redirect_leak_host_a_step_1():
    assert "Authorization" in request.headers
    assert request.cookies.get("fruit") is not None
    return redirect("http://127.0.0.1:8000/follow-redirect-leak/host-b-step-2")


@app.route("/follow-redirect-leak/host-b-step-2")
def follow_redirect_leak_host_b_step_2():
    assert "Authorization" not in request.headers
    assert request.cookies.get("fruit") is None
    return redirect("http://localhost:8000/follow-redirect-leak/host-a-step-3")


@app.route("/follow-redirect-leak/host-a-step-3")
def follow_redirect_leak_host_a_step_3():
    # Back on the original host: credentials must be forwarded again, just like curl does.
    assert "Authorization" in request.headers
    assert request.cookies.get("fruit") is not None
    return "Followed redirect!"
