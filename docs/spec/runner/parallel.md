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
$ hurl --parallel --max-workers 4 --test a.hurl b.hurl c.hurl
```

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

- `--keep-order/-k`: force GNU Parallel to print in the order of values, the commands are still run in parallel.

```shell
$ parallel sleep {}';' echo {} done ::: 5 4 3 2 1
1 done
2 done
3 done
4 done
5 done
```

> From [Hurl issues #87]():
> 
> ```shell
> $ parallel -j $(ls -1 *.hurl | wc -l) -i sh -c "hurl {} --test" -- *.hurl
> $ echo "retval: $?"
> ```

### wrk2

[wrk2](https://github.com/giltene/wrk2), a HTTP benchmarking tool based mostly on wrk.

## --test Output

Demo here => <https://jcamiel.github.io/parallel/>

### Hurl 4.2.0 sync run

```shell
$ hurl --test *.hurl
/tmp/foo/bar/baz/job-1.hurl: Running [1/10]
/tmp/foo/bar/baz/job-1.hurl: Success (10 request(s) in 10096 ms)
/tmp/foo/bar/job-2.hurl: Running [2/10]
/tmp/foo/bar/job-2.hurl: Success (2 request(s) in 3019 ms)
/tmp/foo/bar/zzzzzz/job-3.hurl: Running [3/10]
 [========>               ] 2/3
```

### Hurl x.x.x parallel 

- 5 workers:
 
```shell
$ hurl --parallel --max-workers 5
/tmp/foo/bar/job-2.hurl: Success (2 request(s) in x ms)
/tmp/foo/bar/job-6.hurl: Success (4 request(s) in x ms)
/tmp/foo/bar/job-4.hurl: Success (7 request(s) in x ms)
Executed files: 3/10 (30%)
[=========>              ] 5/10  /tmp/foo/bar/baz/job-1.hurl: Running
[>                       ] 1/1   /tmp/foo/bar/ee/job-7.hurl: Running
[================>       ] 3/3   /tmp/foo/bar/zzzzzz/job-3.hurl: Running
[>                       ] 1/5   /tmp/foo/bar/fff/job-8.hurl: Running
[==============>         ] 8/12  /tmp/foo/bar/ddd/job-5.hurl: Running
```

- 1 worker:

```shell
$ hurl --parallel --max-workers 1
/tmp/foo/bar/baz/job-1.hurl: Success (10 request(s) in x ms)
/tmp/foo/bar/job-2.hurl: Success (2 request(s) in x ms)
/tmp/foo/bar/zzzzzz/job-3.hurl: Success (3 request(s) in x ms)
/tmp/foo/bar/job-4.hurl: Success (7 request(s) in x ms)
Executed files: 4/10 (40%)
[========>               ] 5/12  /tmp/foo/bar/ddd/job-5.hurl: Running
```

When the number of running jobs is more than x (TBD), we display x progress bars and '...y more'

```shell
$ hurl --parallel --max-workers 60
/tmp/foo/bar/job-2.hurl: Success (2 request(s) in x ms)
/tmp/foo/bar/job-6.hurl: Success (4 request(s) in x ms)
/tmp/foo/bar/job-4.hurl: Success (7 request(s) in x ms)
Executed files: 3/100 (3%)
[=========>              ] 5/10  /tmp/foo/bar/baz/job-1.hurl: Running
[>                       ] 1/1   /tmp/foo/bar/ee/job-7.hurl: Running
[================>       ] 3/3   /tmp/foo/bar/zzzzzz/job-3.hurl: Running
[>                       ] 1/5   /tmp/foo/bar/fff/job-8.hurl: Running
[==============>         ] 8/12  /tmp/foo/bar/ddd/job-5.hurl: Running
...55 more
```

To reflect the current Hurl 4.2.0 temporal behaviour, when a job is done, we should output stderr and stdout (in this order).

## Exposed options

- `--parallel`: use parallel runner (should be the default in the future)
- `--max-workers 8`: limit the number of parallel jobs
- `--repeat`? `--repeat-all`?
- `--keep-order`: from GNU Parallel, useful for testing parallel reunner, or for getting response from a set of Hurl files.

## How to Test

Flask `run` method [takes a `threaded` option] to handle concurrent requests using thread or not (`True` by default).  

## Related Issues 

[1139 - how can send bulk request](https://github.com/Orange-OpenSource/hurl/issues/1139)

[88 - add the --concurrency option to launch multiple runs of *.hurl files instead of one](https://github.com/Orange-OpenSource/hurl/issues/88)

[87 - add the --parallel option to run *.hurl files in parallel instead of sequentially](https://github.com/Orange-OpenSource/hurl/issues/87)

- `--repeat` TBD
- `--repeat-all` TBD
- ...

## Console Output of Others Programs

[Console output should better reflect the build process](https://github.com/rust-lang/cargo/issues/8889)

![buck build](https://user-images.githubusercontent.com/1940490/108307902-9dea2180-7163-11eb-9a4d-269d68d40d9f.gif)

![bazel build](https://user-images.githubusercontent.com/1940490/108307921-a7738980-7163-11eb-80c0-4844d55a4390.gif)

![https://asciinema.org/a/nMUGH5T2PiizxwK340n0DTW4M](https://camo.githubusercontent.com/fa245e2401ab21b30aa76cd07f877181a9a07be9be83d748f6c12d248e1024c0/68747470733a2f2f61736369696e656d612e6f72672f612f6e4d5547483554325069697a78774b3334306e30445457344d2e737667)

TODO: make asciinema for different options. 


## Backlog

- Visualization? How do we report the timelines of Hurl files execution
- How to test? Support // in Flask?
- What's the Rust runner API? Actually, we only expose on public method to run an Hurl file, we need(?) to expose methods
to runs multiple files


[takes a `threaded` option]: https://werkzeug.palletsprojects.com/en/3.0.x/serving/#werkzeug.serving.run_simple
[GNU Parallel]: https://www.gnu.org/software/parallel/

