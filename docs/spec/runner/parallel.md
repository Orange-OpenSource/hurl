# Supporting Parallel Execution of Hurl Files

## Usage

### Default Run

```mermaid
stateDiagram-v2
    state "hurl *.hurl" as [*]
    state "a.hurl" as A
    state "b.hurl" as B
    state "c.hurl" as C
    state fork <<fork>>
    state join <<join>>
    direction LR
    [*] --> fork
    fork --> A
    A --> B
    B --> C
    C --> join
    join --> [*]
```

### Parallel Run

```mermaid
stateDiagram-v2
    state "hurl --parallel *.hurl" as [*]
    state "a.hurl" as A
    state "b.hurl" as B
    state "c.hurl" as C
    state fork <<fork>>
    state join <<join>>
    direction LR
    [*] --> fork
    fork --> A
    fork --> B
    fork --> C
    A --> join
    B --> join
    C --> join
    join --> [*]
```


## Related Issues 

[1139 - how can send bulk request](https://github.com/Orange-OpenSource/hurl/issues/1139)

[88 - add the --concurrency option to launch multiple runs of *.hurl files instead of one](https://github.com/Orange-OpenSource/hurl/issues/88)

[87 - add the --parallel option to run *.hurl files in parallel instead of sequentially](https://github.com/Orange-OpenSource/hurl/issues/87)

## Related Options

- `--repeat` TBD
- `--repeat-all` TBD
- ...

## Console Output

We need to defined what will be the output on stdout/stderr when running in parallel. We can take inspiration of 
example of parallel execution from cargo issue [Console output should better reflect the build process](https://github.com/rust-lang/cargo/issues/8889)

![buck build](https://user-images.githubusercontent.com/1940490/108307902-9dea2180-7163-11eb-9a4d-269d68d40d9f.gif)

![bazel build](https://user-images.githubusercontent.com/1940490/108307921-a7738980-7163-11eb-80c0-4844d55a4390.gif)

![https://asciinema.org/a/nMUGH5T2PiizxwK340n0DTW4M](https://camo.githubusercontent.com/fa245e2401ab21b30aa76cd07f877181a9a07be9be83d748f6c12d248e1024c0/68747470733a2f2f61736369696e656d612e6f72672f612f6e4d5547483554325069697a78774b3334306e30445457344d2e737667)

TODO: make asciinema for different options. 


## Tools 

### wrk2 

[wrk2](https://github.com/giltene/wrk2), a HTTP benchmarking tool based mostly on wrk.

### GNU parallel

[GNU parallel](https://www.gnu.org/software/parallel/)

From [#87]():

```shell
$ parallel -j $(ls -1 *.hurl | wc -l) -i sh -c "hurl {} --test" -- *.hurl
$ echo "retval: $?"
```

## Backlog

- What options do we expose?
- Visualization? How do we report the timelines of Hurl files execution
- How to test? Support // in Flask?
- stderr / verbose report: do we prefix log lines by thread id / index ? We could make the debug logs identical whether
files are run sequentially or run in parallel.
- Does the user set "thread" affinity in Hurl files? (see https://github.com/Orange-OpenSource/hurl/issues/88#issuecomment-1674518247)
- What's the Rust runner API? Actually, we only expose on public method to run an Hurl file, we need(?) to expose methods
to runs multiple files


