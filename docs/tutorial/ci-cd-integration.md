# CI/CD Integration

Up until now, we have run our tests files locally. Now, we want to integrate
them in a CI/CD pipeline (like [GitHub Actions] or [GitLab CI/CD pipelines]). As
Hurl is very fast, we're going to run our tests on each commit of our project,
drastically improving the project quality.

A typical web project pipeline is:

- build the application, run units tests and static code analysis,
- publish the application image to a Docker registry,
- pull the application image and run integration tests.

In this workflow, we're testing the same image that will be used and deployed in
production.

> For the tutorial, we are skipping build and publication phases and
> only run integration tests on a prebuilt Docker image. To check a complete
> project with build, Docker upload/publish and integration tests, go to <https://github.com/jcamiel/quiz>

In a first step, we're going to write a bash script that will pull our Docker
image, launch it and run Hurl tests against it. Once we have checked that this
script runs locally, we'll see how to run it automatically in a CI/CD pipeline.

## Integration Script

1. First, create a directory name `quiz-project`, add [`integration/basic.hurl`]
   and [`integration/create-quiz.hurl`] from the previous tutorial to the directory.

<pre><code class="language-shell">$ mkdir quiz-project
$ cd quiz-project
$ mkdir integration
$ vi integration/basic.hurl

# Import <a href="https://raw.githubusercontent.com/jcamiel/quiz/master/integration/basic.hurl">basic.hurl</a> here!

$ vi integration/create-quiz.hurl

# Import <a href="https://raw.githubusercontent.com/jcamiel/quiz/master/integration/create-quiz.hurl">create-quiz.hurl</a> here!</code></pre>

Next, we are going to write a first version of our integration script that will
just pull the Quiz image and run it:

2. Create a script named `bin/integration.sh` with the following content:

```bash
#!/bin/bash
set -eu

echo "Starting Quiz container"
docker run --name quiz --rm --detach --publish 8080:8080 ghcr.io/jcamiel/quiz:latest
```

3. Make the script executable and run it:

```shell
$ chmod u+x bin/integration.sh
$ bin/integration.sh
Starting Quiz container
5d311561828d6078e84eb4b8b87dfd5d67bde6d9614ad83860b60cf310438d2a 
```

4. Verify that our container is up and running, and stop it.

```shell
$ docker ps
CONTAINER ID   IMAGE                         COMMAND                  CREATED         STATUS         PORTS                                       NAMES
c685f3887cc1   ghcr.io/jcamiel/quiz:latest   "java -jar app/quiz.…"   3 seconds ago   Up 3 seconds   0.0.0.0:8080->8080/tcp, :::8080->8080/tcp   quiz
$ docker stop quiz
quiz
```

Now, we have a basic script that starts our container. Before adding our
integration tests, we need to ensure that our application server is ready: the
container have started, but the application server can take a few seconds to be
really ready to accept incoming HTTP requests.

To do so, we can test our health api. With a function `wait_for_url`,
we use Hurl to check a given url to return a `200 OK`. We loop on this function
until the check succeed. Once the test has succeeded, we stop the container.

5. Modify `bin/integration.sh` to wait for the application to be ready:

```bash
#!/bin/bash
set -eu

wait_for_url () {
    echo "Testing $1"
    max_in_s=$2
    delay_in_s=1
    total_in_s=0
    while [ $total_in_s -le "$max_in_s" ]
    do
        echo "Wait ${total_in_s}s"
        if (echo -e "GET $1\nHTTP/* 200" | hurl > /dev/null 2>&1;) then
            return 0
        fi
        total_in_s=$(( total_in_s +  delay_in_s))
        sleep $delay_in_s
    done
    return 1
}

echo "Starting Quiz container"
docker run --name quiz --rm --detach --publish 8080:8080 ghcr.io/jcamiel/quiz:latest

echo "Starting Quiz instance to be ready"
wait_for_url 'http://localhost:8080' 60

echo "Stopping Quiz instance"
docker stop quiz
```

We have now the simplest integration test script: it pulls a Quiz image, then starts
the container and waits for a `200 OK` response.

Next, we're going to add our Hurl tests to the script.

6. Modify `bin/integration.sh` to add integraion tests:

```bash
#!/bin/bash
set -eu

# ...

echo "Starting Quiz container"
# ...

echo "Starting Quiz instance to be ready"
# ...

echo "Running Hurl tests"
hurl integration/*.hurl --test

echo "Stopping Quiz instance"
# ...
```

7. Run [`bin/integration.sh`] to check that our application passes all tests:

```shell
$ bin/integration.sh
Starting Quiz container
48cf21d193a01651fc42b80648abdb51dc626f31c3f9c8917aea899c68eb4a12
Starting Quiz instance to be ready
Testing http://localhost:8080
Wait 0s
Wait 1s
Wait 2s
Wait 3s
Wait 4s
Wait 5s
Running Hurl tests
integration/basic.hurl: RUNNING [1/2]
integration/basic.hurl: SUCCESS
integration/create-quiz.hurl: RUNNING [2/2]
integration/create-quiz.hurl: SUCCESS
--------------------------------------------------------------------------------
Executed:  2
Succeeded: 2 (100.0%)
Failed:    0 (0.0%)
Duration:  1026ms
Stopping Quiz instance
quiz
```

Locally, our test suite is now fully functional. As Hurl is very fast, we can use
it to ensure that new developments don't have regression. Our next step is to run
the integration tests automatically in a CI/CD pipeline. As an example, we're going
to create a [GitHub Action].

## Running Tests with GitHub Action

1. Create a new empty repository in GitHub, named `quiz-project`:

<p>
    <img class="light-img u-drop-shadow u-border" src="/docs/assets/img/github-new-repository-light.png" width="100%" alt="Create new GitHub repository"/>
    <img class="dark-img u-drop-shadow u-border" src="/docs/assets/img/github-new-repository-dark.png" width="100%" alt="Create new GitHub repository"/>
</p>


2. On your computer, create a git repo in `quiz-project` directory and
   commit the projects files:

```shell
$ git init
Initialized empty Git repository in /Users/jc/Documents/Dev/quiz-project/.git/
$ git add .
$ git commit -m "Add integration tests."
[master (root-commit) ea3e5cd] Add integration tests.
 3 files changed, 146 insertions(+)
 create mode 100755 bin/integration.sh
...
$ git branch -M main
$ git remote add origin https://github.com/jcamiel/quiz-project.git
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
        uses: actions/checkout@v2
      - name: Build
        run: echo "Building app..."
      - name: Integration test
        run: |
          curl -LO https://github.com/Orange-OpenSource/hurl/releases/download/1.4.0/hurl_1.4.0_amd64.deb
          sudo dpkg -i hurl_1.4.0_amd64.deb
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

<p>
    <img class="light-img u-drop-shadow u-border" src="/docs/assets/img/github-action-light.png" width="100%" alt="GitHub Action"/>
    <img class="dark-img u-drop-shadow u-border" src="/docs/assets/img/github-action-dark.png" width="100%" alt="GitHub Action"/>
</p>

## Tests Report

TBD

## Recap

In less than half an hour, we have added a fully CI/CD pipeline to our project.
Now, we can add more Hurl tests and start developing new features with confidence!


[`integration/basic.hurl`]: https://raw.githubusercontent.com/jcamiel/quiz/master/integration/basic.hurl
[`integration/create-quiz.hurl`]: https://raw.githubusercontent.com/jcamiel/quiz/master/integration/create-quiz.hurl
[GitHub Actions]: https://github.com/features/actions
[GitHub Action]: https://github.com/features/actions
[GitLab CI/CD pipelines]: https://docs.gitlab.com/ee/ci/pipelines/
[`bin/integration.sh`]: https://github.com/jcamiel/quiz/blob/master/bin/integration.sh
