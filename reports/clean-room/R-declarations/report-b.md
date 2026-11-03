# Predict batches: declarations, reader B

- Date: 2026-11-03
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R-forms (32 programs), R-list (60), R-references (62), R-static (20), R-structured-bindings (55).

Each batch's answers are in its directory:
- /home/ryanm/Code/tidal/build/exercises/R-forms/answers.json
- /home/ryanm/Code/tidal/build/exercises/R-list/answers.json
- /home/ryanm/Code/tidal/build/exercises/R-references/answers.json
- /home/ryanm/Code/tidal/build/exercises/R-static/answers.json
- /home/ryanm/Code/tidal/build/exercises/R-structured-bindings/answers.json

All five files are valid JSON. No write was refused.

## RDB-1

- Batch and programs: R-forms p04, p06, p12, p15.
- Rule IDs: INIT-9, CE-2, CX-1, CE-23.
- What I needed: Each program default-initializes a class with no user-written constructor inside a constant evaluation. CE-2 allows calls only to `constexpr` and `consteval` functions. No page says when the constructor that the implementation writes counts as `constexpr`. CX-1 mentions an implicit default constructor only for a `constexpr` variable with no initializer. I answered `compiles` on the assumption that the implicit constructor is usable.
- Suggestion: Add a rule to the constructors page or the `constexpr` page. The rule should say when an implicit or defaulted constructor can run during constant evaluation, and cite it from CE-2.

## RDB-2

- Batch and program: R-forms p32.
- Rule IDs: INIT-1, CE-9.
- What I needed: The program copies an `unsigned char` that has no value into another `unsigned char` during constant evaluation. INIT-1 says this copy is not undefined, so CE-9 does not reject it. The Evidence segment covers only reads of `int`. I answered `compiles` from the text alone, with low confidence.
- Suggestion: State whether constant evaluation accepts this copy, and record what the compilers do. If they reject it, add a reject rule to the constant-evaluation page.

## RDB-3

- Batch and program: R-forms p29.
- Rule IDs: INIT-7, LIST-1.
- What I needed: The program list-initializes a variable of incomplete class type. INIT-7 requires a complete type only for `=` and parentheses. The list page has no matching condition. I answered `reject` by extending INIT-7 to braces.
- Suggestion: Make the complete-type rule cover every form of initialization, or add the condition to the list page.

## RDB-4

- Batch and program: R-list p58.
- Rule IDs: LIST-2.
- What I needed: Narrowing from a non-constant `long` to `int` depends on whether `int` holds every `long` value. The README names an ILP32 EDG target, where the two types have the same range. No page gives the integer sizes of each baseline target. I answered `depends`.
- Suggestion: Give the sizes of the integer types for each baseline target on the foundations page. Note in LIST-2 that the outcome can differ by target.

## RDB-5

- Batch and program: R-list p31.
- Rule IDs: LIST-2.
- What I needed: LIST-2 says an unscoped enumeration without a fixed type counts as an integer source. It does not say which values the source can hold: the enumerators, a range derived from them, or the values of the underlying type. That choice decides whether converting the enumeration to `unsigned char` is narrowing. I answered `reject`.
- Suggestion: State the set of values that LIST-2 uses for such an enumeration, with an example.

## RDB-6

- Batch and program: R-references p33.
- Rule IDs: REF-5.
- What I needed: The program reads a dangling reference to an ended temporary and is built with `-fsanitize=address`. The README asks for `run-fail` when the sanitizer stops the read. The references page records constant evaluation and warnings, but not what AddressSanitizer does here. I guessed `run-fail`.
- Suggestion: Add an Evidence item saying whether AddressSanitizer reports a read through a reference whose temporary ended at the end of a full expression.

## RDB-7

- Batch and program: R-references p52.
- Rule IDs: REF-6, REF-14.
- What I needed: The program binds a `const int&` and an `int&&` from a braced object whose conversion function returns `int&`. The Evidence says GCC binds through the conversion function for a non-class type. For the rvalue reference, that binding would be to an lvalue. I could not tell whether GCC then rejects the program or runs it with a different result. I answered `depends`.
- Suggestion: Say in the REF-6 Evidence item, or in Q-0157, what GCC does for an rvalue reference in this case.

## RDB-8

- Batch and program: R-references p22.
- Rule IDs: REF-15, REF-5.
- What I needed: The program is built with `-Wall -Werror`. REF-15's Evidence says Clang warns about a temporary bound to a reference member of an object from `new`. It does not say whether `-Wall` turns that warning on, or which Clang versions give it. I answered `depends`, with Clang rejecting.
- Suggestion: Name the warning option or group, and the Clang versions that give the warning.

## RDB-9

- Batch and program: R-list p09.
- Rule IDs: ICS-18, ICS-8, LIST-14.
- What I needed: The outcome follows from three rules on two pages. A designated list gets a conversion sequence in any member order (ICS-18). The other argument then picks the overload whose designators are out of order. That fault rejects the chosen call (ICS-8). The list page gives no hint of this. I answered `reject`.
- Suggestion: Add an example to the list page in which overload resolution picks a function whose designated list is out of order, and the call is rejected.

## RDB-10

- Batch and program: R-structured-bindings p54.
- Rule IDs: SB-8, SB-15.
- What I needed: The SB-15 Evidence says Clang and GCC accept two `_` names as an extension and reject them under `-pedantic-errors`. It does not say what EDG does. I answered `reject`.
- Suggestion: Add EDG's result for two `_` names to the Evidence.
