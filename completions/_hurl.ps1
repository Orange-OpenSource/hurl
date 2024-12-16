using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'hurl' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'hurl'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'hurl'
         {[CompletionResult]::new('--aws-sigv4', 'aws-sigv4', [CompletionResultType]::ParameterName, 'Use AWS V4 signature authentication in the transfer')
            [CompletionResult]::new('--cacert', 'cacert', [CompletionResultType]::ParameterName, 'CA certificate to verify peer against (PEM format)')
            [CompletionResult]::new('--cert', 'cert', [CompletionResultType]::ParameterName, 'Client certificate file and password')
            [CompletionResult]::new('--key', 'key', [CompletionResultType]::ParameterName, 'Private key file name')
            [CompletionResult]::new('--color', 'color', [CompletionResultType]::ParameterName, 'Colorize output')
            [CompletionResult]::new('--compressed', 'compressed', [CompletionResultType]::ParameterName, 'Request compressed response (using deflate or gzip)')
            [CompletionResult]::new('--connect-timeout', 'connect-timeout', [CompletionResultType]::ParameterName, 'Maximum time allowed for connection')
            [CompletionResult]::new('--connect-to', 'connect-to', [CompletionResultType]::ParameterName, 'For a request to the given HOST1:PORT1 pair, connect to HOST2:PORT2 instead')
            [CompletionResult]::new('--continue-on-error', 'continue-on-error', [CompletionResultType]::ParameterName, 'Continue executing requests even if an error occurs')
            [CompletionResult]::new('--cookie', 'cookie', [CompletionResultType]::ParameterName, 'Read cookies from FILE')
            [CompletionResult]::new('--cookie-jar', 'cookie-jar', [CompletionResultType]::ParameterName, 'Write cookies to FILE after running the session (only for one session)')
            [CompletionResult]::new('--curl', 'curl', [CompletionResultType]::ParameterName, 'Export each request to a list of curl commands')
            [CompletionResult]::new('--delay', 'delay', [CompletionResultType]::ParameterName, 'Sets delay before each request (aka sleep)')
            [CompletionResult]::new('--error-format', 'error-format', [CompletionResultType]::ParameterName, 'Control the format of error messages')
            [CompletionResult]::new('--file-root', 'file-root', [CompletionResultType]::ParameterName, 'Set root directory to import files [default: input file directory]')
            [CompletionResult]::new('--location', 'location', [CompletionResultType]::ParameterName, 'Follow redirects')
            [CompletionResult]::new('--location-trusted', 'location-trusted', [CompletionResultType]::ParameterName, 'Follow redirects but allows sending the name + password to all hosts that the site may redirect to')
            [CompletionResult]::new('--from-entry', 'from-entry', [CompletionResultType]::ParameterName, 'Execute Hurl file from ENTRY_NUMBER (starting at 1)')
            [CompletionResult]::new('--glob', 'glob', [CompletionResultType]::ParameterName, 'Specify input files that match the given GLOB. Multiple glob flags may be used')
            [CompletionResult]::new('--header', 'header', [CompletionResultType]::ParameterName, 'Pass custom header(s) to server')
            [CompletionResult]::new('--http1.0', 'http1.0', [CompletionResultType]::ParameterName, 'Tell Hurl to use HTTP version 1.0')
            [CompletionResult]::new('--http1.1', 'http1.1', [CompletionResultType]::ParameterName, 'Tell Hurl to use HTTP version 1.1')
            [CompletionResult]::new('--http2', 'http2', [CompletionResultType]::ParameterName, 'Tell Hurl to use HTTP version 2')
            [CompletionResult]::new('--http3', 'http3', [CompletionResultType]::ParameterName, 'Tell Hurl to use HTTP version 3')
            [CompletionResult]::new('--ignore-asserts', 'ignore-asserts', [CompletionResultType]::ParameterName, 'Ignore asserts defined in the Hurl file')
            [CompletionResult]::new('--include', 'include', [CompletionResultType]::ParameterName, 'Include the HTTP headers in the output')
            [CompletionResult]::new('--insecure', 'insecure', [CompletionResultType]::ParameterName, 'Allow insecure SSL connections')
            [CompletionResult]::new('--interactive', 'interactive', [CompletionResultType]::ParameterName, 'Turn on interactive mode')
            [CompletionResult]::new('--ipv4', 'ipv4', [CompletionResultType]::ParameterName, 'Tell Hurl to use IPv4 addresses only when resolving host names, and not for example try IPv6')
            [CompletionResult]::new('--ipv6', 'ipv6', [CompletionResultType]::ParameterName, 'Tell Hurl to use IPv6 addresses only when resolving host names, and not for example try IPv4')
            [CompletionResult]::new('--jobs', 'jobs', [CompletionResultType]::ParameterName, 'Maximum number of parallel jobs')
            [CompletionResult]::new('--json', 'json', [CompletionResultType]::ParameterName, 'Output each Hurl file result to JSON')
            [CompletionResult]::new('--limit-rate', 'limit-rate', [CompletionResultType]::ParameterName, 'Specify the maximum transfer rate in bytes/second, for both downloads and uploads')
            [CompletionResult]::new('--max-filesize', 'max-filesize', [CompletionResultType]::ParameterName, 'Specify the maximum size in bytes of a file to download')
            [CompletionResult]::new('--max-redirs', 'max-redirs', [CompletionResultType]::ParameterName, 'Maximum number of redirects allowed, -1 for unlimited redirects')
            [CompletionResult]::new('--max-time', 'max-time', [CompletionResultType]::ParameterName, 'Maximum time allowed for the transfer')
            [CompletionResult]::new('--netrc', 'netrc', [CompletionResultType]::ParameterName, 'Must read .netrc for username and password')
            [CompletionResult]::new('--netrc-file', 'netrc-file', [CompletionResultType]::ParameterName, 'Specify FILE for .netrc')
            [CompletionResult]::new('--netrc-optional', 'netrc-optional', [CompletionResultType]::ParameterName, 'Use either .netrc or the URL')
            [CompletionResult]::new('--no-color', 'no-color', [CompletionResultType]::ParameterName, 'Do not colorize output')
            [CompletionResult]::new('--no-output', 'no-output', [CompletionResultType]::ParameterName, 'Suppress output. By default, Hurl outputs the body of the last response')
            [CompletionResult]::new('--noproxy', 'noproxy', [CompletionResultType]::ParameterName, 'List of hosts which do not use proxy')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'Write to FILE instead of stdout')
            [CompletionResult]::new('--parallel', 'parallel', [CompletionResultType]::ParameterName, 'Run files in parallel (default in test mode)')
            [CompletionResult]::new('--path-as-is', 'path-as-is', [CompletionResultType]::ParameterName, 'Tell Hurl to not handle sequences of /../ or /./ in the given URL path')
            [CompletionResult]::new('--proxy', 'proxy', [CompletionResultType]::ParameterName, 'Use proxy on given PROTOCOL/HOST/PORT')
            [CompletionResult]::new('--repeat', 'repeat', [CompletionResultType]::ParameterName, 'Repeat the input files sequence NUM times, -1 for infinite loop')
            [CompletionResult]::new('--report-html', 'report-html', [CompletionResultType]::ParameterName, 'Generate HTML report to DIR')
            [CompletionResult]::new('--report-json', 'report-json', [CompletionResultType]::ParameterName, 'Generate JSON report to DIR')
            [CompletionResult]::new('--report-junit', 'report-junit', [CompletionResultType]::ParameterName, 'Write a JUnit XML report to FILE')
            [CompletionResult]::new('--report-tap', 'report-tap', [CompletionResultType]::ParameterName, 'Write a TAP report to FILE')
            [CompletionResult]::new('--resolve', 'resolve', [CompletionResultType]::ParameterName, 'Provide a custom address for a specific HOST and PORT pair')
            [CompletionResult]::new('--retry', 'retry', [CompletionResultType]::ParameterName, 'Maximum number of retries, 0 for no retries, -1 for unlimited retries')
            [CompletionResult]::new('--retry-interval', 'retry-interval', [CompletionResultType]::ParameterName, 'Interval in milliseconds before a retry')
            [CompletionResult]::new('--secret', 'secret', [CompletionResultType]::ParameterName, 'Define a variable which value is secret')
            [CompletionResult]::new('--ssl-no-revoke', 'ssl-no-revoke', [CompletionResultType]::ParameterName, '(Windows) Tell Hurl to disable certificate revocation checks')
            [CompletionResult]::new('--test', 'test', [CompletionResultType]::ParameterName, 'Activate test mode (use parallel execution)')
            [CompletionResult]::new('--to-entry', 'to-entry', [CompletionResultType]::ParameterName, 'Execute Hurl file to ENTRY_NUMBER (starting at 1)')
            [CompletionResult]::new('--unix-socket', 'unix-socket', [CompletionResultType]::ParameterName, '(HTTP) Connect through this Unix domain socket, instead of using the network')
            [CompletionResult]::new('--user', 'user', [CompletionResultType]::ParameterName, 'Add basic Authentication header to each request')
            [CompletionResult]::new('--user-agent', 'user-agent', [CompletionResultType]::ParameterName, 'Specify the User-Agent string to send to the HTTP server')
            [CompletionResult]::new('--variable', 'variable', [CompletionResultType]::ParameterName, 'Define a variable')
            [CompletionResult]::new('--variables-file', 'variables-file', [CompletionResultType]::ParameterName, 'Define a properties file in which you define your variables')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Turn on verbose')
            [CompletionResult]::new('--very-verbose', 'very-verbose', [CompletionResultType]::ParameterName, 'Turn on verbose output, including HTTP response and libcurl logs')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}    
    
