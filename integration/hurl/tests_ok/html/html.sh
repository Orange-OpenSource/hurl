#!/bin/bash
set -Eeuo pipefail

# Some tests are failing but we want to continue until the end
set +euo pipefail
hurl --ipv4 --test \
  --report-html build/a/b/c/ \
  --glob "tests_ok/test/test.*.hurl"

hurl --ipv4 --test \
  --report-html build/a/b/c/ \
  tests_ok/assert/assert_body.hurl \
  tests_ok/assert/assert_header.hurl \
  tests_ok/assert/assert_json.hurl \
  tests_ok/assert/assert_match.hurl \
  tests_ok/assert/assert_regex.hurl \
  tests_ok/assert/assert_status_code.hurl \
  tests_ok/assert/assert_xpath.hurl \
  tests_ok/bytes/bytes.hurl \
  tests_ok/bytes/bytes_empty.hurl \
  tests_ok/captures/captures.hurl \
  tests_ok/charset/charset.hurl \
  tests_ok/compressed/compressed_option.hurl \
  tests_ok/cookie/cookie_storage.hurl \
  tests_ok/empty/empty.hurl \
  tests_ok/encoding/encoding.hurl \
  tests_ok/expect/expect.hurl \
  tests_ok/filter/filter.hurl \
  tests_ok/float/float.hurl \
  tests_ok/follow_redirect/follow_redirect_option.hurl \
  tests_ok/form_params/form_params.hurl \
  tests_ok/gb2312/gb2312.hurl \
  tests_ok/get_large/get_large.hurl \
  tests_ok/graphql/graphql.hurl \
  tests_ok/head/head.hurl \
  tests_ok/header/headers.hurl \
  tests_ok/hello/hello.hurl \
  tests_ok/hello/hello_gb2312.hurl \
  tests_ok/http_version/http_version_option.hurl \
  tests_ok/insecure/insecure_option.hurl \
  tests_ok/json_output/json_output.hurl \
  tests_ok/method/method.hurl \
  tests_ok/multilines/multilines.hurl \
  tests_ok/multipart/multipart_form_data.hurl \
  tests_ok/no_entry/no_entry.hurl \
  tests_ok/non_utf8/non_utf8.hurl \
  tests_ok/output/output.hurl \
  tests_ok/patch/patch.hurl \
  tests_ok/post/post_base64.hurl \
  tests_ok/post/post_bytes.hurl \
  tests_ok/post/post_file.hurl \
  tests_ok/post/post_multilines.hurl \
  tests_ok/post/post_xml.hurl \
  tests_ok/predicates/predicates_string.hurl \
  tests_ok/proxy/proxy_option.hurl \
  tests_ok/put/put.hurl \
  tests_ok/querystring/querystring_params.hurl \
  tests_ok/redirect/redirect.hurl \
  tests_ok/request_content_length/request_content_length.hurl \
  tests_ok/retry/retry_option.hurl \
  tests_ok/retry/retry_until_200.hurl \
  tests_ok/url/url.hurl \
  tests_ok/utf8/utf8.hurl \
  tests_ok/verbose/verbose_option.hurl \
  tests_ok_not_linted/bom.hurl \
  tests_failed/assert_base64/assert_base64.hurl \
  tests_failed/assert_bytearray/assert_bytearray.hurl \
  tests_failed/assert_content_encoding/assert_content_encoding.hurl \
  tests_failed/assert_file/assert_file.hurl \
  tests_failed/assert_header/assert_header_not_found.hurl \
  tests_failed/assert_header/assert_header_value.hurl \
  tests_failed/assert_http_version/assert_http_version.hurl \
  tests_failed/assert_invalid_predicate_type/assert_invalid_predicate_type.hurl \
  tests_failed/assert_match_utf8/assert_match_utf8.hurl \
  tests_failed/assert_newline/assert_newline.hurl \
  tests_failed/assert_query/assert_query_cookie.hurl \
  tests_failed/assert_query/assert_query_invalid_regex.hurl \
  tests_failed/assert_query/assert_query_invalid_xpath.hurl \
  tests_failed/assert_status/assert_status.hurl \
  tests_failed/assert_template/assert_template_variable_not_found.hurl \
  tests_failed/assert_value_error/assert_value_error.hurl \
  tests_failed/assert_variable/assert_variable.hurl \
  tests_failed/assert_xpath/assert_xpath.hurl \
  tests_failed/file_read_access.hurl \
  tests_failed/filter.hurl \
  tests_failed/filter_decode.hurl \
  tests_failed/filter_in_capture.hurl \
  tests_failed/hello_gb2312_failed.hurl \
  tests_failed/http_connection.hurl \
  tests_failed/invalid_jsonpath.hurl \
  tests_failed/invalid_url.hurl \
  tests_failed/invalid_xml.hurl \
  tests_failed/max_redirect_option.hurl \
  tests_failed/multipart_form_data.hurl \
  tests_failed/predicate.hurl \
  tests_failed/query_header_not_found.hurl \
  tests_failed/query_invalid_json.hurl \
  tests_failed/query_invalid_utf8.hurl \
  tests_failed/query_match_none.hurl \
  tests_failed/retry_option.hurl \
  tests_failed/template_variable_not_found.hurl \
  tests_failed/template_variable_not_renderable.hurl
