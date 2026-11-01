# Q-0062: Must an implementation diagnose a comparison defaulted outside its class that would be deleted?

**Status:** provisional
**Affects:** DCMP-8, `tests/classes/comparisons/out-of-class-would-delete.cpp`

## Context

A class declares `operator==` and defaults it after the class
definition. A member of the class has no `==`, so the defaulted function
would be deleted. Clang rejects the definition. GCC and EDG accept it
while no code uses the function. For copy and move constructors, all
compilers in the matrix reject the same pattern (CPY-17).

## Options

1. The program is not valid, and an implementation need not diagnose it
   (`no-diagnostic`).
2. The program is not valid and must be rejected (`reject`).
3. The function is deleted, and only a use is rejected.

## Provisional answer

No; the program is not valid, and the error is `no-diagnostic` (option 1). Clang rejects the definition, and the rule matches CPY-17. GCC
and EDG still conform.

## Opinions
