#!/bin/bash
set -Eeuo pipefail

hurl --header foo tests_failed/header/header.hurl
