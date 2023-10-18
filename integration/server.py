#!/usr/bin/env python3
import argparse

from app import app, load_tests


def main():
    parser = argparse.ArgumentParser(
        description="Run Hurl integration server",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument("--host", default="127.0.0.1", help="the IP address to bind")
    parser.add_argument("--port", type=int, default=8000, help="server port")
    args = parser.parse_args()

    load_tests()
    app.run(host=args.host, port=args.port)


if __name__ == "__main__":
    main()
