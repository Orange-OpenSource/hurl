#!/usr/bin/env python3
# Run Hurl files one by one
# appending to JSON/XML/HTML reports
#
import sys
import subprocess
import os
import glob
from lxml import etree
import shutil
from typing import List, Dict


def get_files(glob_expr: str) -> List[str]:
    """return sorted list of files matching a glob expresion

    Arguments:
      glob_expr -- the input file glob expression

    Example:
      get_files("tests_ok/*.hurl")
    """
    return sorted([f.replace("\\", "/") for f in glob.glob(glob_expr)])


def get_options(hurl_file: str) -> List[str]:
    """return the command-line options for a given Hurl file.

    Arguments:
      hurl_file -- the Hurl file to run

    """
    options = [
        "--report-html",
        "build/html",
        "--report-junit",
        "build/tests.xml",
        "--json",
    ]
    options_file = hurl_file.replace(".hurl", ".options")
    if os.path.exists(options_file):
        for option in open(options_file, encoding="utf-8").read().strip().split("\n"):
            options.append(option)
    return options


def get_env(hurl_file: str) -> Dict[str, str]:
    """return the env for a given Hurl file.

    Arguments:
      hurl_file -- the Hurl file to run
    """
    env = os.environ.copy()
    profile_file = hurl_file.replace(".hurl", ".profile")
    if os.path.exists(profile_file):
        for line in open(profile_file, encoding="utf-8").readlines():
            line = line.strip()
            if line == "":
                continue
            index = line.index("=")
            name = line[:index]
            value = line[(index + 1) :]
            env[name] = value
    return env


def exec_hurl(hurl_file: str):
    """exec a Hurl file returning the CompletedProcess
    https://docs.python.org/3/library/subprocess.html#subprocess.CompletedProcess

    Arguments:
      hurl_file -- the Hurl file to run
    """
    cmd = ["hurl", hurl_file] + get_options(hurl_file)
    print(" ".join(cmd))
    result = subprocess.run(
        cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=get_env(hurl_file)
    )
    return result


def exec_hurl_files():
    """exec hurl files returning the count of files executed
    throw Exception in case of unexpected exit code
    """
    count = 0
    json_output_file = open("build/tests.json", "w")

    for hurl_file in get_files("tests_ok/*.hurl"):
        count += 1
        result = exec_hurl(hurl_file)
        if result.returncode != 0:
            raise Exception("unexpected exit code %d" % result.returncode)
        json_output_file.write(result.stdout.decode("utf-8"))

    for hurl_file in get_files("tests_failed/*.hurl"):
        count += 1
        result = exec_hurl(hurl_file)
        if result.returncode != 0 and result.returncode != 3 and result.returncode != 4:
            raise Exception("unexpected exit code %d" % result.returncode)
        json_output_file.write(result.stdout.decode("utf-8"))
    json_output_file.close()
    return count


def check_count(expected_count: int):
    """checkout number of testscase in JSON/XML report
    throw Exception if it does not atch
    """
    count_in_json = len(open("build/tests.json").readlines())
    if count_in_json != expected_count:
        raise Exception("count mismatch in JSON report: %d" % count_in_json)

    tree = etree.fromstring(open("build/tests.xml").read().encode("utf-8"))
    count_in_xml = len(tree.xpath("//testcase"))
    if count_in_xml != expected_count:
        raise Exception("count mismatch in XML report: %d" % count_in_xml)


def main():
    shutil.rmtree("build", ignore_errors=True)
    os.mkdir("build")

    try:
        count = exec_hurl_files()
        print("Total number of tests: %d" % count)
        check_count(count)
    except Exception as e:
        print(e)
        sys.exit(1)


if __name__ == "__main__":
    main()
