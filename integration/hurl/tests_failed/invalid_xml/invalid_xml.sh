#!/bin/bash
set -Eeuo pipefail

hurl tests_failed/invalid_xml/invalid_xml.hurl
