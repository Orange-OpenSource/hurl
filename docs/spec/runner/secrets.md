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
  - ~~Environment variable prefixed by `HURLSECRET_`~~
  - ~~`[Options]` section~~

<strike>

```hurl
GET https://{{host}}/{{id}}/status
[Options]
secret: host=example.net
secret: id=1234
HTTP 304

GET https://{{host}}/health
HTTP 200
```

</strike>

> [!NOTE]
> What happens if we define a secret, and declared it afterward as a variable?
> ```shell
> $ hurl --secret foo=toto test.hurl
> ```
> `test.hurl` being:
> ```hurl
>  GET https://sample.com
>  [Options]
>  variable foo=tata
> ```

<strike>

- Using a specific pattern in variable value and do not introduce any options
    - Command line for a single variable `hurl --variable host=SECRET(example.net) --variable id=SECRET(1234) test.hurl`
    - Command line for a variables file `hurl --variables-file vars.env test.hurl` where `vars.env` is the following file:
    ```
    host=SECRET(example.net)
    id=SECRET(1234)
    ```
    - Question: how do we make a literal "public" variable `SECRET(foo)`

</strike>

Injecting secret and variable with the same name must lead to an error:

```shell
$ hurl --variable foo=toto --secret foo=tutu /tmp/test.hurl
error: the variable 'foo' cannot be public and private at the same time
```


## Implementation

Options defined at the CLI are represented by [`CliOptions` struct](https://github.com/Orange-OpenSource/hurl/blob/master/packages/hurl/src/cli/options/mod.rs)

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliOptions {
    // ...
    pub variables: HashMap<String, Value>,
    // ...
}
```

Throughout the code, we're using an `HashMap` for owning variables. Variables are just Hurl `Value`. The public API 
for running a Hurl sample is :

```rust
pub fn run(
    content: &str,
    filename: Option<&Input>,
    runner_options: &RunnerOptions,
    variables: &HashMap<String, Value>,
    logger_options: &LoggerOptions,
) -> Result<HurlResult, String> {
    // ...
}
```

With secret, we'll neet to distinguish if a variable is public or private. 

Proposition:

- introduce a proper type `Variable` that holds a `String` name, a `Value` value and a variable kind (`public` or `private`) 
- introduce a proper `VariableSet` that have the same interface as `HashMap`, for the moment.