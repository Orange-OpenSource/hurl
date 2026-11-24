from app import app
from flask import redirect, request


@app.route("/follow-redirect-leak/host-a-step-1")
def follow_redirect_leak_host_a_step_1():
    # step-1 domain is localhost
    expect_auth_forwarded = request.args.get("expect_auth_forwarded", "false")
    expect_cookie_forwarded = request.args.get("expect_cookie_forwarded", "false")
    assert "Authorization" in request.headers
    assert request.cookies.get("fruit") is not None
    return redirect(
        f"http://127.0.0.1:8000/follow-redirect-leak/host-b-step-2"
        f"?expect_auth_forwarded={expect_auth_forwarded}"
        f"&expect_cookie_forwarded={expect_cookie_forwarded}"
    )


@app.route("/follow-redirect-leak/host-b-step-2")
def follow_redirect_leak_host_b_step_2():
    # step-2 domain is 127.0.0.1, we change domain.
    # Credentials are forwarded only if the test expects them to be.
    expect_auth_forwarded = request.args.get("expect_auth_forwarded", "false")
    expect_cookie_forwarded = request.args.get("expect_cookie_forwarded", "false")
    if expect_auth_forwarded == "true":
        assert "Authorization" in request.headers
    else:
        assert "Authorization" not in request.headers
    if expect_cookie_forwarded == "true":
        assert request.cookies.get("fruit") is not None
    else:
        assert request.cookies.get("fruit") is None
    return redirect("http://localhost:8000/follow-redirect-leak/host-a-step-3")


@app.route("/follow-redirect-leak/host-a-step-3")
def follow_redirect_leak_host_a_step_3():
    # Back on the original host: credentials must be forwarded again, just like curl does.
    assert "Authorization" in request.headers
    assert request.cookies.get("fruit") is not None
    return "Followed redirect!"
