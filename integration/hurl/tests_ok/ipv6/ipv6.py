from app import app


@app.route("/hello/ipv6")
def hello_ipv6():
    return "Hello World with IPv6!"
