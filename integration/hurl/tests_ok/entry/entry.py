# coding=utf-8
from app import app


@app.route("/to-entry/<entry_count>")
def to_entry(entry_count: 1):
    return f"Reached entry {entry_count}\n"
