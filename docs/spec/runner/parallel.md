# Parallel Execution Design Document

## Run Execution Diagram

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

```shell
$ hurl --test a.hurl b.hurl c.hurl
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

```shell
$ hurl --parallel --test a.hurl b.hurl c.hurl
```

Reuse `--jobs` option from [GNU Parallel] to specify the number of threads.

```shell
$ hurl --parallel --jobs 4 --test a.hurl b.hurl c.hurl
```

> `--jobs 0` will run as many jobs in parallel as possible.



## State of the Art / Tools

### GNU Parallel

[GNU Parallel] buffers stdout/stderr and postpones the command until the command completes. So the command outputs as soon
as it completes, not necessary in the same order:

```shell
$ parallel echo ::: A B C D
A
C
B
D
$ parallel echo ::: A B C D
B
A
C
D
```

With Hurl:

```shell
$ parallel hurl ::: a.hurl b.hurl c.hurl d.hurl
ABCD%
$ parallel hurl ::: a.hurl b.hurl c.hurl d.hurl
BACD%
```

> The last test has been executed with the Flask instance. If we block in the /a endpoint, we have the direct response
> from /b, /c, /d and then /a. By default, Flask can handle concurrent requests with thread.

Regarding stderr, we can see that stdout is flush, then stderr. In the next test, we have the response before the stderr:


```shell
$ parallel hurl --verbose ::: a.hurl b.hurl c.hurl d.hurl
A* ------------------------------------------------------------------------------
* Executing entry 1
*
* Cookie store:
*
* Request:
* GET http://localhost:8000/a
*
...
< Connection: close
<
*
B* ------------------------------------------------------------------------------
* Executing entry 1
*
* Cookie store:
*
* Request:
* GET http://localhost:8000/b
*
...
< Connection: close
<
*
C* ------------------------------------------------------------------------------
* Executing entry 1
*
* Cookie store:
*
* Request:
* GET http://localhost:8000/c
*
...
< Connection: close
<
*
D* ------------------------------------------------------------------------------
* Executing entry 1
*
* Cookie store:
*
* Request:
* GET http://localhost:8000/d
*
...
< Connection: close
<
*
```

#### Interesting option

- `--tag`: add the parameter value before each call:

```shell
$ parallel --tag echo ::: A B C D
A	A
B	B
C	C
D	D
```

```shell
$ parallel --tag hurl ::: a.hurl b.hurl c.hurl d.hurl
a.hurl	Ab.hurl	Bc.hurl	Cd.hurl	D%
```

The tag value is configurable.

- `keep-order/-k`: force GNU Parallel to print in the order of values, the commands are still run in parallel.

```shell
$ parallel sleep {}';' echo {} done ::: 5 4 3 2 1
1 done
2 done
3 done
4 done
5 done
```

#### From Hurl issues/discussions:

From [#87]():

```shell
$ parallel -j $(ls -1 *.hurl | wc -l) -i sh -c "hurl {} --test" -- *.hurl
$ echo "retval: $?"
```

### wrk2

[wrk2](https://github.com/giltene/wrk2), a HTTP benchmarking tool based mostly on wrk.

## How to Test

Flask `run` method [takes a `threaded` option] to handle concurrent requests using thread or not (`True` by default).  






## Related Issues 

[1139 - how can send bulk request](https://github.com/Orange-OpenSource/hurl/issues/1139)

[88 - add the --concurrency option to launch multiple runs of *.hurl files instead of one](https://github.com/Orange-OpenSource/hurl/issues/88)

[87 - add the --parallel option to run *.hurl files in parallel instead of sequentially](https://github.com/Orange-OpenSource/hurl/issues/87)

## Related Options

- `--repeat` TBD
- `--repeat-all` TBD
- ...

## Console Output

[Console output should better reflect the build process](https://github.com/rust-lang/cargo/issues/8889)

![buck build](https://user-images.githubusercontent.com/1940490/108307902-9dea2180-7163-11eb-9a4d-269d68d40d9f.gif)

![bazel build](https://user-images.githubusercontent.com/1940490/108307921-a7738980-7163-11eb-80c0-4844d55a4390.gif)

![https://asciinema.org/a/nMUGH5T2PiizxwK340n0DTW4M](https://camo.githubusercontent.com/fa245e2401ab21b30aa76cd07f877181a9a07be9be83d748f6c12d248e1024c0/68747470733a2f2f61736369696e656d612e6f72672f612f6e4d5547483554325069697a78774b3334306e30445457344d2e737667)

TODO: make asciinema for different options. 


## Backlog

- What options do we expose?
- Visualization? How do we report the timelines of Hurl files execution
- How to test? Support // in Flask?
- stderr / verbose report: do we prefix log lines by thread id / index ? We could make the debug logs identical whether
files are run sequentially or run in parallel.
- Does the user set "thread" affinity in Hurl files? (see https://github.com/Orange-OpenSource/hurl/issues/88#issuecomment-1674518247)
- What's the Rust runner API? Actually, we only expose on public method to run an Hurl file, we need(?) to expose methods
to runs multiple files


[takes a `threaded` option]: https://werkzeug.palletsprojects.com/en/3.0.x/serving/#werkzeug.serving.run_simple
[GNU Parallel]: https://www.gnu.org/software/parallel/

