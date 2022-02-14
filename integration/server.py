from app import app, load_tests

if __name__ == "__main__":
    load_tests()
    app.run(host="0.0.0.0", port=8000)
