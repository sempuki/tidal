# Predict batches: translation, values, and conversions, reader A

- Date: 2026-11-04
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R9-translation (88 programs), R9-values (64), R9-conversions (89)

All three answers files were written:
- /home/ryanm/Code/tidal/build/exercises/R9-translation/answers.json
- /home/ryanm/Code/tidal/build/exercises/R9-values/answers.json
- /home/ryanm/Code/tidal/build/exercises/R9-conversions/answers.json

I answered `depends` where the compilers differ by an implementation-defined limit or choice, or where an Evidence segment records a split that the rules allow:
- translation: p04, p09, p11, p12, p15, p22, p23, p25, p38, p49, p56, p58, p61, p64, p67, p69, p72, p78
- values: p24
- conversions: p15, p23, p64, p72

Where one compiler differs from a settled rule, I answered with the rule's outcome. These are translation p02, p48, p60, p74, and p80 (TR-3, Q-0366), values p35 (Q-0034), and conversions p26 (Q-0042) and p43 (Q-0100).

## R9A-1

- Pages and rules: foundations/translation.md, TR-13
- Kind: needs an example
- What happened: Translation p09 nests 100 instantiations of one class template. p15 and p55 alternate between two templates, Even and Odd, 1000 and 100 deep. The TR-13 table says EDG limits "the nested instantiations of one template" to 64. Clang and GCC count all templates together. I read the EDG limit as a count per template. So I predicted that EDG rejects p09 and p15 (100 and 500 per template) and accepts p55 (50 per template). The page has no example that shows how a chain through two templates counts on each compiler.
- Suggestion: Add an example to TR-13 with a chain through two class templates. Give the depth that each compiler counts for it.

## R9A-2

- Pages and rules: foundations/translation.md, TR-15, Q-0350
- Kind: unclear; I needed outside knowledge
- What happened: Translation p04, p22, p38, and p61 define large classes on x86-64 and check `sizeof` with a `static_assert`. The TR-15 table says Clang 18 has "no limit", and Q-0350 says Clang gives `sizeof` "a wrong value" for a class "whose size it cannot represent". The page does not say which sizes Clang 18 can represent. I assumed that Clang counts sizes in bits in a 64-bit value. On that assumption, 2^61 bytes and more give a wrong `sizeof`, so the `static_assert` fails on Clang 18 for p04, p38, and p61, and passes for p22. That assumption came from outside the specification. The same gap applies to Clang with `-m32` (p23, p25).
- Suggestion: In TR-15 or its Evidence, give the largest size for which Clang 18 and Clang with `-m32` still give the right `sizeof`.

## R9A-3

- Pages and rules: the batch README, foundations/translation.md, TR-4, LNK-7
- Kind: unclear
- What happened: Translation p45, p46, and p53 have no `main`. The README says a program with no `main` can be answered `compiles`. The TR-4 note says every compiler that links reports a missing `main` at link time. I could not tell which tests link their program. I guessed `link-fail` for p46 (only a helper function, which looks like the missing-`main` test) and `compiles` for p45 and p53.
- Suggestion: Say in the README, or in the TR-4 note, how a reader can tell whether a test with no `main` is linked.

## R9A-4

- Pages and rules: foundations/values.md, VC-P3, VC-9
- Kind: wrong or contradictory
- What happened: VC-P3 lists "the initialization of each array element" as a separate full expression. VC-9 says "All initializers of one aggregate are one full expression". The only exception it gives is an array whose elements a constructor with a default argument initializes. In values p22 (`int arr[2] = {read(T{1}), read(T{2})}`), VC-P3 gives the trace 1929 and VC-9 gives 1299. I followed the rule and predicted 1299, which the program expects. p45 (a default argument per element) follows both texts.
- Suggestion: Narrow the VC-P3 bullet so that it covers only elements that a constructor initializes one by one. Or add the aggregate case to the principle as its own bullet.

## R9A-5

- Pages and rules: foundations/values.md, VC-10; foundations/constant-evaluation.md, DTOR-7 row
- Kind: hard to find
- What happened: Values p24 has a callee that destroys its parameter during constant evaluation, and then the caller destroys the parameter again. VC-10 says this is a second destruction under DTOR-7. VC-10 does not say what the compilers do in constant evaluation. I found the answer only in the undefined-rule table on the constant-evaluation page, where EDG accepts `parameter-destroyed-by-callee-constant.cpp`. So I answered `depends`.
- Suggestion: Add one line to the VC-10 Evidence. The line should say that Clang and GCC reject the second destruction in constant evaluation and EDG accepts it.

## R9A-6

- Pages and rules: foundations/conversions.md, CONV-17, CONV-P3; overloading/conversion-sequences.md, ICS-8
- Kind: hard to find
- What happened: Conversions p35 (private base) and p86 (ambiguous base) call an overload set of `take(Base*)` and `take(...)`. CONV-P3 says a conversion through such a base "is not implicit". From that sentence, a new reader expects that the ellipsis overload is chosen and that the program compiles. ICS-8 on another page says that the sequence still forms, the pointer overload wins, and the program is rejected. I answered reject from ICS-8. The conversions page does not link to ICS-8.
- Suggestion: In CONV-17, add a sentence that the conversion still takes part in overload resolution, and link to ICS-8.

## R9A-7

- Pages and rules: foundations/conversions.md, CONV-21, Q-0331; declarations/initialization/list.md Evidence
- Kind: hard to find
- What happened: Conversions p72 brace-initializes an `unsigned char` from an unscoped enumeration with the values 0 and 100. CONV-21 says the enumeration's values are those of ENUM-22, so the conversion is not narrowing. The fact that Clang and EDG reject the program and GCC accepts it appears only in the Evidence of the list-initialization page. The conversions page Evidence does not mention it. I answered `depends`.
- Suggestion: Add the Q-0331 split to the conversions page Evidence, next to Q-0100.

## R9A-8

- Pages and rules: foundations/translation.md, TR-3, Q-0366
- Kind: unclear
- What happened: Translation p02 and p80 declare `main` with `extern "C++"`. Q-0366 records Clang 22 and GCC 16 accepting and EDG rejecting. The note on TR-3 says Clang 18 and GCC 14 accept the C language linkage forms. It does not say what they do with C++ language linkage. I answered `runs` by the rule.
- Suggestion: Name the result of Clang 18 and GCC 14 in the Q-0366 divergence line.

## R9A-9

- Pages and rules: declarations/enumerations.md, the `enum_head` production
- Kind: missing rule text
- What happened: Translation p05 declares `enum class { first, second } value;`. Only the grammar production shows that a scoped enumeration needs a name. No rule sentence says it. I answered reject under TR-12.
- Suggestion: Add a sentence to an ENUM constraint saying that a scoped enumeration must have a name. TRY-5 already does the same for its own grammar.
