#!/bin/bash
set -Eeuo pipefail

hurl --no-color --parallel tests_failed_not_linted/parallel_io_a.hurl tests_failed_not_linted/parallel_io_b.hurl tests_failed_not_linted/parallel_io_c.hurl
