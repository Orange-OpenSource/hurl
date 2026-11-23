from app import app
from flask import redirect, request


@app.route("/follow-redirect-leak/host-a-step-1")
def follow_redirect_leak_host_a_step_1():
    # step-1 domain is localhost
    trusted = get_trusted_param()
    assert "Authorization" in request.headers
    assert request.cookies.get("fruit") is not None
    return redirect(
        f"http://127.0.0.1:8000/follow-redirect-leak/host-b-step-2?trusted={trusted}"
    )


@app.route("/follow-redirect-leak/host-b-step-2")
def follow_redirect_leak_host_b_step_2():
    trusted = get_trusted_param()
    # step-1 domain is 127.0.0.1, we change domain.
    # We need to check if the credentials are here when the domain is trusted
    if trusted == "true":
        assert "Authorization" in request.headers
        assert request.cookies.get("fruit") is not None
    else:
        assert "Authorization" not in request.headers
        assert request.cookies.get("fruit") is None
    return redirect("http://localhost:8000/follow-redirect-leak/host-a-step-3")


@app.route("/follow-redirect-leak/host-a-step-3")
def follow_redirect_leak_host_a_step_3():
    # Back on the original host: credentials must be forwarded again, just like curl does.
    assert "Authorization" in request.headers
    assert request.cookies.get("fruit") is not None
    return "Followed redirect!"


def get_trusted_param():
    return request.args.get("trusted", "false")
