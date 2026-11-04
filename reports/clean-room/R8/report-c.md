# Predict batches: dependent names and storage, reader C

- Date: 2026-11-04
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R8-dependent-names (127 programs), R8-storage (93 programs)

Both answers files were written:
- /home/ryanm/Code/tidal/build/exercises/R8-dependent-names/answers.json
- /home/ryanm/Code/tidal/build/exercises/R8-storage/answers.json

I answered `depends` wherever an Evidence segment records that the compilers differ:
- dependent names: p19, p29, p32, p34, p41, p46, p48, p54, p60, p65, p80, p85, p88, p91, p98, p101, p103, p106, p108, p115, p118, p120, p121
- storage: p09, p43, p55, p56, p79

## R8C-1

- Pages and rules: templates/dependent-names, DEP-30; classes/virtual, VIRT-3
- Kind: hard to find
- What happened: p29 is an unused class template with a base that is not dependent. It has a member marked `override` that overrides nothing. The dependent-names page does not mention `override`, and DEP-30's Evidence lists only an invalid conversion, an invalid call, and undeclared names. I found the split (Clang rejects, GCC and EDG accept) only in the VIRT-3 Evidence on the virtual page. I answered `depends`.
- Suggestion: Add the `override` case to the DEP-30 bullet in the Evidence, or link to the VIRT-3 note from there.

## R8C-2

- Pages and rules: templates/dependent-names, DEP-30; declarations/functions/default-arguments, DEFARG-10
- Kind: hard to find
- What happened: p121 is an unused function template whose default argument `int* p = 1.5` uses no parameter. The DEP page does not say whether such a default argument counts as a construct in the template's body. INST-9 says default arguments are instantiated only on use, which suggested the opposite answer. The split (GCC accepts, Clang and EDG reject) is only in the DEFARG-10 Evidence. I answered `depends`.
- Suggestion: In DEP-30, say that an independent default argument is such a construct, and link DEFARG-10.

## R8C-3

- Pages and rules: DEP-17, DEP-30
- Kind: unclear
- What happened: The DEP-17 Evidence gives the compiler split for "a member access through `this` that finds nothing" (GCC 14 and Clang 18 accept the unused template). Its example is a call, `this->f`. p65 is a call, `this->missing()`. p80 is a data member read, `this->missing`, in a primary template whose only use is an explicit specialization that declares `missing`. I assumed the same split for both and answered `depends`, but I could not confirm that the data member form and the explicit specialization change nothing. The current-instantiation grid covers only qualified names.
- Suggestion: Add a row for `this->name` to the current-instantiation grid, or state in the bullet that the split holds for data members too. Say that an explicit specialization declared later does not save the primary template.

## R8C-4

- Pages and rules: foundations/storage, STG-19, STG-9; the Split tests table
- Kind: unclear
- What happened: p07 constructs a `Base` in the base subobject of a `Derived` during constant evaluation and then reads through the old base pointer. STG-19 makes the read undefined, so the Split tests table expects `reject`. The same row shows that Clang, GCC, and EDG all accept. I could not tell whether to predict the rules or the compilers. I answered `compiles`, because every compiler accepts. p30, p32, and p75 raise the same doubt from the other side. The Evidence says that each of their tests "checks only that the program compiles", and I answered `undefined` from the rules.
- Suggestion: For each such test, give the outcome that the test expects in the README's terms (`compiles`, `undefined`, or `reject`).

## R8C-5

- Pages and rules: CPY-27, STG-12, UNI-3
- Kind: needs an example
- What happened: p08 asserts that a union wrapping a class with a `volatile int` member keeps all its special members. The answer depends on whether a volatile member makes copying non-trivial. CPY-27 settles the question only by leaving volatile out of its list. A reader with outside knowledge might expect volatile to matter. I answered `runs`.
- Suggestion: Add one sentence to CPY-27 saying that a volatile member does not make a copy or move non-trivial.

## R8C-6

- Pages and rules: STG-22, STG-9, UNI-5
- Kind: missing
- What happened: p04 uses `construct_at` on one union member during constant evaluation and then reads through a reference to the member that was active before. No rule or Evidence item covers this case directly. I reasoned that the new member ends the old one (STG-6), and that the read is outside a lifetime (STG-22), so constant evaluation rejects it. I answered `reject`.
- Suggestion: Add an example or an Evidence line for a new object in another union member and a read through the old member's reference.

## R8C-7

- Pages and rules: STG-12, INIT-1
- Kind: unclear
- What happened: STG-12 says that a byte copy "through a `char` glvalue needs a value in each byte, padding bytes included". p26 copies a struct with padding through `unsigned char`. I could not tell whether "char" in STG-12 means only plain `char` or all three byte types. INIT-1 says that a copy into `unsigned char` is not undefined, so I answered `runs`.
- Suggestion: Write "a plain `char` glvalue" in STG-12, and say that `unsigned char` and `std::byte` copies of padding are valid (INIT-1).

## R8C-8

- Pages and rules: STG-8
- Kind: unclear
- What happened: p54 calls a class's own `operator new` by name and uses the result as a `Sample`. STG-8 says "a call of an allocation function, by name". It does not say whether a class allocation function counts. Here the class function forwards to `::operator new`, so I answered `runs` either way, but a class function that returns a byte array's address would not be settled.
- Suggestion: State whether a class allocation function creates objects implicitly.

## R8C-9

- Pages and rules: specification/index.md (Baseline compilers); STG-18, STG-23 Evidence
- Kind: wrong
- What happened: The skip table in index.md lists only EDG as skipping `-fsanitize=...`. The storage Evidence says that GCC and Clang 18 have no type sanitizer and skip the `-fsanitize=type` tests. For p34, p45, p48, p59, and p66 I followed the storage page and took the outcome from Clang 22 alone. The index also lists GCC 16 as skipping `-m32`, and I could not tell whether host GCC (which is GCC 16) skips it too. That question matters for p55.
- Suggestion: Add a row for `-fsanitize=type` (skipped by GCC 14, GCC 16, host GCC, Clang 18, and EDG) to the index table, and say whether host GCC skips `-m32`.

## R8C-10

- Pages and rules: DEP-9, DEP-10
- Kind: missing
- What happened: p61 tests a dependent call inside a requires expression, in an `if constexpr` condition and in a variable template. DEP-9 does not say that a requires expression uses the same two lookups. I assumed that it does, so argument-dependent lookup finds `lib::use` and does not find the later global `use(other::B)`. I answered `runs`.
- Suggestion: Add a sentence to DEP-9, or an example, for a dependent call inside a requires expression.

## R8C-11

- Pages and rules: DEP-1, DEP-5
- Kind: needs an example
- What happened: In p15 a block variable has the type of a member alias of the current instantiation whose target is `shop::Item`, and the program calls `pick(i)`. Only the last sentence of DEP-1 says that such an alias is not dependent. From that sentence the call is independent, binds at the definition, and returns 1. I found the sentence only on a second reading. I answered `runs`.
- Suggestion: Add an example under DEP-P1 or DEP-5 where a call's argument has a member alias type with an independent target, and show that the call binds at the definition.
