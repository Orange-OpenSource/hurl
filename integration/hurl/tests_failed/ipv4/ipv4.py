from app import app


@app.route("/hello/ipv4")
def hello_ipv4():
    return "Hello World with IPv4!"
