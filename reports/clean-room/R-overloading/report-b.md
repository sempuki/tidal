# Predict batches: overloading, reader B

- Date: 2026-11-03
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R-conversion-sequences (129 programs)

The answers file was written:
- /home/ryanm/Code/tidal/build/exercises/R-conversion-sequences/answers.json

The outcomes are 88 compiles, 14 reject, 1 runs, and 26 depends. The Evidence says the compilers agree on 102 of 129 tests, which leaves 27 splits. I found 26, so I probably missed one split.

I answered `depends` wherever the Divergences list or a probe grid records a split. The programs are p10, p15, p16, p19, p27, p34, p37, p38, p48, p55, p65, p68, p72, p74, p75, p76, p79, p84, p86, p88, p98, p111, p112, p115, p120 and p129.

## RPB-1

- Pages and rules: language-support, LSUP-1; conversion-sequences, ICS-16
- Kind: unclear
- What happened: p82 declares an empty `std::initializer_list` template with no members. An overload takes `std::initializer_list<int*>`, and that candidate is not viable because `1` does not convert to `int*`. LSUP-1 requires the full shape only where a braced list initializes an object of that type. I could not tell whether forming (or failing to form) an ICS-16 sequence counts as "needing" the type. I predicted runs.
- Suggestion: In LSUP-1, say whether a candidate whose parameter is `std::initializer_list<E>` needs the shape when the candidate is not chosen, or is not viable.

## RPB-2

- Pages and rules: conversion-sequences, ICS-45, OVL-15; Evidence, Q-0156 and Q-0337
- Kind: hard to find
- What happened: p16, p34 and p38 test `static_cast` to reference types from classes with `explicit` conversion functions. The splits sit in two prose sentences of the Divergences list. I had to work out each pair of target and result kind myself. One example is `long&&` from `explicit operator int()`: I could not tell whether Clang's leniency covers a reference that would bind a temporary of another type. I answered depends for all three. I assumed the negative assertions in p16 hold on every compiler.
- Suggestion: Add a small table to ICS-45. Its rows would be the reference kinds (`T&&`, `const T&`, and `U&&` for another type `U`). Its columns would be the result kinds (value, `T&`, `T&&`). Each cell would give the rule's answer and each compiler's answer.

## RPB-3

- Pages and rules: conversion-sequences, ICS-31; Evidence, Q-0153
- Kind: unclear
- What happened: p112 compares `const int* const&`, which binds a temporary, with a value parameter `const volatile int*`. The Divergence entry says two things. GCC and EDG find a tie for the array-bound case, and EDG does not compare a temporary binding with a value parameter. The entry does not say whether GCC applies ICS-31 to added cv-qualifiers in this mixed case. I predicted that only EDG rejects.
- Suggestion: Split the ICS-31 Divergence entry into two lines. One covers the array bound and one covers the reference-against-value case. Each line names every compiler's result.

## RPB-4

- Pages and rules: conversion-sequences, ICS-38; Evidence, Q-0155
- Kind: needs an example
- What happened: In p37, `{"ab"}` goes to `const char(&)[2]` against `const char*`. The Evidence says Clang gives a literal a sequence to an array that is too short. The Evidence does not say whether Clang then picks the array or finds a tie with the pointer. Clang rejects p37 either way, so the answer did not depend on it. A reader still has to reason through ICS-26 to see that.
- Suggestion: State Clang's result for that pair in the ICS-38 Divergence entry.

## RPB-5

- Pages and rules: conversion-sequences, ICS-8, ICS-10, ICS-7
- Kind: needs an example
- What happened: Two places needed rules that have no example on the page. In p57, a doubly braced list goes to a class whose only constructor takes two ints. In p128, a converting constructor `Target(Source&)` competes with `Source::operator Target()`. In p91, the same pair has a `const Source&` constructor parameter. For p57 I had to find the last bullet of ICS-10, which is dense. For p128 against p91, I had to apply ICS-7 with ICS-30 to the object parameter. I predicted p57 compiles, p91 compiles, and p128 is rejected because the sequence is ambiguous.
- Suggestion: Add two examples to the Examples section. One shows `take({{1, 2}})` with a two-int class against an aggregate. The other shows a constructor and a conversion function that tie and give the ambiguous sequence.

## RPB-6

- Pages and rules: conversion-sequences, ICS-13, ICS-37
- Kind: hard to find
- What happened: In p43, a base class has `operator int` and a derived class has `operator long`. The program converts a derived object to `int`. The outcome depends on the two object parameters tying, which follows from the ICS-13 bullet "For a conversion function, the class is the class of the object". That bullet does not say why it matters. I predicted that `operator int` wins.
- Suggestion: In ICS-13, add one sentence that says the bullet makes the object parameters of inherited and derived conversion functions tie. Add a matching line in ICS-37.

## RPB-7

- Pages and rules: conversion-sequences, ICS-8; Evidence, Q-0132
- Kind: unclear
- What happened: ICS-8 lists an `explicit` constructor in copy-list-initialization as a fault that still forms a sequence. The Divergence entry says only GCC does so, and Clang and EDG form no sequence. The page says which compilers ICS-21, ICS-22, ICS-31 and ICS-45 follow, but not which ones ICS-8 follows for this bullet. For p98 I answered depends: GCC rejects, and Clang and EDG compile.
- Suggestion: Say in the ICS-8 Divergence entry that the bullet follows GCC, and that it is provisional under Q-0132.

## RPB-8

- Pages and rules: conversion-sequences, Evidence
- Kind: unclear
- What happened: The Evidence says the eight compilers agree on 102 of 129 tests, so 27 tests split. I found 26 programs with a recorded split. Some splits are recorded only in grid cells and not in the prose, such as the order grid for p72 and the lists grid for p15. A reader cannot check their list against the count.
- Suggestion: List the split tests by name in the Divergences section, or give the count for each question.
