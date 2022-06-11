# Your First Hurl File

Throughout this tutorial, we'll walk through the creation of multiple
Hurl files to test a basic quiz application. We'll show how to test
this site locally, and how to automate these integration tests in a CI/CD
chain like [GitHub Action] and [GitLab CI/CD].

The quiz application consists of:

- a website that lets people create or play a series of quizzes
- a set of REST apis to list, create and delete question and quiz

With Hurl, we're going to add tests for the website and the apis.

## Prerequisites

We’ll assume you have Hurl installed already. You can test it by running the
following command in a shell prompt (indicated by the $ prefix):

```shell
$ hurl --version
```

If Hurl is already installed, you should see the version of Hurl. If it isn't, you
can check [Installation] to see how to install Hurl.

Next, we’re going to install our quiz application locally, in order to test it. We are
not going to build our application from scratch, in order to focus on how to test it.

> Hurl being really language agnostic, you can use it to validate any type of application: in
> this tutorial, our quiz application is built with [Spring Boot],
> but this could as well be a [Node.js] or a [Flask] app.

Our quiz application can be launched locally either:

- using a Docker image
- directly using the jar of the application

If you want to use the Docker image, you must have Docker installed locally. If it is the case,
just run in a shell:

```shell
$ docker pull ghcr.io/jcamiel/quiz:latest
$ docker run --name quiz --rm --detach --publish 8080:8080 ghcr.io/jcamiel/quiz:latest
```

And check that the container is running with:

```shell
$ docker ps
CONTAINER ID   IMAGE                         COMMAND                  CREATED         STATUS         PORTS                                       NAMES
922d387923ec   ghcr.io/jcamiel/quiz:latest   "java -jar app/quiz.…"   8 seconds ago   Up 6 seconds   0.0.0.0:8080->8080/tcp, :::8080->8080/tcp   quiz
```

If you want to use the jar application, you must have Java installed locally. If it is the case, download
the jar application from <https://github.com/jcamiel/quiz/releases/latest> and run in a shell:

```shell
$ java -jar quiz-0.0.2.jar 
```

Either you're using the Docker images ot the jar app, you can open a browser and test the quiz application by
typing the url <http://localhost:8080>:

<div>
     <img class="light-img" src="/docs/assets/img/quiz-light.png" width="400px" alt="Quiz home page"/>
     <img class="dark-img" src="/docs/assets/img/quiz-dark.png" width="400px" alt="Quiz home page"/>
</div>

<small>Our quiz app: we've only secured a budget for integration tests and nothing for the site design...</small>

## A Basic Test

Next, we’re going to write our first test.

1. Open a text editor and create a file named `basic.hurl`. In this file, just type the following text and save:

```hurl
GET http://localhost:8080
```

This is your first Hurl file, and probably one of the simplest. It consists of only one [entry].

> An entry has a mandatory [request specification]: in this case, we want to perform a
> `GET` HTTP request on the endpoint <http://localhost:8080>. A request can be optionally followed by a [response
> description], to add asserts on the HTTP response. For the moment, we don't have any response description.

2. In a shell, execute `hurl` with `basic.hurl` as argument:

```shell
$ hurl basic.hurl
<!doctype html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Welcome to Quiz!</title>
<!--    <link rel="stylesheet" href="style.css">
    <script src="script.js"></script>-->
</head>
....
</html>
```

If the quiz app is running, you should see the content of the html file at <http://localhost:8080>. If the quiz app
is not running, you'll see an error:

```shell
$ hurl basic.hurl 
error: Http Connection
  --> basic.hurl:1:5
   |
 1 | GET http://localhost:8080
   |     ^^^^^^^^^^^^^^^^^^^^^ Fail to connect
   |
```


As there are no response description, this basic test only checks that an HTTP server is running at
<http://localhost:8080> and responds _something_. If the server had a problem on this endpoint, and had responded
with a [`500 Internal Server Error`], Hurl would have just executed successfully the HTTP request,
without checking the actual HTTP response.

As this test is not sufficient to ensure that our server is alive and running, we're going to add some asserts on
the response and, at least, check that the HTTP response status code is [`200 OK`].

3. Open `basic.hurl` and modify it to test the status code response:

```hurl
GET http://localhost:8080
HTTP/1.1 200
```

4. Execute `basic.hurl`:

```shell
$ hurl basic.hurl
<!doctype html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Welcome to Quiz!</title>
    <link rel="stylesheet" href="style.css">
    <script src="script.js"></script>
</head>
....
</html>
```

There is no modification to the output of Hurl, the content of the HTTP request is outputted to the terminal. But, now,
we check that our server is responding with a `200 OK`. 

By default, Hurl behaves like [curl] and outputs the HTTP response. This is useful when you want to get data from a 
server, and you need to perform additional steps (like login, confirmation etc...) before being able to call your last
request.

In our case, we want to add tests to our project, so we can use [`--test`] command line option to have an adapted 
test output:

5. Execute `basic.hurl` in test mode:

```shell
$ hurl --test basic.hurl
basic.hurl: RUNNING [1/1]
basic.hurl: SUCCESS
--------------------------------------------------------------------------------
Executed:  1
Succeeded: 1 (100.0%)
Failed:    0 (0.0%)
Duration:  14ms
```

6. Modify `basic.hurl` to test a different HTTP response status code:

```hurl
GET http://localhost:8080
HTTP/1.1 500
```

7. Save and execute it:


```shell
$ hurl --test basic.hurl
error: Assert Status
  --> basic.hurl:2:10
   |
 2 | HTTP/1.1 500
   |          ^^^ actual value is <200>
   |
```

8. Revert your changes and finally add a comment at the beginning of the file:


```hurl
# Our first Hurl file, just checking
# that our server is up and running.
GET http://localhost:8080
HTTP/1.1 200
```

## Recap

That's it, this is your first Hurl file!

This is really a basic test, but Hurl's file format strength is its simplicity.
We're going to see in the next section how to improve our tests while keeping it really simple.

[GitHub Action]: https://github.com/features/actions
[GitLab CI/CD]: https://docs.gitlab.com/ee/ci/
[Installation]: /docs/installation.md
[Spring Boot]: https://spring.io/projects/spring-boot
[Node.js]: https://nodejs.org/en/
[Flask]: https://flask.palletsprojects.com
[entry]: /docs/entry.md
[request specification]: /docs/request.md
[response description]: /docs/response.md
[`500 Internal Server Error`]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500
[`200 OK`]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/200
[`--test`]: /docs/man-page.md#test
[curl]: https://curl.se