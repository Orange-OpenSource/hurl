# Grammar

## Definitions

Short description:

- operator &#124; denotes alternative,
- operator * denotes iteration (zero or more),
- operator + denotes iteration (one or more),

## Syntax Grammar

<div class="grammar">
<div class="rule">
  <div class="non-terminal" id="hurl-file">hurl-file&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#entry">entry</a>*<br>&nbsp;
<a href="#lt">lt</a>*</div></div>
<div class="rule">
  <div class="non-terminal" id="entry">entry&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#request">request</a><br>&nbsp;
<a href="#response">response</a>?</div></div>
<div class="rule">
  <div class="non-terminal" id="request">request&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#lt">lt</a>*<br>&nbsp;
<a href="#sp">sp</a>* <a href="#method">method</a> <a href="#sp">sp</a> <a href="#sp">sp</a>* <a href="#url">url</a> <a href="#lt">lt</a><br>&nbsp;
<a href="#header">header</a>*<br>&nbsp;
<a href="#request-section">request-section</a>*<br>&nbsp;
<a href="#body">body</a>?</div></div>
<div class="rule">
  <div class="non-terminal" id="response">response&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#lt">lt</a>*<br>&nbsp;
<a href="#sp">sp</a>* <a href="#version">version</a> <a href="#sp">sp</a> <a href="#sp">sp</a>* <a href="#status">status</a> <a href="#lt">lt</a><br>&nbsp;
<a href="#header">header</a>*<br>&nbsp;
<a href="#response-section">response-section</a>*<br>&nbsp;
<a href="#body">body</a>?</div></div>
<div class="rule">
  <div class="non-terminal" id="method">method&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"GET"</span><br>
| <span class="terminal">"HEAD"</span><br>
| <span class="terminal">"POST"</span><br>
| <span class="terminal">"PUT"</span><br>
| <span class="terminal">"DELETE"</span><br>
| <span class="terminal">"CONNECT"</span><br>
| <span class="terminal">"OPTIONS"</span><br>
| <span class="terminal">"TRACE"</span><br>
| <span class="terminal">"PATCH"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="url">url&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="definition">&lt;(alphanum | ":" | "/" | "{" | "}" | "*" | "," | "@" | "]")+&gt;</span></div></div>
<div class="rule">
  <div class="non-terminal" id="version">version&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"HTTP/1.0"</span> | <span class="terminal">"HTTP/1.1"</span> | <span class="terminal">"HTTP/2"</span> | <span class="terminal">"HTTP/*"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="status">status&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="definition">&lt;[0-9]+&gt;</span></div></div>
<div class="rule">
  <div class="non-terminal" id="header">header&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#lt">lt</a>*<br>&nbsp;
<a href="#sp">sp</a>* <a href="#key-value">key-value</a> <a href="#lt">lt</a></div></div>
<div class="rule">
  <div class="non-terminal" id="body">body&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#lt">lt</a>*<br>&nbsp;
<a href="#sp">sp</a>* <a href="#bytes">bytes</a> <a href="#lt">lt</a></div></div>
<div class="rule">
  <div class="non-terminal" id="request-section">request-section&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#query-string-params-section">query-string-params-section</a><br>
| <a href="#form-params-section">form-params-section</a><br>
| <a href="#multipart-form-data-section">multipart-form-data-section</a><br>
| <a href="#cookies-section">cookies-section</a></div></div>
<div class="rule">
  <div class="non-terminal" id="response-section">response-section&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#captures-section">captures-section</a> | <a href="#asserts-section">asserts-section</a></div></div>
<div class="rule">
  <div class="non-terminal" id="query-string-params-section">query-string-params-section&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#lt">lt</a>*<br>&nbsp;
<a href="#sp">sp</a>* <span class="terminal">"[QueryStringParams]"</span> <a href="#lt">lt</a><br>&nbsp;
<a href="#param">param</a>*</div></div>
<div class="rule">
  <div class="non-terminal" id="form-params-section">form-params-section&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#lt">lt</a>*<br>&nbsp;
<a href="#sp">sp</a>* <span class="terminal">"[FormParams]"</span> <a href="#lt">lt</a><br>&nbsp;
<a href="#param">param</a>*</div></div>
<div class="rule">
  <div class="non-terminal" id="multipart-form-data-section">multipart-form-data-section&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#lt">lt</a>*<br>&nbsp;
<a href="#sp">sp</a>* <span class="terminal">"[MultipartFormData]"</span> <a href="#lt">lt</a><br>&nbsp;
<a href="#multipart-form-data-param">multipart-form-data-param</a>*</div></div>
<div class="rule">
  <div class="non-terminal" id="cookies-section">cookies-section&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#lt">lt</a>*<br>&nbsp;
<a href="#sp">sp</a>* <span class="terminal">"[Cookies]"</span> <a href="#lt">lt</a><br>&nbsp;
<a href="#cookie">cookie</a>*</div></div>
<div class="rule">
  <div class="non-terminal" id="captures-section">captures-section&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#lt">lt</a>*<br>&nbsp;
<a href="#sp">sp</a>* <span class="terminal">"[Captures]"</span> <a href="#lt">lt</a><br>&nbsp;
<a href="#capture">capture</a>*</div></div>
<div class="rule">
  <div class="non-terminal" id="asserts-section">asserts-section&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#lt">lt</a>*<br>&nbsp;
<a href="#sp">sp</a>* <span class="terminal">"[Asserts]"</span> <a href="#lt">lt</a><br>&nbsp;
<a href="#assert">assert</a>*</div></div>
<div class="rule">
  <div class="non-terminal" id="param">param&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#lt">lt</a>*<br>&nbsp;
<a href="#sp">sp</a>* <a href="#key-value">key-value</a> <a href="#lt">lt</a></div></div>
<div class="rule">
  <div class="non-terminal" id="multipart-form-data-param">multipart-form-data-param&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#file-param">file-param</a> | <a href="#param">param</a></div></div>
<div class="rule">
  <div class="non-terminal" id="file-param">file-param&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#lt">lt</a>*<br>&nbsp;
<a href="#sp">sp</a>* <a href="#key-string">key-string</a> <a href="#sp">sp</a>* <span class="terminal">":"</span> <a href="#sp">sp</a>* <a href="#file-value">file-value</a> <a href="#lt">lt</a></div></div>
<div class="rule">
  <div class="non-terminal" id="file-value">file-value&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"file,"</span> <a href="#sp">sp</a>* <a href="#filename">filename</a> <a href="#sp">sp</a>* <span class="terminal">";"</span> (<a href="#sp">sp</a>* <a href="#file-contenttype">file-contenttype</a>)?</div></div>
<div class="rule">
  <div class="non-terminal" id="file-contenttype">file-contenttype&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="definition">&lt;(alphanum | "/" | ";" | "=" | " ")+  without leading/trailing space&gt;</span></div></div>
<div class="rule">
  <div class="non-terminal" id="cookie">cookie&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#lt">lt</a>*<br>&nbsp;
<a href="#sp">sp</a>* <a href="#key-string">key-string</a> <a href="#sp">sp</a>* <span class="terminal">":"</span> <a href="#sp">sp</a>* <a href="#cookie-value">cookie-value</a> <a href="#lt">lt</a></div></div>
<div class="rule">
  <div class="non-terminal" id="cookie-value">cookie-value&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="definition">&lt;(alphanum | "!" | "#" | "$" | "%" | "&" | "'" | "(" | ")" | "*" | "+"
                           | "-" | "." | "/" | ":" | "<" | "=" | ">" | "?" | "@" | "["
                           | "]" | "^" | "_" | "`" | "~" )* &gt;</span></div></div>
<div class="rule">
  <div class="non-terminal" id="capture">capture&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#lt">lt</a>*<br>&nbsp;
<a href="#sp">sp</a>* <a href="#key-string">key-string</a> <a href="#sp">sp</a>* <span class="terminal">":"</span> <a href="#sp">sp</a>* <a href="#query">query</a> <a href="#lt">lt</a></div></div>
<div class="rule">
  <div class="non-terminal" id="query">query&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#main-query">main-query</a> (<a href="#sp">sp</a> <a href="#sp">sp</a>* <a href="#subquery">subquery</a>)?</div></div>
<div class="rule">
  <div class="non-terminal" id="main-query">main-query&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#status-query">status-query</a><br>
| <a href="#header-query">header-query</a><br>
| <a href="#cookie-query">cookie-query</a><br>
| <a href="#body-query">body-query</a><br>
| <a href="#xpath-query">xpath-query</a><br>
| <a href="#jsonpath-query">jsonpath-query</a><br>
| <a href="#regex-query">regex-query</a><br>
| <a href="#variable-query">variable-query</a><br>
| <a href="#duration-query">duration-query</a><br>
| <a href="#bytes-query">bytes-query</a><br>
| <a href="#sha256-query">sha256-query</a></div></div>
<div class="rule">
  <div class="non-terminal" id="status-query">status-query&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"status"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="header-query">header-query&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"header"</span> <a href="#sp">sp</a> <a href="#sp">sp</a>* <a href="#quoted-string">quoted-string</a></div></div>
<div class="rule">
  <div class="non-terminal" id="cookie-query">cookie-query&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"cookie"</span> <a href="#sp">sp</a> <a href="#sp">sp</a>* <a href="#quoted-string">quoted-string</a></div></div>
<div class="rule">
  <div class="non-terminal" id="body-query">body-query&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"body"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="xpath-query">xpath-query&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"xpath"</span> <a href="#sp">sp</a> <a href="#sp">sp</a>* <a href="#quoted-string">quoted-string</a></div></div>
<div class="rule">
  <div class="non-terminal" id="jsonpath-query">jsonpath-query&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"jsonpath"</span> <a href="#sp">sp</a> <a href="#sp">sp</a>* <a href="#quoted-string">quoted-string</a></div></div>
<div class="rule">
  <div class="non-terminal" id="regex-query">regex-query&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"regex"</span> <a href="#sp">sp</a> <a href="#sp">sp</a>* <a href="#quoted-string">quoted-string</a></div></div>
<div class="rule">
  <div class="non-terminal" id="variable-query">variable-query&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"variable"</span> <a href="#sp">sp</a> <a href="#sp">sp</a>* <a href="#quoted-string">quoted-string</a></div></div>
<div class="rule">
  <div class="non-terminal" id="duration">duration&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"duration"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="sha256-query">sha256-query&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"sha256"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="bytes-query">bytes-query&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"bytes"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="subquery">subquery&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#regex-subquery">regex-subquery</a> | <a href="#count-subquery">count-subquery</a></div></div>
<div class="rule">
  <div class="non-terminal" id="regex-subquery">regex-subquery&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"regex"</span> <a href="#sp">sp</a> <a href="#sp">sp</a>* <a href="#quoted-string">quoted-string</a></div></div>
<div class="rule">
  <div class="non-terminal" id="count-subquery">count-subquery&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"count"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="assert">assert&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#lt">lt</a>*<br>&nbsp;
<a href="#sp">sp</a>* <a href="#query">query</a> <a href="#sp">sp</a> <a href="#sp">sp</a>* <a href="#predicate">predicate</a> <a href="#lt">lt</a></div></div>
<div class="rule">
  <div class="non-terminal" id="predicate">predicate&nbsp;</div>
  <div class="tokens">=&nbsp;(<span class="terminal">"not"</span> <a href="#sp">sp</a> <a href="#sp">sp</a>*)? <a href="#predicate-func">predicate-func</a></div></div>
<div class="rule">
  <div class="non-terminal" id="predicate-func">predicate-func&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#equal-predicate">equal-predicate</a><br>
| <a href="#not-equal-predicate">not-equal-predicate</a><br>
| <a href="#greater-predicate">greater-predicate</a><br>
| <a href="#greater-or-equal-predicate">greater-or-equal-predicate</a><br>
| <a href="#less-predicate">less-predicate</a><br>
| <a href="#less-or-equal-predicate">less-or-equal-predicate</a><br>
| <a href="#start-with-predicate">start-with-predicate</a><br>
| <a href="#end-with-predicate">end-with-predicate</a><br>
| <a href="#contain-predicate">contain-predicate</a><br>
| <a href="#match-predicate">match-predicate</a><br>
| <a href="#exist-predicate">exist-predicate</a><br>
| <a href="#include-predicate">include-predicate</a><br>
| <a href="#integer-predicate">integer-predicate</a><br>
| <a href="#float-predicate">float-predicate</a><br>
| <a href="#boolean-predicate">boolean-predicate</a><br>
| <a href="#string-predicate">string-predicate</a><br>
| <a href="#collection-predicate">collection-predicate</a></div></div>
<div class="rule">
  <div class="non-terminal" id="equal-predicate">equal-predicate&nbsp;</div>
  <div class="tokens">=&nbsp;(<span class="terminal">"equals"</span> | <span class="terminal">"=="</span>) <a href="#sp">sp</a> <a href="#sp">sp</a>* (<a href="#null">null</a> | <a href="#float">float</a> | <a href="#integer">integer</a> | <a href="#boolean">boolean</a> | <a href="#quoted-string">quoted-string</a> | <a href="#raw-string">raw-string</a> | <a href="#hex">hex</a> | <a href="#base64">base64</a> | <a href="#expr">expr</a>)</div></div>
<div class="rule">
  <div class="non-terminal" id="not-equal-predicate">not-equal-predicate&nbsp;</div>
  <div class="tokens">=&nbsp;(<span class="terminal">"notEquals"</span> | <span class="terminal">"!="</span>) <a href="#sp">sp</a> <a href="#sp">sp</a>* (<a href="#null">null</a> | <a href="#float">float</a> | <a href="#integer">integer</a> | <a href="#boolean">boolean</a> | <a href="#quoted-string">quoted-string</a> | <a href="#raw-string">raw-string</a> | <a href="#hex">hex</a> | <a href="#base64">base64</a> | <a href="#expr">expr</a>)</div></div>
<div class="rule">
  <div class="non-terminal" id="greater-predicate">greater-predicate&nbsp;</div>
  <div class="tokens">=&nbsp;(<span class="terminal">"greaterThan"</span> | <span class="terminal">">"</span>) <a href="#sp">sp</a> <a href="#sp">sp</a>* (<a href="#integer">integer</a> | <a href="#float">float</a>)</div></div>
<div class="rule">
  <div class="non-terminal" id="greater-or-equal-predicate">greater-or-equal-predicate&nbsp;</div>
  <div class="tokens">=&nbsp;(<span class="terminal">"greaterThanOrEquals"</span> | <span class="terminal">">="</span>) <a href="#sp">sp</a> <a href="#sp">sp</a>* (<a href="#integer">integer</a> | <a href="#float">float</a>)</div></div>
<div class="rule">
  <div class="non-terminal" id="less-predicate">less-predicate&nbsp;</div>
  <div class="tokens">=&nbsp;(<span class="terminal">"lessThan"</span> | <span class="terminal">"<"</span>) <a href="#sp">sp</a> <a href="#sp">sp</a>* (<a href="#integer">integer</a> | <a href="#float">float</a>)</div></div>
<div class="rule">
  <div class="non-terminal" id="less-or-equal-predicate">less-or-equal-predicate&nbsp;</div>
  <div class="tokens">=&nbsp;(<span class="terminal">"lessThanOrEquals"</span> | <span class="terminal">"<="</span>) <a href="#sp">sp</a> <a href="#sp">sp</a>* (<a href="#integer">integer</a> | <a href="#float">float</a>)</div></div>
<div class="rule">
  <div class="non-terminal" id="start-with-predicate">start-with-predicate&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"startsWith"</span> <a href="#sp">sp</a> <a href="#sp">sp</a>* (<a href="#quoted-string">quoted-string</a> | <a href="#hex">hex</a> | <a href="#base64">base64</a>)</div></div>
<div class="rule">
  <div class="non-terminal" id="end-with-predicate">end-with-predicate&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"endsWith"</span> <a href="#sp">sp</a> <a href="#sp">sp</a>* (<a href="#quoted-string">quoted-string</a> | <a href="#hex">hex</a> | <a href="#base64">base64</a>)</div></div>
<div class="rule">
  <div class="non-terminal" id="contain-predicate">contain-predicate&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"contains"</span> <a href="#sp">sp</a> <a href="#sp">sp</a>* <a href="#quoted-string">quoted-string</a></div></div>
<div class="rule">
  <div class="non-terminal" id="match-predicate">match-predicate&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"matches"</span> <a href="#sp">sp</a> <a href="#sp">sp</a>* <a href="#quoted-string">quoted-string</a></div></div>
<div class="rule">
  <div class="non-terminal" id="exist-predicate">exist-predicate&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"exists"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="include-predicate">include-predicate&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"includes"</span> <a href="#sp">sp</a> <a href="#sp">sp</a>* (<a href="#null">null</a> |<a href="#float">float</a> | <a href="#integer">integer</a> | <a href="#boolean">boolean</a> | <a href="#quoted-string">quoted-string</a> | <a href="#expr">expr</a>)</div></div>
<div class="rule">
  <div class="non-terminal" id="integer-predicate">integer-predicate&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"isInteger"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="float-predicate">float-predicate&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"isFloat"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="boolean-predicate">boolean-predicate&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"isBoolean"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="string-predicate">string-predicate&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"isString"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="collection-predicate">collection-predicate&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"isCollection"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="key-value">key-value&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#key-string">key-string</a> <a href="#sp">sp</a>* <span class="terminal">":"</span> <a href="#sp">sp</a>* <a href="#value-string">value-string</a></div></div>
<div class="rule">
  <div class="non-terminal" id="key-string">key-string&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="definition">&lt;(alphanum | "_" | "-" | "." | escape-char)+ &gt;</span></div></div>
<div class="rule">
  <div class="non-terminal" id="value-string">value-string&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="definition">&lt;(anychar except escaped char and #| escape-char)* without leading/trailing space&gt;</span></div></div>
<div class="rule">
  <div class="non-terminal" id="quoted-string">quoted-string&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"""</span> <span class="definition">&lt;(anychar except escaped char | escape-char)*&gt;</span> <span class="terminal">"""</span></div></div>
<div class="rule">
  <div class="non-terminal" id="expr">expr&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"{{"</span> <a href="#sp">sp</a>* <a href="#variable-name">variable-name</a> <a href="#sp">sp</a>* <span class="terminal">"}}"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="variable-name">variable-name&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="definition">&lt;(alphanum | "_" )+&gt;</span></div></div>
<div class="rule">
  <div class="non-terminal" id="escaped-char">escaped-char&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"\""</span><br>
| <span class="terminal">"\\"</span><br>
| <span class="terminal">"\b"</span><br>
| <span class="terminal">"\f"</span><br>
| <span class="terminal">"\n"</span><br>
| <span class="terminal">"\r"</span><br>
| <span class="terminal">"\t"</span><br>
| <span class="terminal">"\u"</span> <a href="#unicode-char">unicode-char</a></div></div>
<div class="rule">
  <div class="non-terminal" id="unicode-char">unicode-char&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"{"</span> <a href="#hexdigit">hexdigit</a>+ <span class="terminal">"}"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="bytes">bytes&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#json">json</a><br>
| <a href="#xml">xml</a><br>
| <a href="#raw-string">raw-string</a><br>
| <a href="#base64">base64</a><br>
| <a href="#file">file</a><br>
| <a href="#hex">hex</a></div></div>
<div class="rule">
  <div class="non-terminal" id="raw-string">raw-string&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"```"</span> (<a href="#sp">sp</a>* <a href="#newline">newline</a>)? (<a href="#any">any</a> <a href="#characters">characters</a>) <span class="terminal">"```"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="base64">base64&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"base64,"</span> <span class="definition">&lt;base64 encoding with optional whitesp/padding&gt;</span> <span class="terminal">";"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="file">file&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"file,"</span> <a href="#sp">sp</a>* <a href="#filename">filename</a> <a href="#sp">sp</a>* <span class="terminal">";"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="hex">hex&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"hex,"</span> <a href="#sp">sp</a>* <a href="#hexdigit">hexdigit</a>* <a href="#sp">sp</a>* <span class="terminal">";"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="lt">lt&nbsp;</div>
  <div class="tokens">=&nbsp;<a href="#sp">sp</a>* <a href="#comment">comment</a>? (<a href="#newline">newline</a> | <a href="#eof">eof</a>)</div></div>
<div class="rule">
  <div class="non-terminal" id="comment">comment&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"#"</span> <span class="definition">&lt;any characters except newline - does not end with sp&gt;</span></div></div>
<div class="rule">
  <div class="non-terminal" id="newline">newline&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">"\n"</span> | <span class="terminal">"\r\n"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="sp">sp&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="terminal">" "</span> | <span class="terminal">"\t"</span></div></div>
<div class="rule">
  <div class="non-terminal" id="filename">filename&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="definition">&lt;(alphanum | ".")+&gt;</span></div></div>
<div class="rule">
  <div class="non-terminal" id="integer">integer&nbsp;</div>
  <div class="tokens">=&nbsp;<span class="definition">&lt;-?[1-9][0-9]*&gt;</span></div></div>

</div>

