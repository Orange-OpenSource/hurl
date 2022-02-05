#!/usr/bin/env python3
import json
import sys


def check(expected, actual):
    expected_test = json.loads(expected)
    actual_test = json.loads(actual)
    check_output(expected_test, actual_test)


def check_output(expected_test, actual_test):
    expected_entries = expected_test["entries"]
    actual_entries = actual_test["entries"]
    if len(expected_entries) != len(actual_entries):
        print("expected entries: %d" % len(expected_entries))
        print("actual entries: %d" % len(actual_entries))
        sys.exit(1)
    for i in range(len(expected_entries)):
        check_entry(i, expected_entries[i], actual_entries[i])


def check_entry(entry_index, expected_entry, actual_entry):
    check_request(entry_index, expected_entry["request"], actual_entry["request"])
    check_response(entry_index, expected_entry["response"], actual_entry["response"])
    if expected_entry.get("asserts") is not None:
        check_asserts(entry_index, expected_entry["asserts"], actual_entry["asserts"])


def check_request(entry_index, expected_request, actual_request):
    check_method(entry_index, expected_request["method"], actual_request["method"])
    check_url(entry_index, expected_request["url"], actual_request["url"])


def check_response(entry_index, expected_response, actual_response):
    check_status(entry_index, expected_response["status"], actual_response["status"])


def check_method(entry_index, expected, actual):
    if expected != actual:
        print("Invalid entry %d" % entry_index)
        print("expected method: %s" % expected)
        print("actual method  : %s" % actual)
        sys.exit(1)


def check_url(entry_index, expected, actual):
    if expected != actual:
        print("Invalid entry %d" % entry_index)
        print("expected url: %s" % expected)
        print("actual url  : %s" % actual)
        sys.exit(1)


def check_status(entry_index, expected, actual):
    if expected != actual:
        print("Invalid entry %d" % entry_index)
        print("expected status: %s" % expected)
        print("actual status  : %s" % actual)
        sys.exit(1)


def check_asserts(entry_index, expected, actual):
    if expected != actual:
        print("Invalid entry %d" % entry_index)
        print("expected asserts:\n%s" % json.dumps(expected, indent=2))
        print("actual asserts:\n%s" % json.dumps(actual, indent=2))
        sys.exit(1)


def main():
    if len(sys.argv) < 3:
        print("usage: check_output.py expected_file actual_file")
        sys.exit(1)
    expected_file = sys.argv[1]
    actual_file = sys.argv[2]
    expected = open(expected_file).read()
    actual = open(actual_file).read()
    check_file(expected, actual)


if __name__ == "__main__":
    main()
