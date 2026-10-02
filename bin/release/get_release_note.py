#!/usr/bin/env python3
"""Create Release note from GitHub Issues and Pull Requests for a given version

Example:
    $ python3 bin/release/get_release_note.py 1.7.0

"""

import argparse
import datetime
import json
import sys

import requests

hurl_repo_url = "https://github.com/Orange-OpenSource/hurl"


class Pull:
    def __init__(
        self,
        url: str,
        description: str,
        author: str,
        tags: list[str] | None = None,
        issues: list[int] | None = None,
    ):
        if tags is None:
            tags = []
        if issues is None:
            issues = []
        self.url = url
        self.description = description
        self.author = author
        self.tags = tags
        self.issues = issues

    def __repr__(self):
        return f'Pull("{self.url}", "{self.description}", "{self.author}", "{self.tags}", {self.issues})'

    def __eq__(self, other):
        """Overrides the default implementation"""
        if isinstance(other, Pull):
            if self.url != other.url:
                return False
            if self.description != other.description:
                return False
            if self.author != other.author:
                return False
            if self.tags != other.tags:
                return False
            if self.issues != other.issues:  # noqa: SIM103
                return False
            return True
        return False


class Issue:
    def __init__(self, number: int, tags: list[str], author: str, pulls: list[Pull]):
        self.number = number
        self.tags = tags
        self.author = author
        self.pulls = pulls

    def __repr__(self):
        tags = ",".join([f'"{t!s}"' for t in self.tags])
        pulls = ",".join([str(p) for p in self.pulls])
        return (
            f'Issue(\n    number={self.number!s},\n    tag=["{tags}"],\n'
            f'    author="{self.author!s}",\n    pulls=[{pulls}]\n)'
        )


def release_note(milestone: str, token: str | None) -> str:
    """return Markdown release note for the given milestone"""
    date = datetime.datetime.now().astimezone()

    query = """\
query {
    repository(owner:"Orange-OpenSource", name:"hurl") {
        milestones(query:"MILESTONE", first:1) {
            edges {
                node {
                    issues(last:100, states:CLOSED) {
                        edges {
                            node {
                                title
                                number
                                url
                                author {
                                    login
                                }
                                closedByPullRequestsReferences(includeClosedPrs:true, first:5) {
                                    edges {
                                        node {
                                            title
                                            url
                                            author {
                                                login
                                            }
                                        }
                                    }
                                }
                                labels(first:5) {
                                    edges {
                                        node {
                                            name
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
"""
    query = query.replace("MILESTONE", milestone)
    payload = github_graphql(token=token, query=query)
    response = json.loads(payload)
    issues_dict = response["data"]["repository"]["milestones"]["edges"][0]["node"][
        "issues"
    ]["edges"]
    issues = []
    for issue_dict in issues_dict:
        number = issue_dict["node"]["number"]
        author_issue = issue_dict["node"]["author"]["login"]
        tags_dict = issue_dict["node"]["labels"]["edges"]
        tags = [t["node"]["name"] for t in tags_dict]

        pulls = []
        pulls_dict = issue_dict["node"]["closedByPullRequestsReferences"]["edges"]
        for pull_dict in pulls_dict:
            title = pull_dict["node"]["title"]
            url = pull_dict["node"]["url"]
            author_pull = pull_dict["node"]["author"]["login"]
            pull = Pull(description=title, url=url, author=author_pull)
            pulls.append(pull)

        issue = Issue(number=number, tags=tags, author=author_issue, pulls=pulls)
        issues.append(issue)

    pulls = pulls_from_issues(issues)
    authors = [
        author
        for author in authors_from_issues(issues)
        if author not in ["jcamiel", "lepapareil", "fabricereix"]
    ]
    return generate_md(milestone, date, pulls, authors)


def pulls_from_issues(issues: list[Issue]) -> list[Pull]:
    """return list of pulls from list of issues"""
    pulls: dict[str, Pull] = {}
    for issue in issues:
        for pull in issue.pulls:
            if pull.url in pulls:
                saved_pull = pulls[pull.url]
                for tag in issue.tags:
                    if tag not in saved_pull.tags:
                        saved_pull.tags.append(tag)
                saved_pull.issues.append(issue.number)
            else:
                if pull.url.startswith("https://github.com/Orange-OpenSource/hurl"):
                    pull.tags = issue.tags
                    pull.issues.append(issue.number)
                    pulls[pull.url] = pull

    return list(pulls.values())


def authors_from_issues(issues: list[Issue]) -> list[str]:
    """return list of unique authors from a list of issues"""
    authors = []
    for issue in issues:
        if issue.author not in authors:
            authors.append(issue.author)
        for pull in issue.pulls:
            if pull.author not in authors:
                authors.append(pull.author)
    return authors


def generate_md(
    milestone: str, date: datetime.datetime, pulls: list[Pull], authors: list[str]
) -> str:
    """Generate Markdown"""

    changelog_url = hurl_repo_url + "/blob/master/CHANGELOG.md#" + milestone
    s = f"[{milestone!s} ({date.strftime('%Y-%m-%d')})]({changelog_url})"
    s += "\n========================================================================================================================"
    s += "\n\nThanks to"
    for author in authors:
        s += f"\n[@{author!s}](https://github.com/{author!s}),"

    categories = {
        "breaking": "Breaking Changes",
        "enhancement": "Enhancements",
        "bug": "Bugs Fixed",
        "security": "Security Issues Fixed",
        "deprecation": "Deprecations",
    }

    for category, heading in categories.items():
        category_pulls = [pull for pull in pulls if category in pull.tags]
        if len(category_pulls) > 0:
            s += "\n\n" + heading + ":" + "\n\n"
        for pull in category_pulls:
            issues = " ".join(
                f"[#{issue!s}]({hurl_repo_url!s}/issues/{issue!s})"
                for issue in pull.issues
            )
            s += f"* {pull.description!s} {issues}\n"

    s += "\n"
    return s


def github_graphql(token: str | None, query: str) -> str:
    """Execute a GraphQL query using GitHub API."""
    url = "https://api.github.com/graphql"
    query_json = {"query": query}
    body = json.dumps(query_json)
    sys.stderr.write(f"* POST {url}\n")
    headers = {}
    if token:
        headers["Authorization"] = f"Bearer {token}"
    r = requests.post(url, data=body, headers=headers)
    if r.status_code != 200:
        raise requests.HTTPError(
            f"HTTP Error {r.status_code!s} - {r.text!s}", response=r
        )
    return r.text


def main():
    parser = argparse.ArgumentParser(
        description="Get Hurl release notes from issues/PR"
    )
    parser.add_argument("version", help="Hurl release version ex 4.2.0")
    parser.add_argument("--token", help="GitHub authentication token")
    args = parser.parse_args()
    if args.version == "":
        raise ValueError("version can not be empty")
    print(release_note(milestone=args.version, token=args.token))


if __name__ == "__main__":
    main()
