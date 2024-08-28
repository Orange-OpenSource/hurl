using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'hurlfmt' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'hurlfmt'
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
        'hurlfmt'
         {[CompletionResult]::new('--check', 'check', [CompletionResultType]::ParameterName, 'Run in check mode')
            [CompletionResult]::new('--color', 'color', [CompletionResultType]::ParameterName, 'Colorize Output')
            [CompletionResult]::new('--format', 'format', [CompletionResultType]::ParameterName, 'Specify output format: hurl, json or html')
            [CompletionResult]::new('--in-place', 'in-place', [CompletionResultType]::ParameterName, 'Modify files in place')
            [CompletionResult]::new('--in', 'in', [CompletionResultType]::ParameterName, 'Specify input format: hurl or curl')
            [CompletionResult]::new('--no-color', 'no-color', [CompletionResultType]::ParameterName, 'Do not colorize output')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'Write to FILE instead of stdout')
            [CompletionResult]::new('--out', 'out', [CompletionResultType]::ParameterName, 'Specify output format: hurl, json or html')
            [CompletionResult]::new('--standalone', 'standalone', [CompletionResultType]::ParameterName, 'Standalone HTML')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}    
    
