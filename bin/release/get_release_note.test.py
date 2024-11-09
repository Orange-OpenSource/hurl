#!/usr/bin/env python3
import datetime
import unittest

from get_release_note import (
    Issue,
    Pull,
    authors_from_issues,
    generate_md,
    pulls_from_issues,
    webscrapping_linked_pulls,
)

ISSUES = [
    Issue(number=1, tags=["enhancement"], author="bob", pulls=[Pull("url1", "pull1")]),
    Issue(
        number=2,
        tags=["bug"],
        author="bill",
        pulls=[Pull("url2", "pull2"), Pull("url3", "pull3")],
    ),
    Issue(
        number=3,
        tags=["ignore", "enhancement"],
        author="bob",
        pulls=[Pull("url4", "pull4")],
    ),
    Issue(number=4, tags=["enhancement"], author="bob", pulls=[Pull("url4", "pull4")]),
]

PULLS = [
    Pull("url1", "pull1", ["enhancement"], [1]),
    Pull("url2", "pull2", ["bug"], [2]),
    Pull("url3", "pull3", ["bug"], [2]),
    Pull("url4", "pull4", ["ignore", "enhancement"], [3, 4]),
]

# Example from https://github.com/Orange-OpenSource/hurl/issues/903
ISSUE_HTML = """<development-menu data-catalyst="">
      <!-- '"` --><!-- </textarea></xmp> --><form data-target="create-branch.developmentForm" data-turbo="false" class="js-issue-sidebar-form" aria-label="Link issues" action="/Orange-OpenSource/hurl/issues/closing_references?source_id=1411443741&amp;source_type=ISSUE" accept-charset="UTF-8" method="post"><input type="hidden" name="_method" value="put" autocomplete="off"><input type="hidden" name="authenticity_token" value="9x5KgxKfdSMSV4AvkKo8xG2tb4BzMW-ZaZpRZBtx30AGFQE4zYBJEf0oAWEdnepcxG-5dH1Edkm000q_FXXkhw">
        
      <details class="details-reset details-overlay position-relative" data-target="development-menu.details" data-action="click:development-menu#openSelectedRepoFromStorage" current-user="fabricereix" repo-nwo="Orange-OpenSource/hurl">
      <summary class="text-bold discussion-sidebar-heading discussion-sidebar-toggle" aria-haspopup="menu" data-hotkey="x" data-menu-trigger="development-select-menu" role="button">
        <svg aria-hidden="true" height="16" viewBox="0 0 16 16" version="1.1" width="16" data-view-component="true" class="octicon octicon-gear">
    <path fill-rule="evenodd" d="M7.429 1.525a6.593 6.593 0 011.142 0c.036.003.108.036.137.146l.289 1.105c.147.56.55.967.997 1.189.174.086.341.183.501.29.417.278.97.423 1.53.27l1.102-.303c.11-.03.175.016.195.046.219.31.41.641.573.989.014.031.022.11-.059.19l-.815.806c-.411.406-.562.957-.53 1.456a4.588 4.588 0 010 .582c-.032.499.119 1.05.53 1.456l.815.806c.08.08.073.159.059.19a6.494 6.494 0 01-.573.99c-.02.029-.086.074-.195.045l-1.103-.303c-.559-.153-1.112-.008-1.529.27-.16.107-.327.204-.5.29-.449.222-.851.628-.998 1.189l-.289 1.105c-.029.11-.101.143-.137.146a6.613 6.613 0 01-1.142 0c-.036-.003-.108-.037-.137-.146l-.289-1.105c-.147-.56-.55-.967-.997-1.189a4.502 4.502 0 01-.501-.29c-.417-.278-.97-.423-1.53-.27l-1.102.303c-.11.03-.175-.016-.195-.046a6.492 6.492 0 01-.573-.989c-.014-.031-.022-.11.059-.19l.815-.806c.411-.406.562-.957.53-1.456a4.587 4.587 0 010-.582c.032-.499-.119-1.05-.53-1.456l-.815-.806c-.08-.08-.073-.159-.059-.19a6.44 6.44 0 01.573-.99c.02-.029.086-.075.195-.045l1.103.303c.559.153 1.112.008 1.529-.27.16-.107.327-.204.5-.29.449-.222.851-.628.998-1.189l.289-1.105c.029-.11.101-.143.137-.146zM8 0c-.236 0-.47.01-.701.03-.743.065-1.29.615-1.458 1.261l-.29 1.106c-.017.066-.078.158-.211.224a5.994 5.994 0 00-.668.386c-.123.082-.233.09-.3.071L3.27 2.776c-.644-.177-1.392.02-1.82.63a7.977 7.977 0 00-.704 1.217c-.315.675-.111 1.422.363 1.891l.815.806c.05.048.098.147.088.294a6.084 6.084 0 000 .772c.01.147-.038.246-.088.294l-.815.806c-.474.469-.678 1.216-.363 1.891.2.428.436.835.704 1.218.428.609 1.176.806 1.82.63l1.103-.303c.066-.019.176-.011.299.071.213.143.436.272.668.386.133.066.194.158.212.224l.289 1.106c.169.646.715 1.196 1.458 1.26a8.094 8.094 0 001.402 0c.743-.064 1.29-.614 1.458-1.26l.29-1.106c.017-.066.078-.158.211-.224a5.98 5.98 0 00.668-.386c.123-.082.233-.09.3-.071l1.102.302c.644.177 1.392-.02 1.82-.63.268-.382.505-.789.704-1.217.315-.675.111-1.422-.364-1.891l-.814-.806c-.05-.048-.098-.147-.088-.294a6.1 6.1 0 000-.772c-.01-.147.039-.246.088-.294l.814-.806c.475-.469.679-1.216.364-1.891a7.992 7.992 0 00-.704-1.218c-.428-.609-1.176-.806-1.82-.63l-1.103.303c-.066.019-.176.011-.299-.071a5.991 5.991 0 00-.668-.386c-.133-.066-.194-.158-.212-.224L10.16 1.29C9.99.645 9.444.095 8.701.031A8.094 8.094 0 008 0zm1.5 8a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0zM11 8a3 3 0 11-6 0 3 3 0 016 0z"></path>
</svg>
        Development
      </summary>

      <details-menu class="SelectMenu SelectMenu--hasFilter right-0 page-responsive" data-target="development-menu.repoMenu" data-action="click:development-menu#stopPropagation" aria-label="Repository menu" role="menu">
        <div class="SelectMenu-modal development-menu-component-menu-modal hx_rsm-modal-sm">
          <header class="SelectMenu-header">
            <h3 class="SelectMenu-title color-fg-default">
              Link a branch or pull request
              <span class="text-normal color-fg-muted d-block">
                Select a repository to search for branches and pull requests
                or
                  <button aria-label="Create a branch" data-action="click:create-branch#openDialog" type="button" data-view-component="true" class="btn-link">    create a branch
</button>
              </span>
            </h3>
            <button class="SelectMenu-closeButton top-0 right-0" type="button" data-action="click:development-menu#closeMenu">
              <svg aria-label="Close menu" role="img" height="16" viewBox="0 0 16 16" version="1.1" width="16" data-view-component="true" class="octicon octicon-x">
    <path fill-rule="evenodd" d="M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z"></path>
</svg>
            </button>
          </header>
          <div class="SelectMenu-filter">
            <remote-input aria-owns="development-menu-repository-list" param="repositories" src="/Orange-OpenSource/hurl/issues/closing_references/referencing_repositories?source_id=1411443741&amp;source_type=ISSUE" data-action="
                remote-input-success:development-menu#repositoryListLoaded
                remote-input-error:development-menu#repositoryListLoadEnd
                loadstart:development-menu#repositoryListLoadStart
                loadend:development-menu#repositoryListLoadEnd
              ">
              <input type="text" class="SelectMenu-input form-control" placeholder="Search for repositories" data-target="development-menu.repoSearchInput" aria-label="Search for repositories" autocomplete="off" autofocus="" spellcheck="false">
            </remote-input>
          </div>
          <div class="SelectMenu-list">
            <div class="SelectMenu-loading" data-target="development-menu.repositoryListSpinner">
              <svg style="box-sizing: content-box; color: var(--color-icon-primary);" width="32" height="32" viewBox="0 0 16 16" fill="none" data-view-component="true" class="anim-rotate">
  <circle cx="8" cy="8" r="7" stroke="currentColor" stroke-opacity="0.25" stroke-width="2" vector-effect="non-scaling-stroke"></circle>
  <path d="M15 8a7.002 7.002 0 00-7-7" stroke="currentColor" stroke-width="2" stroke-linecap="round" vector-effect="non-scaling-stroke"></path>
</svg>
            </div>
            <div data-target="development-menu.repositoryList" id="development-menu-repository-list"></div>
          </div>
        </div>
      </details-menu>
      <modal-dialog class="development-menu-component-dialog right-0 hx_rsm-modal-sm" data-target="development-menu.branchOrPullRequestDialog" data-action="click:development-menu#stopPropagation" aria-label="Branch or pull request dialog" role="dialog" hidden="">
        <input type="text" name="repository_id" data-target="development-menu.selectedRepoIdInput" value="" hidden="">
        <div class="SelectMenu-modal development-menu-component-dialog-modal">
          <div class="SelectMenu-header">
            <button class="SelectMenu-closeButton top-0 left-0 mr-2" type="button" data-action="click:development-menu#closeBranchOrPullRequestDialog">
              <svg aria-label="Back to repository menu" role="img" height="16" viewBox="0 0 16 16" version="1.1" width="16" data-view-component="true" class="octicon octicon-arrow-left">
    <path fill-rule="evenodd" d="M7.78 12.53a.75.75 0 01-1.06 0L2.47 8.28a.75.75 0 010-1.06l4.25-4.25a.75.75 0 011.06 1.06L4.81 7h7.44a.75.75 0 010 1.5H4.81l2.97 2.97a.75.75 0 010 1.06z"></path>
</svg>
            </button>
            <h3 class="SelectMenu-title">
              <div class="color-fg-default line-clamp-1" data-target="development-menu.dialogTitle"></div>
              <span class="text-normal color-fg-muted">Link a branch, pull request, or
                  <button aria-label="create a branch" data-action="click:create-branch#openDialog" type="button" data-view-component="true" class="btn-link">    create a branch
</button>
              </span>
            </h3>
          </div>
          <div class="SelectMenu-filter">
            <remote-input aria-owns="development-menu-branch-and-pull-request-list" param="linkable_items" data-target="development-menu.branchAndPullRequestSearch" data-action="
                remote-input-success:development-menu#branchAndPullRequestListLoaded
                remote-input-error:development-menu#branchAndPullRequestListLoadEnd
                loadstart:development-menu#branchAndPullRequestListLoadStart
                loadend:development-menu#branchAndPullRequestListLoadEnd
              " src="">
              <input type="text" name="linkable_items" class="SelectMenu-input form-control" data-target="development-menu.branchAndPullRequestSearchInput" placeholder="Search for branches or pull requests" aria-label="Search for branches or pull requests" autocomplete="off" autofocus="" spellcheck="false">
            </remote-input>
          </div>
          <div class="SelectMenu-list">
            <div class="SelectMenu-loading" data-target="development-menu.branchAndPullRequestListSpinner">
              <svg style="box-sizing: content-box; color: var(--color-icon-primary);" width="32" height="32" viewBox="0 0 16 16" fill="none" data-view-component="true" class="anim-rotate">
  <circle cx="8" cy="8" r="7" stroke="currentColor" stroke-opacity="0.25" stroke-width="2" vector-effect="non-scaling-stroke"></circle>
  <path d="M15 8a7.002 7.002 0 00-7-7" stroke="currentColor" stroke-width="2" stroke-linecap="round" vector-effect="non-scaling-stroke"></path>
</svg>
            </div>
            <div data-target="development-menu.branchAndPullRequestList" id="development-menu-branch-and-pull-request-list"></div>
          </div>
          <div class="SelectMenu-footer">
            <div class="d-flex flex-justify-start flex-row-reverse">
                <button disabled="disabled" aria-label="Apply" data-target="development-menu.applyButton" type="submit" data-view-component="true" class="btn-primary btn-sm btn ml-2">    Apply
</button>

                <button aria-label="Close" data-action="click:development-menu#resetForm" type="button" data-view-component="true" class="btn-sm btn">    Cancel
</button>
            </div>
          </div>
        </div>
      </modal-dialog>
      <div class="development-menu-component-dialog-overlay"></div>
    </details>



          
  <p>Successfully merging a pull request may close this issue.</p>




    
<div class="my-1">
  <span data-view-component="true" class="Truncate truncate-with-responsive-width">
    <a href="/Orange-OpenSource/hurl/pull/904" data-hydro-click="{&quot;event_type&quot;:&quot;issue_cross_references.click&quot;,&quot;payload&quot;:{&quot;reference_location&quot;:&quot;ISSUE_SIDEBAR&quot;,&quot;user_id&quot;:682123,&quot;issue_id&quot;:1411443741,&quot;pull_request_id&quot;:1089349607,&quot;originating_url&quot;:&quot;https://github.com/Orange-OpenSource/hurl/issues/903&quot;}}" data-hydro-click-hmac="d05663479c42215476657359e8e93599ac2bf7972f47d0a45f571ae306dbe56c" data-hovercard-type="pull_request" data-hovercard-url="/Orange-OpenSource/hurl/pull/904/hovercard" data-view-component="true" class="Truncate-text Link--primary markdown-title text-bold d-block">      <svg class="octicon octicon-git-merge merged" title="Merged" aria-label="Merged pull request" viewBox="0 0 16 16" version="1.1" width="16" height="16" role="img"><path fill-rule="evenodd" d="M5 3.254V3.25v.005a.75.75 0 110-.005v.004zm.45 1.9a2.25 2.25 0 10-1.95.218v5.256a2.25 2.25 0 101.5 0V7.123A5.735 5.735 0 009.25 9h1.378a2.251 2.251 0 100-1.5H9.25a4.25 4.25 0 01-3.8-2.346zM12.75 9a.75.75 0 100-1.5.75.75 0 000 1.5zm-8.5 4.5a.75.75 0 100-1.5.75.75 0 000 1.5z"></path></svg>
      Fix HTTP HEAD
</a>
</span>    <a href="/Orange-OpenSource/hurl" class="d-block Link--muted f6 pl-1 ml-3">
      Orange-OpenSource/hurl
    </a>
</div>


</form>    </development-menu>
"""


ISSUE_WITH_EMOJI_HTML = """
<html>
<head>
    <meta charset="UTF-8">
    <meta name="description" content="👋 hello">
    <title>Issue</title>
</head>
<body>
<development-menu>
    <a href="/Orange-OpenSource/hurl/pull/958">Issue 958</a>
</development-menu>
</body>
</html>
"""


class GetReleaseNoteTest(unittest.TestCase):
    def test_authors_from_issues(self):
        self.assertEqual(["bob", "bill"], authors_from_issues(ISSUES))

    def test_pulls_from_issues(self):
        self.assertEqual(PULLS, pulls_from_issues(ISSUES))

    def test_webscrapping_issue(self):
        self.assertEqual(
            [Pull("/Orange-OpenSource/hurl/pull/904", "Fix HTTP HEAD", [], [])],
            webscrapping_linked_pulls(ISSUE_HTML),
        )

    def test_webscrapping_issue_with_emoji(self):
        self.assertEqual(
            [Pull("/Orange-OpenSource/hurl/pull/958", "Issue 958", [], [])],
            webscrapping_linked_pulls(ISSUE_WITH_EMOJI_HTML),
        )

    def test_generate_md(self):
        self.assertEqual(
            """[1.0.0 (2022-01-01)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#1.0.0)
========================================================================================================================

Thanks to
[@bob](https://github.com/bob),
[@bill](https://github.com/bill),


Enhancements:

* pull1 [#1](https://github.com/Orange-OpenSource/hurl/issues/1)

* pull4 [#3](https://github.com/Orange-OpenSource/hurl/issues/3) [#4](https://github.com/Orange-OpenSource/hurl/issues/4)


Bugs Fixed:

* pull2 [#2](https://github.com/Orange-OpenSource/hurl/issues/2)

* pull3 [#2](https://github.com/Orange-OpenSource/hurl/issues/2)
""",
            generate_md(
                milestone="1.0.0",
                date=datetime.datetime(2022, 1, 1),
                pulls=PULLS,
                authors=["bob", "bill"],
            ),
        )


if __name__ == "__main__":
    unittest.main()
