from app import app


@app.route("/parallel/hello/<name>")
def parallel_hello(name: str):
    return hello_from(name)


def hello_from(name: str) -> str:
    return f"Hello from {name}!\n"
