#!/usr/bin/env python3
"""Create Release note from GitHub Issues and Pull Requests for a given version

Example:
    $ python3 bin/release/get_release_note.py 1.7.0

"""

import argparse
import datetime
import json
import sys
from typing import List, Optional

import requests

hurl_repo_url = "https://github.com/Orange-OpenSource/hurl"


class Pull:
    def __init__(
        self,
        url: str,
        description: str,
        author: str,
        tags: Optional[List[str]] = None,
        issues: Optional[List[int]] = None,
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
        return 'Pull("%s", "%s", "%s", "%s", %s)' % (
            self.url,
            self.description,
            self.author,
            str(self.tags),
            str(self.issues),
        )

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
            if self.issues != other.issues:
                return False
            return True
        return False


class Issue:
    def __init__(self, number: int, tags: List[str], author: str, pulls: List[Pull]):
        self.number = number
        self.tags = tags
        self.author = author
        self.pulls = pulls

    def __repr__(self):
        return (
            'Issue(\n    number=%s,\n    tag=["%s"],\n    author="%s",\n    pulls=[%s]\n)'
            % (
                self.number,
                ",".join(['"%s"' % t for t in self.tags]),
                self.author,
                ",".join([str(p) for p in self.pulls]),
            )
        )


def release_note(milestone: str, token: Optional[str]) -> str:
    """return markdown release note for the given milestone"""
    date = datetime.datetime.now()

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


def pulls_from_issues(issues: List[Issue]) -> List[Pull]:
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


def authors_from_issues(issues: List[Issue]) -> List[str]:
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
    milestone: str, date: datetime.datetime, pulls: List[Pull], authors: List[str]
) -> str:
    """Generate Markdown"""

    s = "[%s (%s)](%s)" % (
        milestone,
        date.strftime("%Y-%m-%d"),
        hurl_repo_url + "/blob/master/CHANGELOG.md#" + milestone,
    )
    s += "\n========================================================================================================================"
    s += "\n\nThanks to"
    for author in authors:
        s += "\n[@%s](https://github.com/%s)," % (author, author)

    categories = {
        "breaking": "Breaking Changes",
        "enhancement": "Enhancements",
        "bug": "Bugs Fixed",
        "security": "Security Issues Fixed",
    }

    for category in categories:
        category_pulls = [pull for pull in pulls if category in pull.tags]
        if len(category_pulls) > 0:
            s += "\n\n" + categories[category] + ":" + "\n\n"
        for pull in category_pulls:
            issues = " ".join(
                "[#%s](%s/issues/%s)" % (issue, hurl_repo_url, issue)
                for issue in pull.issues
            )
            s += "* %s %s\n" % (pull.description, issues)

    s += "\n"
    return s


def github_graphql(token: Optional[str], query: str) -> str:
    """Execute a GraphQL query using GitHub API."""
    url = "https://api.github.com/graphql"
    query_json = {"query": query}
    body = json.dumps(query_json)
    sys.stderr.write("* POST %s\n" % url)
    headers = {}
    if token:
        headers["Authorization"] = f"Bearer {token}"
    r = requests.post(url, data=body, headers=headers)
    if r.status_code != 200:
        raise Exception("HTTP Error %s - %s" % (r.status_code, r.text))
    return r.text


def main():
    parser = argparse.ArgumentParser(
        description="Get Hurl release notes from issues/PR"
    )
    parser.add_argument("version", help="Hurl release version ex 4.2.0")
    parser.add_argument("--token", help="GitHub authentication token")
    args = parser.parse_args()
    if args.version == "":
        raise Exception("version can not be empty")
    print(release_note(milestone=args.version, token=args.token))


if __name__ == "__main__":
    main()
