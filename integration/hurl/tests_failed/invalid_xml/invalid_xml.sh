#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/invalid_xml/invalid_xml.hurl
