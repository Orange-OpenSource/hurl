# CI/CD Integration

Up until now, we have run our test files locally. Now, we want to integrate
them in a CI/CD pipeline (like [GitHub Actions] or [GitLab CI/CD pipelines]). As
Hurl is very fast, we're going to run our tests on each commit of our project,
drastically improving the project quality.

A typical web project pipeline is:

- build the application, run unit tests and static code analysis,
- publish the application image to a Docker registry,
- pull the application image and run integration tests.

In this workflow, we're testing the same image that will be used and deployed in
production.

> For the tutorial, we are skipping build and publication phases and
> only run integration tests on a prebuilt Docker image. To check a complete
> project with build, Docker upload/publish and integration tests, go to <https://github.com/jcamiel/hurl-express-tutorial>

In a first step, we're going to write a bash script that will pull our Docker
image, launch it and run Hurl tests against it. Once we have checked that this
script runs locally, we'll see how to run it automatically in a CI/CD pipeline.

## Integration Script

1. First, create a directory name `movies-project`, add [`integration/basic.hurl`]
   and [`integration/create-quiz.hurl`] from the previous tutorial to the directory.

<pre><code class="language-shell">$ mkdir movies-project
$ cd movies-project
$ mkdir integration
$ vi integration/basic.hurl

# Import <a href="https://github.com/jcamiel/hurl-express-tutorial/raw/main/integration/basic.hurl">basic.hurl</a> here!

$ vi integration/login.hurl

# Import <a href="https://github.com/jcamiel/hurl-express-tutorial/raw/main/integration/login.hurl">login.hurl</a> here!</code></pre>

Next, we are going to write the first version of our integration script that will
just pull the Quiz image and run it:

2. Create a script named `bin/integration.sh` with the following content:

```bash
#!/bin/bash
set -eu

echo "Starting container"
docker run --name movies --rm --detach --publish 3000:3000 ghcr.io/jcamiel/hurl-express-tutorial:latest
```

3. Make the script executable and run it:

```shell
$ chmod u+x bin/integration.sh
$ bin/integration.sh
Starting container
5d311561828d6078e84eb4b8b87dfd5d67bde6d9614ad83860b60cf310438d2a
```

4. Verify that our container is up and running, and stop it.

```shell
$ docker ps
CONTAINER ID   IMAGE                                          COMMAND                  CREATED         STATUS         PORTS                                       NAMES
4002ce42e507   ghcr.io/jcamiel/hurl-express-tutorial:latest   "node dist/bin/www.js"   3 seconds ago   Up 2 seconds   0.0.0.0:3000->3000/tcp, :::3000->3000/tcp   movies
$ docker stop movies
movies
```

Now, we have a basic script that starts our container. Before adding our
integration tests, we need to ensure that our application server is ready: the
container has started, but the application server can take a few seconds to be
really ready to accept incoming HTTP requests.

To do so, we can test our health API. With a function `wait_for_url`,
we use Hurl to check a given URL to return a `200 OK`. We loop on this function
until the check succeeds with [`--retry`] Hurl option. Once the test has succeeded, we stop the container.

5. Modify `bin/integration.sh` to wait for the application to be ready:

```bash
#!/bin/bash
set -eu

wait_for_url () {
    echo "Testing $1..."
    echo -e "GET $1\nHTTP 200" | hurl --retry "$2" > /dev/null;
    return 0
}

echo "Starting container"
docker run --name movies --rm --detach --publish 3000:3000 ghcr.io/jcamiel/hurl-express-tutorial:latest

echo "Waiting server to be ready"
wait_for_url 'http://localhost:3000' 60

echo "Stopping container"
docker stop movies
```

We have now the simplest integration test script: it pulls our Docker image, then starts
the container and waits for a `200 OK` response.

Next, we're going to add our Hurl tests to the script.

6. Modify `bin/integration.sh` to add integration tests:

```bash
#!/bin/bash
set -eu

# ...

echo "Starting container"
# ...

echo "Waiting server to be ready"
# ...

echo "Running Hurl tests"
hurl --test integration/*.hurl

echo "Stopping container"
# ...
```

7. Run [`bin/integration.sh`] to check that our application passes all tests:

```shell
$ bin/integration.sh
Starting container
48cf21d193a01651fc42b80648abdb51dc626f31c3f9c8917aea899c68eb4a12
Waiting server to be ready
Testing http://localhost:3000
Running Hurl tests
[1mintegration/basic.hurl[0m: [1;36mRunning[0m [1/2]
[1mintegration/basic.hurl[0m: [1;32mSuccess[0m (4 request(s) in 18 ms)
[1mintegration/login.hurl[0m: [1;36mRunning[0m [2/2]
[1mintegration/login.hurl[0m: [1;32mSuccess[0m (6 request(s) in 18 ms)
--------------------------------------------------------------------------------
Executed files:  2
Succeeded files: 2 (100.0%)
Failed files:    0 (0.0%)
Duration:        48 ms

Stopping container
movies
```

Locally, our test suite is now fully functional. As Hurl is very fast, we can use
it to ensure that new developments don't have regression. Our next step is to run
the integration tests automatically in a CI/CD pipeline. As an example, we're going
to create a [GitHub Action]. You can also see how to integrate your tests in [GitLab CI/CD here].

## Running Tests with GitHub Action

1. Create a new empty repository in GitHub, named `movies-project`:

<div class="picture">
    <img class="light-img u-drop-shadow u-border u-max-width-100" src="/docs/assets/img/github-new-repository-light.png" width="680" alt="Create new GitHub repository"/>
    <img class="dark-img u-drop-shadow u-border u-max-width-100" src="/docs/assets/img/github-new-repository-dark.png" width="680" alt="Create new GitHub repository"/>
</div>


2. On your computer, create a git repo in `movies-project` directory and
   commit the projects files:

```shell
$ git init
Initialized empty Git repository in /Users/jc/Documents/Dev/movies-project/.git/
$ git add .
$ git commit -m "Add integration tests."
[master (root-commit) ea3e5cd] Add integration tests.
 3 files changed, 146 insertions(+)
 create mode 100755 bin/integration.sh
...
$ git remote add origin https://github.com/jcamiel/movies-project.git
$ git push -u origin main
Enumerating objects: 7, done.
Counting objects: 100% (7/7), done.
...
```

Next, we are going to add a GitHub Action to our repo. The purpose of this action
will be to launch our integration script on each commit.

3. Create a  file in `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches:
      - main

jobs:
  build:
    runs-on: ubuntu-latest
    permissions:
      contents: read
    steps:
      - name: Checkout
        uses: actions/checkout@v3
      - name: Build
        run: echo "Building app..."
      - name: Integration test
        run: |
          curl --location --remote-name https://github.com/Orange-OpenSource/hurl/releases/download/4.0.0/hurl_4.0.0_amd64.deb
          sudo dpkg -i hurl_4.0.0_amd64.deb
          bin/integration.sh
```

4. Commit and push the new action:

```shell
$ git add .github/workflows/ci.yml
$ git commit -m "Add GitHub action."
[main 077d754] Add GitHub action.
 1 file changed, 19 insertions(+)
 ...
$ git push
Enumerating objects: 6, done.
Counting objects: 100% (6/6), done.
...
```

Finally, you can check on GitHub that our action is running:

<div class="picture">
    <img class="light-img u-drop-shadow u-border u-max-width-100" src="/docs/assets/img/github-action-light.png" width="752" alt="GitHub Action"/>
    <img class="dark-img u-drop-shadow u-border u-max-width-100" src="/docs/assets/img/github-action-dark.png" width="752" alt="GitHub Action"/>
</div>

## Running Tests with GitLab CI/CD

If you use [GitLab CI/CD], you can check [this detailed tutorial] on how to continuously run your Hurl test suite.


## Tests Report

TBD

## Recap

In less than half an hour, we have added a full CI/CD pipeline to our project.
Now, we can add more Hurl tests and start developing new features with confidence!


[`integration/basic.hurl`]: https://raw.githubusercontent.com/jcamiel/quiz/master/integration/basic.hurl
[`integration/create-quiz.hurl`]: https://raw.githubusercontent.com/jcamiel/quiz/master/integration/create-quiz.hurl
[GitHub Actions]: https://github.com/features/actions
[GitLab CI/CD pipelines]: https://docs.gitlab.com/ee/ci/pipelines/
[`bin/integration.sh`]: https://github.com/jcamiel/quiz/blob/master/bin/integration.sh
[GitLab CI/CD here]: https://about.gitlab.com/blog/2022/12/14/how-to-continously-test-web-apps-apis-with-hurl-and-gitlab-ci-cd/
[GitLab CI/CD]: https://about.gitlab.com/why-gitlab/
[this detailed tutorial]: https://about.gitlab.com/blog/2022/12/14/how-to-continously-test-web-apps-apis-with-hurl-and-gitlab-ci-cd/
[`--retry`]: /docs/manual.md#retry
