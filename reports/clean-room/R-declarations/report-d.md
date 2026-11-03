# Predict batches: declarations, reader D

- Date: 2026-11-03
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done, all six with answers.json written in each batch directory under /home/ryanm/Code/tidal/build/exercises/:
  - R-enumerations: 59 answers
  - R-exception-specifications: 41 answers
  - R-default-arguments: 23 answers
  - R-definitions: 45 answers
  - R-deleted: 17 answers
  - R-inline: 35 answers

## RDD-1

- Batch and program: R-inline p01
- Rule IDs: INL-1, LNK-4
- What I needed: p01.cpp and p01-2.cpp both include `shared-limit.h`. That header is not in the batch directory. Built as given, both units would fail on the missing header. I guessed the header defines one inline const variable, one plain const variable, and declares an extern const variable. On that guess I answered `runs`.
- Suggestion: Copy the header into the batch directory, or list it in the README table.

## RDD-2

- Batch and program: R-enumerations p19, R-definitions p39
- Rule IDs: ENUM-6, FN-3
- What I needed: Both programs build with the undefined-behavior sanitizer. The README says to answer `run-fail` when the sanitizer stops the operation. Neither page says whether the sanitizer catches an out-of-range enumeration value or the end of a function reached without a return. I guessed `run-fail` for both.
- Suggestion: Add an Evidence line to each rule that says whether the sanitizer stops the operation, and at which point (at the conversion, or at a later load).

## RDD-3

- Batch and program: R-definitions p12, p23, p42 (also p15)
- Rule IDs: FN-6
- What I needed: The Evidence for FN-6 records `(int)`, `(double)`, three parameters, and four parameters, but only partly. For example, it says EDG accepts four parameters and `(double)`. It does not say what Clang and GCC do with them. Nothing covers a `long` first parameter. I answered `depends` and guessed that Clang and GCC reject.
- Suggestion: Give a small table in the Evidence with each tested parameter list and each compiler's result.

## RDD-4

- Batch and program: R-definitions p34
- Rule IDs: FN-6, FN-8
- What I needed: The program takes a third `char**` parameter and checks that it is not null. The page says all compilers accept three parameters. It does not say what the third parameter receives. I assumed a non-null pointer.
- Suggestion: Say what an implementation passes in a third parameter of `main`, or say that the value is implementation-defined.

## RDD-5

- Batch and program: R-enumerations p36
- Rule IDs: ENUM-2
- What I needed: The program adds values of two different unscoped enumerations. ENUM-2 says each value promotes to an integer. The page says nothing about mixing two enumeration types in one arithmetic operation. I answered `compiles`.
- Suggestion: Say whether arithmetic between two different enumeration types is valid, and what the compilers do with it under `-pedantic-errors`.

## RDD-6

- Batch and program: R-enumerations p50
- Rule IDs: ENUM-10
- What I needed: ENUM-10 says an enumerator with an initializer has "the type of the initializer, without promotion" inside the list. In p50 the initializer is a `const short` variable. The rule does not say whether the `const` stays in that type. I assumed the `const` is dropped, so the type is `short`.
- Suggestion: State that cv-qualifiers do not carry into the enumerator's type.

## RDD-7

- Batch and program: R-inline p15
- Rule IDs: DEL-2, INL-3
- What I needed: The program declares a deleted function and then declares it again without `= delete`. DEL-2 forbids a second definition and requires the deletion on the first declaration. DEL-2 does not say whether a later plain declaration is valid. I answered `compiles`.
- Suggestion: Add a sentence to DEL-2 saying that later declarations without a body are valid.

## RDD-8

- Batch and program: R-exception-specifications p20
- Rule IDs: EXS-2, CONV-10, ICS-26
- What I needed: Two overloads take a plain function pointer and a noexcept function pointer. The argument is a noexcept function. I found the answer in the conversion-sequences page: dropping noexcept is an extra step, so the noexcept overload's sequence is a proper subsequence and wins. The exception-specifications page does not point there.
- Suggestion: Add a cross-reference from EXS-2 to the rule that ranks a sequence that drops noexcept below one that keeps it.

## RDD-9

- Batch and program: R-default-arguments p10
- Rule IDs: DEFARG-8
- What I needed: The program has several parameter packs whose element types come from deduction, such as a pack of function pointers. These programs mostly test pack deduction. The default-arguments page covers only the last function, and I had to assume the deduction works.
- Suggestion: Move the pack-deduction cases to the templates batch, or name the deduction rules the program depends on.
