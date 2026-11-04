# Predict batches: overloading, reader A

- Date: 2026-11-03
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R-overload-resolution (171 programs)

The answers file was written:
- /home/ryanm/Code/tidal/build/exercises/R-overload-resolution/answers.json

I answered `depends` wherever an Evidence segment records that the compilers differ on the case. These are p09, p16, p23, p51, p56, p60, p63, p66, p67, p78, p83, p85, p89, p101, p110, p112, p113, p114, p115, p120, p121, p131, p140, p143, p159, p161, and p168. For p62 I answered reject, because the Evidence describes EDG only for the form with no argument.

## RPA-1

- Pages and rules: overload-resolution, OVL-11 (Q-0141)
- Kind: unclear
- What happened: p08 adds a class that converts to `int*` and the literal `1.5`. The built-in `(T*, std::ptrdiff_t)` candidate wins. OVL-11 says only the class operand converts by its conversion function, so `int* + double` stays, and I predicted reject. The Q-0141 divergence says EDG converts a class operand to the parameter type. The page does not say what EDG does with a non-class operand whose parameter type differs, such as the double here. In p159 the double comes from the class operand, so I answered `depends` there.
- Suggestion: In OVL-11, state what happens to a non-class operand of the winning built-in candidate. Add to the Q-0141 bullet whether EDG also converts that operand.

## RPA-2

- Pages and rules: overload-resolution, OVL-21, OVL-44, Evidence bullets for Q-0143 and Q-0161
- Kind: hard to find
- What happened: p09, p60, p62, p101, p131, and p41 all call through `&f`, `&(C::f)`, or `&C::f` on a set that holds a non-static member. Three Evidence bullets give the compiler behavior, and each covers a different subset (with or without an argument, parenthesized or not, the static member winning or not). "EDG accepts both forms with no argument" left me unsure what EDG does with an argument (p62). I could not map each program to one result with confidence.
- Suggestion: Replace the three bullets with one table. Give one row per form (`&f`, `&(C::f)`, `&C::f`, each with and without an argument) and one column per compiler family.

## RPA-3

- Pages and rules: overload-resolution, OVL-14, OVL-26 (Q-0189)
- Kind: unclear
- What happened: In p15 and p70, `Base b = Source{}` copy-initializes a class from a class that converts to it or to a class derived from it. OVL-14 lists the converting constructors of `T` as candidates. The page does not say whether `T`'s copy or move constructor can take the `Source` through a user-defined conversion. The Q-0189 bullet says a constructor and a conversion function to a derived class tie, but it does not say which initialization form that is. I used the initialization grid ("conversion to derived", copy init: accept) and answered compiles for both. For p110 I answered `depends`, since there a real converting constructor `Target(const Source&)` competes.
- Suggestion: In OVL-14, say that in this form a copy or move constructor cannot get its argument by a user-defined conversion (or say the opposite, if that is the rule). Name the form that Q-0189 covers.

## RPA-4

- Pages and rules: conversion-sequences, ICS-10; overload-resolution, OVL-P4 table
- Kind: missing
- What happened: p53 applies `++` and postfix `++` to a class that converts to `int&`. ICS-10 forbids a user-defined conversion only for the left operand of a built-in assignment. The page does not say whether the operand of `++` and `--` is also restricted. I assumed it is not and answered compiles. p136 relies on the same point, with `bool&` and the missing `bool` row.
- Suggestion: In ICS-10 or the OVL-P4 table, state whether the operand of a built-in `++` or `--` can come from a conversion function, with one example.

## RPA-5

- Pages and rules: overload-resolution, OVL-9, OVL-36, Evidence "Results where every compiler agrees"
- Kind: needs an example
- What happened: p07 and p154 set the built-in `!` or `&&` with an `explicit` conversion to `bool` against an operator function that takes a class with a converting constructor. The Evidence says the two "tie" but does not say the program is rejected. I answered reject under OVL-36.
- Suggestion: Say in that bullet that the tie makes the expression ambiguous and every compiler rejects it.

## RPA-6

- Pages and rules: overload-resolution, OVL-10
- Kind: needs an example
- What happened: p65 declares `operator==(const Color&, const Color&)` for an unscoped enumeration. OVL-10 says the built-in `==` stays. I had to work out myself that the two then tie, since a reference binding and a plain identity rank the same, and that the comparison is ambiguous. I answered reject.
- Suggestion: Add to OVL-10 that the kept built-in candidate and such a function tie, and that `red == blue` is then ambiguous.

## RPA-7

- Pages and rules: overload-resolution, OVL-7; names page
- Kind: hard to find
- What happened: p109 calls `operator==(a, b)` by name inside a member function. OVL-7 says a member can hide a non-member in that form. I also needed to know that argument-dependent lookup does not add `::operator==(Key, Key)` once ordinary lookup finds a class member. The page does not say that, so I relied on outside knowledge. I answered reject.
- Suggestion: In OVL-7, add that argument-dependent lookup does not run when ordinary lookup finds a class member, and link the names-page rule.

## RPA-8

- Pages and rules: overload-resolution, function-reference grid, OVL-15, OVL-27 to OVL-29 (Q-0138, Q-0148)
- Kind: hard to find
- What happened: p66, p89, p113, p140, and p161 bind a reference to function to one of two conversion functions. The prose bullets are general, so I had to read each grid cell to see which compiler picks which function. Those cells list eight compilers on two sides and use short labels such as "plain rref / template lref". p66 (two templates) and p161 took several readings.
- Suggestion: Add a short table with one row per program shape (plain or template, lref or rref, lvalue or rvalue target) that names the function each compiler family picks.

## RPA-9

- Pages and rules: overload-resolution, OVL-2; ICS-41
- Kind: needs an example
- What happened: p46 calls an explicit-object member by its unqualified name as `take(*this, 1)`. OVL-2 says `*this` meets the explicit object parameter. I worked out that `*this` then also counts as an extra argument, so the call passes two arguments to one remaining parameter. The calls grid confirmed the implied object, but no rule shows the case of a caller who also passes the object explicitly. I answered reject.
- Suggestion: Add an example to OVL-2 in which an explicit-object member called by name inside a member function takes `*this` implicitly, and show that passing `*this` again gives one argument too many.

## RPA-10

- Pages and rules: overload-resolution, OVL-15; constant expressions
- Kind: outside knowledge
- What happened: p48 uses `&left == &a` in an array bound, where `left` and `right` are references bound through conversion functions. The outcome depends on whether those references can be used in a constant expression. The overload page decides the binding, and I had to rely on outside knowledge for the constant-expression part. I answered compiles.
- Suggestion: None for this page. A test that mixes in constant-expression rules could cite the rule that makes such a reference usable.
