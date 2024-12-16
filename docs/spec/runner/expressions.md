# Generalizing Expressions in Hurl 

Expressions could both simplify the Hurl Model and make it more powerful at the same time.

Existing users should not be impacted, existing Hurl files will still be valid.
Changes will only apply under the hood, uniformizing the code.
But new usages should be naturally supported.


## Defining an expression

Expressions in Hurl will be similar to the one in programming languages, consisting of literals, variables, operators and functions.
They will have all the common precedence and associativity.

However, they will be consistent with existing Hurl style.
For example, parameters in function calls are not enclosed in parentheses, and separated with a whitespace rather than a comma.

### Examples

```
values nth 0                  
# values is a variable
# nth is a function extracting the nth element of a list
#     it is called using infix notation (aka filter)
# 0 is an integer literal and the parameter of the nth function
# the return type is the type of the element that the variable values contains

newDate format "%Y%m%d"
# newDate is a function that returns the current Datetime (aka generator)
# format is a function that convert the date to a string
# "%Y%m%d" is a string literal and the parameter the format function
# The return type is a String

a == 1 or a == 2
# This is a boolean expression (returns a boolean)
# With standard precedence, no parenthesis is needed.
```



## Using expressions in [Captures] section


The value part of the capture will be replaced by an expression.
```
[Captures]
name1: expression1
name2: expression2
...
```

### Supporting the current format

All the existing captures (with query) will still be valid.


    [Captures]
    csrf_token: xpath "string(//meta[@name='_csrf_token']/@content)"

    # xpath is a function that takes implicitly the response body as input.



### New usages

Any expression can be used. It does not have to be related to the response body anymore.

    [Captures]
    counter: counter + 1



## Using expression in [Assert] section

Asserts will be a list of expressions that should be evaluated to true in order to pass.

    [Asserts]
    expression1
    expression2 


### Supporting the current format

All the existing assert (with query and predicate) will still be valid.

    [Asserts]
    jsonpath "$.count' == 82

    # One expression using the comparison operator ==
    # with left expression <jsonpath "$count">
    # and right expression <82>


### New usages

    [Asserts]
    (status == 200) or (jsonpath "$.count' == 82) 

    # expression using the boolean operator or
    # with left expression <status == 200>
    # and right expression <jsonpath "$.count' == 82>
    # (Parenthesis have only been added for readability)


### Assert error messages

Expression that do not evaluate to true should make the assert fail.
The assert error message should be consistent with the existing one using expected/actual semantics.

Expected/actual value should be calculated implicitly from the expression tree structure.

For example
`jsonpath "$.count' == 82`
the actual value will be the left branch of the top == operator
while the expected value will be the right branch.

