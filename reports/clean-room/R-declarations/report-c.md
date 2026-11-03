# Predict batches: declarations, reader C

- Date: 2026-11-03
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done, in order: R-constexpr (69 programs), R-attributes (72), R-alignment (35), R-static-assert (29), R-volatile (37).

Each batch's answers are in `answers.json` in its own directory under `/home/ryanm/Code/tidal/build/exercises/`. All five files were written, and each one parses as JSON with one entry per program. I read only the batch directories and their `specification/` copies.

Where the specification records a compiler split, I answered `depends` and named the compilers. There is one exception. For the Clang 18 `assume` cases (R-attributes p02, p04, p18, p56, p64), I gave the outcome of the compilers that implement `assume` and mentioned Clang 18 in `why`.

## RDC-1

- Batch and program: R-constexpr p53
- Rule IDs: CINIT-1, CINIT-2
- What I needed: a rule that says `constinit` applies only to variables. CINIT-2 says a `constinit` variable must have static or thread storage duration. It does not say what happens when `constinit` is on a function declaration. CEV-3 says this directly for `consteval`. I answered reject and cited CINIT-2.
- Suggestion: add a sentence to CINIT-2, "`constinit` applies only to variables."

## RDC-2

- Batch and program: R-constexpr p25
- Rule IDs: CX-2, CE-3, CE-14
- What I needed: whether every compiler accepts a namespace-scope `constexpr` aggregate whose reference member binds to a temporary. The CX-2 Evidence says all compilers accept this. The CE Evidence says EDG rejects a namespace `constexpr` reference when the temporary is extended "through a reference member of a braced aggregate" (Q-0193). I could not tell whether the two notes describe the same program. I answered `runs`.
- Suggestion: in the CE Evidence, say which form of declaration EDG rejects. One option is a `constexpr` reference variable that binds through an aggregate. The other is a `constexpr` aggregate variable. Then add a cross-reference from the CX-2 note.

## RDC-3

- Batch and programs: R-constexpr p40, p44, p49
- Rule IDs: CX-3
- What I needed: the CX-3 Evidence says Clang 18 rejects a `constexpr` function that never gives a constant, and a non-literal parameter type. CX-3 also allows a non-literal return type, and the Evidence does not mention return types. For p44 (a non-literal return type in a function that can never be constant), I assumed Clang 18 rejects it and answered `depends`.
- Suggestion: say in the Evidence whether Clang 18 also rejects a non-literal return type.

## RDC-4

- Batch and programs: R-constexpr p01, p03, p54
- Rule IDs: CEV-1
- What I needed: CEV-1 says a lambda, or a `constexpr` function from a template, becomes `consteval` when it calls a `consteval` function with arguments that are not constant. The Evidence does not say whether the older baseline compilers (Clang 18, GCC 14) implement this. I answered as CEV-1 says, with no split.
- Suggestion: add one Evidence line that says whether all eight compilers support this, or record the split in a question.

## RDC-5

- Batch and program: R-attributes p50
- Rule IDs: ATTR-12
- What I needed: `[[noreturn]] int main() { for (;;) {} }` compiles, and the README says to run every program that links. The run never ends, and the README's list of outcomes has no category for a program that does not stop. I answered `compiles`.
- Suggestion: say in the README whether a test like this one is compile-only, or add an outcome for a program that does not terminate.

## RDC-6

- Batch and programs: R-attributes p10, p66
- Rule IDs: ATTR-6
- What I needed: ATTR-6 says an implementation should warn about a variable that is only assigned, and about an unused structured binding declaration. The Evidence confirms the compiler behavior for class types, condition variables, and range-`for` variables. It does not confirm these two cases. I answered reject for both, assuming Clang and GCC warn under `-Wunused`.
- Suggestion: add Evidence lines for the only-assigned variable and the unused structured binding.

## RDC-7

- Batch and programs: R-alignment p11, p26
- Rule IDs: ALN-3
- What I needed: ALN-3 says the definition must have "the same alignment" as an earlier declaration that has `alignas`. It also says a definition can add `alignas`. It does not say whether a definition that leaves out `alignas` after a declaration that has one counts as a mismatch, or inherits the alignment. I treated it as a mismatch and answered `depends` (GCC accepts, per Q-0070).
- Suggestion: state the case directly. For example: "If any declaration has `alignas`, the definition must have an `alignas` with the same alignment."

## RDC-8

- Batch and program: R-alignment p18
- Rule IDs: ALN-6
- What I needed: ALN-6 is class no-diagnostic, and every compiler accepts two units that declare different alignments for one variable. The page does not say what the program does at run time, or whether the behavior is undefined. I answered `runs`, from what the compilers do.
- Suggestion: say whether a program that breaks ALN-6 has undefined behavior or just runs with one of the alignments.

## RDC-9

- Batch and program: R-volatile p25
- Rule IDs: VOL-11, VOL-9
- What I needed: in p25 a non-volatile `S` is assigned to a `volatile S&`. VOL-11 explains the rejection by saying the implicit copy functions take `const T&`, which does not bind to a volatile object. That explanation covers a volatile source. In p25 the source is not volatile and the target is. The rejection here comes from the implicit assignment operator not being a volatile member function, so VOL-9 applies. I had to put the two rules together myself. I answered reject.
- Suggestion: in VOL-11, say that the implicit assignment operator is also not `volatile`, so it cannot be called on a volatile target.

## RDC-10

- Batch and programs: R-volatile p02, p23, p26
- Rule IDs: CV-9
- What I needed: CV-9 says to warn about a `volatile` return type and a `volatile` parameter type. It does not say that only a top-level `volatile` counts, and that a pointer to volatile (`volatile int*`) does not. I answered `compiles` for p23, which uses only pointers to volatile.
- Suggestion: add an example to CV-9 that contrasts `volatile int f()` with `volatile int* f()`.
