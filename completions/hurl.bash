# hurl(1) completion                            -*- shell-script -*-
_hurl()
{
    cur="${COMP_WORDS[COMP_CWORD]}"

    if [[ $cur == -* ]]; then
        COMPREPLY=($(compgen -W '--aws-sigv4 --cacert --cert --key --color --compressed --connect-timeout --connect-to --continue-on-error --cookie --cookie-jar --curl --delay --error-format --file-root --location --location-trusted --from-entry --glob --header --http1.0 --http1.1 --http2 --http3 --ignore-asserts --include --insecure --interactive --ipv4 --ipv6 --jobs --json --limit-rate --max-filesize --max-redirs --max-time --negotiate --netrc --netrc-file --netrc-optional --no-color --no-output --no-pretty --noproxy --ntlm --output --parallel --path-as-is --pinnedpubkey --pretty --progress-bar --proxy --repeat --report-html --report-json --report-junit --report-tap --resolve --retry --retry-interval --secret --secrets-file --ssl-no-revoke --test --to-entry --unix-socket --user --user-agent --variable --variables-file --verbose --very-verbose --help --version' -- "$cur"))
        return
    fi
    # Generate filenames by default
    COMPREPLY=($(compgen -f "$cur" | sort))
} &&
    complete -F _hurl hurl
# ex: filetype=sh

