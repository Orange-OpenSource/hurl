Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
if (Test-Path build/parallel-all.txt) {
    Remove-Item build/parallel-all.txt
}

# Some tests are failing but we want to continue until the end
$ErrorActionPreference = 'Continue'

hurl --ipv4 --parallel --test `
  --report-tap build/parallel-all.txt `
  tests_ok/assert_body.hurl `
  tests_ok/assert_header.hurl `
  tests_ok/assert_json.hurl `
  tests_ok/assert_match.hurl `
  tests_ok/assert_regex.hurl `
  tests_ok/assert_status_code.hurl `
  tests_ok/assert_xpath.hurl `
  tests_ok/bytes.hurl `
  tests_ok/bytes_empty.hurl `
  tests_ok/captures.hurl `
  tests_ok/charset.hurl `
  tests_ok/compressed_option.hurl `
  tests_ok/cookie_storage.hurl `
  tests_ok/empty.hurl `
  tests_ok/encoding.hurl `
  tests_ok/expect.hurl `
  tests_ok/filter.hurl `
  tests_ok/float.hurl `
  tests_ok/follow_redirect_option.hurl `
  tests_ok/form_params.hurl `
  tests_ok/gb2312.hurl `
  tests_ok/get_large.hurl `
  tests_ok/graphql.hurl `
  tests_ok/head.hurl `
  tests_ok/headers.hurl `
  tests_ok/hello.hurl `
  tests_ok/hello_gb2312.hurl `
  tests_ok/insecure_option.hurl `
  tests_ok/json_output.hurl `
  tests_ok/method.hurl `
  tests_ok/multilines.hurl `
  tests_ok/multipart_form_data.hurl `
  tests_ok/no_entry.hurl `
  tests_ok/non_utf8.hurl `
  tests_ok/output.hurl `
  tests_ok/patch.hurl `
  tests_ok/post_base64.hurl `
  tests_ok/post_bytes.hurl `
  tests_ok/post_file.hurl `
  tests_ok/post_multilines.hurl `
  tests_ok/post_xml.hurl `
  tests_ok/predicates_string.hurl `
  tests_ok/proxy_option.hurl `
  tests_ok/put.hurl `
  tests_ok/querystring_params.hurl `
  tests_ok/redirect.hurl `
  tests_ok/request_content_length.hurl `
  tests_ok/retry_option.hurl `
  tests_ok/retry_until_200.hurl `
  tests_ok/url.hurl `
  tests_ok/utf8.hurl `
  tests_ok/verbose_option.hurl `
  tests_ok_not_linted/bom.hurl `
  tests_failed/assert_base64.hurl `
  tests_failed/assert_bytearray.hurl `
  tests_failed/assert_content_encoding.hurl `
  tests_failed/assert_file.hurl `
  tests_failed/assert_header_not_found.hurl `
  tests_failed/assert_header_value.hurl `
  tests_failed/assert_http_version.hurl `
  tests_failed/assert_invalid_predicate_type.hurl `
  tests_failed/assert_match_utf8.hurl `
  tests_failed/assert_newline.hurl `
  tests_failed/assert_query_cookie.hurl `
  tests_failed/assert_query_invalid_regex.hurl `
  tests_failed/assert_query_invalid_xpath.hurl `
  tests_failed/assert_status.hurl `
  tests_failed/assert_template_variable_not_found.hurl `
  tests_failed/assert_value_error.hurl `
  tests_failed/assert_variable.hurl `
  tests_failed/assert_xpath.hurl `
  tests_failed/file_read_access.hurl `
  tests_failed/filter.hurl `
  tests_failed/filter_decode.hurl `
  tests_failed/filter_in_capture.hurl `
  tests_failed/hello_gb2312_failed.hurl `
  tests_failed/http_connection.hurl `
  tests_failed/invalid_jsonpath.hurl `
  tests_failed/invalid_url.hurl `
  tests_failed/invalid_xml.hurl `
  tests_failed/max_redirect_option.hurl `
  tests_failed/multipart_form_data.hurl `
  tests_failed/predicate.hurl `
  tests_failed/query_header_not_found.hurl `
  tests_failed/query_invalid_json.hurl `
  tests_failed/query_invalid_utf8.hurl `
  tests_failed/query_match_none.hurl `
  tests_failed/retry_option.hurl `
  tests_failed/template_variable_not_found.hurl `
  tests_failed/template_variable_not_renderable.hurl
$ErrorActionPreference = 'Stop'

Write-Host (Get-Content build/parallel-all.txt -Raw) -NoNewLine
