# Secret Variables Management

## Secret definition

Secret are user provided variables whose value is never written in:

- standard error (verbose logs included curl logs, asserts error etc...)
- various report debug logs (for instance headers in `--report-html`)

> [!NOTE]
> Do we need to protect standard output? If we define a secret with `--secret foo=bar` and that
> the HTTP response is `{"value": "bar"}`, do we output {"value": "xxx"} 


## Injecting variables

As of Hurl 5.0.1, the way to define variables in Hurl are:

- Command line for a single variable `hurl --variable host=example.net --variable id=1234 test.hurl`
- Command line for a variables file `hurl --variables-file vars.env test.hurl` where `vars.env` is the following file:

    ```
    host=example.net
    id=1234
    ```

- Environment variable prefixed by `HURL_`
   export HURL_host=example.net
   export HURL_id=1234
   hurl test.hurl
- `[Options]` section, inside a Hurl file

    ```hurl
    GET https://{{host}}/{{id}}/status
    [Options]
    variable: host=example.net
    variable: id=1234
    HTTP 304
    
    GET https://{{host}}/health
    HTTP 200
    ```

## Injecting secrets

Some ideas:

- Duplicating command line / Hurl syntax for secret with `--secret`, `--secrets-variable`
  - Command line for a single variable `hurl --secret host=example.net --secret id=1234 test.hurl`
  - Command line for a variables file `hurl --secrets-file vars.env test.hurl`
  - Environment variable prefixed by `HURLSECRET_`
  - `[Options]` section

    ```hurl
    GET https://{{host}}/{{id}}/status
    [Options]
    secret: host=example.net
    secret: id=1234
    HTTP 304
    
    GET https://{{host}}/health
    HTTP 200
    ```
- Using a specific pattern in variable value and do not introduce any options
    - Command line for a single variable `hurl --variable host=SECRET(example.net) --variable id=SECRET(1234) test.hurl`
    - Command line for a variables file `hurl --variables-file vars.env test.hurl` where `vars.env` is the following file:
    ```
    host=SECRET(example.net)
    id=SECRET(1234)
    ```
    - Question: how do we make a literal "public" variable `SECRET(foo)`

