# Predict batches: classes, reader D

- Date: 2026-11-03
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R-unions (122 programs), R-virtual (119 programs)

I wrote both answer files:

- /home/ryanm/Code/tidal/build/exercises/R-unions/answers.json
- /home/ryanm/Code/tidal/build/exercises/R-virtual/answers.json

Some answers are `depends` because the Evidence segments record that the compilers disagree:

- **R-unions:** p03, p21, p74, p78, p81, p102, p103, p116.
- **R-virtual:** p27, p72, p76, p83.

The specification did not settle six answers, so I gave a best guess for each:

- **R-unions:** p53, p107.
- **R-virtual:** p19, p47, p91, p88.

The findings below cover these, plus the link-fail answers in R-virtual (p90 and p119), where the outcome for EDG is unclear.

## RCD-1

- Batch and program: R-unions p53 (built with `-Werror` but no `-Wall`). Also p46, p72, p30.
- Rules: BF-8, BF-10.
- What I needed: p53 stores the constant 10 in a 3-bit field. BF-8 says an implementation should warn. The Evidence says Clang and GCC warn "under `-Wall`". It does not say whether the warning is also on without `-Wall`. That fact decides whether `-Werror` alone rejects p53. I guessed reject. I also needed to know whether this warning ever becomes an error under `-pedantic-errors` (p30 and p72). I assumed it does not.
- Suggestion: In the BF-8 Evidence, say for each compiler whether each of the two warnings is on by default or needs `-Wall`, and say that `-pedantic-errors` does not turn them into errors.

## RCD-2

- Batch and program: R-unions p107.
- Rules: BF-10, BF-5.
- What I needed: The program stores 5 in a signed 3-bit field and expects -3. BF-10 says the value "is reduced modulo 2^n". For an unsigned field the meaning is clear. For a signed field, 5 modulo 8 is 5, and 5 does not fit. The reader has to assume the result is the value in the field's signed range that matches modulo 2^n. I answered runs on that assumption.
- Suggestion: Say in BF-10 that for a signed bit-field, the stored value is the one that fits the field's range (the range of an n-bit two's complement value) and has the same remainder as the source value when both are divided by 2^n. Add one example with a negative result.

## RCD-3

- Batch and program: R-virtual p19 and p47 (both with `-fsanitize=address -fsized-deallocation`).
- Rules: VIRT-12. NEW-40 is related.
- What I needed: Both programs are undefined under VIRT-12. p19 is `delete[]` through a base pointer. p47 is `delete` through a base without a virtual destructor. The README asks for run-fail when a sanitizer stops the operation. The VIRT-12 Evidence says nothing about any sanitizer. The NEW-40 Evidence says the address sanitizer catches allocation mismatches, but it does not cover these cases. I guessed run-fail for both.
- Suggestion: Add a line to the VIRT-12 Evidence. It should say whether the address sanitizer with sized deallocation stops each case, and what it reports.

## RCD-4

- Batch and program: R-virtual p91 (built with the undefined sanitizer).
- Rules: VIRT-12.
- What I needed: A constructor reaches a pure virtual function through a virtual call. The specification calls this undefined. For constant evaluation, the Evidence mentions a run-time support function that a pure call reaches. It does not say what that function does in a real run. I needed to know whether the run stops with a nonzero status, and whether the sanitizer has a part in it. I guessed run-fail.
- Suggestion: Add to the VIRT-12 Evidence what Clang and GCC do at run time for this call. If the program prints a message and aborts with or without a sanitizer, say so.

## RCD-5

- Batch and program: R-virtual p88, p34, p99 (built with `-Wall -Werror`).
- Rules: VIRT-13.
- What I needed: The Evidence says "Clang and GCC give both warnings under `-Wall`". The README says the matrix includes old compiler versions. As far as I know, older GCC releases do not include the hiding warning in `-Wall`. The specification does not say which versions it means. I answered reject for p88 from the specification.
- Suggestion: Name the compiler versions in the VIRT-13 Evidence. If an older GCC does not give the hiding warning under `-Wall`, record that disagreement.

## RCD-6

- Batch and program: R-virtual p90 and p119. R-unions p03 is related.
- Rules: VIRT-17, VIRT-20, BF-9.
- What I needed: For a missing definition, the Evidence says Clang and GCC fail at link time and that "EDG does not link". I could not tell whether EDG compiles the program and stops there, or whether the matrix never links EDG builds. That changes whether the answer is link-fail or depends. I answered link-fail. For p03, I could not tell whether the EDG i686 configuration is part of the `-m32` run. I answered depends.
- Suggestion: Add a sentence to the Evidence format, or the README, that says what "EDG does not link" means for a predicted outcome. Also say which compilers run each build flag such as `-m32`.
