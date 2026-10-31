# Q-0100: Is a character literal with value zero a null pointer constant?

**Status:** provisional
**Affects:** CONV-8, `tests/expressions/conversions/char-zero-not-null.cpp`

## Context

A pointer is initialized from a character literal whose value is zero. Clang and GCC reject it. EDG 7.0 accepts it as a null pointer.

## Options

1. Only an integer literal with value zero is a null pointer constant.

2. A character literal with value zero is one too.

## Provisional answer

Option 1. Clang and GCC agree. ## Can a case or default label be the last item of a block? Affects: STMT-5, tests/statements/blocks/case-label-ends-block.cpp, tests/statements/blocks/default-label-ends-block.cpp Context: A switch body ends with a case or default label and no statement after it. Clang and EDG accept it. GCC 14 and 16 reject it with a parse error, though they accept an identifier label in that place. Options: 1. Valid, as for an identifier label. 2. Not valid. Provisional answer: Option 1. Clang and EDG agree, and the rule for identifier labels is the same. ## Can a jump skip a variable without an initializer whose destructor is not trivial? Affects: JMP-6, SW-5, tests/statements/jumps/goto-skips-nontrivial-destructor.cpp, tests/statements/switch/case-skips-nontrivial-destructor.cpp Context: A goto or case label skips the declaration, without an initializer, of a class whose destructor is user-provided. Clang and EDG reject the jump. GCC accepts it without a diagnostic. Options: 1. Not valid; the destructor would run for an object never constructed. 2. Valid. Provisional answer: Option 1. Clang and EDG agree. ## Does a data member named begin or end select the member form of a range-based for? Affects: RFOR-2, tests/statements/range-for/data-member-begin-counts.cpp, tests/statements/range-for/callable-data-members.cpp Context: A class has members named begin and end that are data members. Clang and GCC use the member form, so data members of callable type work and an int data member makes the loop not valid. EDG 7.0 uses the member form only when they are member functions, so it falls back to free functions. Options: 1. Members of any kind count. 2. Only member functions count. Provisional answer: Option 1. Clang and GCC agree. ## Is a subscript of an array xvalue an xvalue? Affects: VC-3, tests/expressions/value-categories/xvalue-array-subscript.cpp Context: A subscript applied to an array that is an xvalue, for example a member array of a class rvalue. Clang and GCC give an xvalue. EDG 7.0 gives an lvalue. Options: 1. An xvalue. 2. An lvalue. Provisional answer: Option 1. Clang and GCC agree, and it matches the member access rule. ## Is a character literal with the value zero a null pointer constant? Affects: CONV-8, tests/expressions/conversions/char-zero-not-null.cpp Context: A pointer is initialized from a character literal whose value is zero. Clang and GCC reject it. EDG 7.0 accepts it. Options: 1. Not a null pointer constant; only integer literals with value zero and nullptr are. 2. Any constant with value zero of integral type. Provisional answer: Option 1. Clang and GCC agree.

## Opinions
