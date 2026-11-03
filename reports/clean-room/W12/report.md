# W12 report: callbacks

- Date: 2026-11-03
- Role: clean-room reader
- Persona: Haskell programmer
- Programs: counter.cpp, sort.cpp, compose.cpp, dispatcher.cpp

All four programs build without diagnostics on g++ 16.2.1 and clang++ 22.1.8 with `-std=c++23 -pedantic-errors -Wall -Wextra`. Each one exits 0, and each one also exits 0 under `-fsanitize=undefined,address`. Every program checks its results with `static_assert` and with `main`'s return value. None of them needed a language-support type, so no program declares anything in `namespace std`.

## Summary

The three biggest problems:

1. **A forwarding `decltype(auto)` wrapper can dangle, and no page says so (W12-1).** I wrote `compose` as RET-2, AUTO-10 and PACK-P3 suggest. Composing it with a forwarding identity function returned a reference to a temporary that had already ended. Neither compiler warned in a normal build. Constant evaluation rejected the program, and AddressSanitizer caught it at run time.
2. **The lambdas page never says a closure can be copied (W12-2).** LAM-11, LAM-26 and the LAM-P4 table cover default construction and assignment only. Every callback program copies closures: into by-value parameters, into captures, and into variables.
3. **No page shows how to store callables of different types together (W12-4).** LAM-2 gives each lambda its own type, so a dispatcher needs type erasure. I had to put the design together from VIRT, NEW-P4, CE-1, CE-4 and the "literal type" glossary entry.

## Construct-to-rule notes (made before writing)

**counter.cpp**
- Init capture `[k = 0]`: LAM-7. Changing the copy needs `mutable`: LAM-3, LAM-23.
- A counter shared by reference and changed without `mutable`: LAM-3, LAM-4. A snapshot by copy: LAM-4, LAM-P2.
- A factory that returns a closure: RET-1, RET-2, LAM-2.
- Closure copies: LAM-26 forbids only assignment and default construction.
- Calls inside `static_assert`: LAM-12, CE-1, CE-5.
- Types of captures: `decltype((x))` is a reference to `const` in a lambda that is not `mutable` (LAM-18, DTY-1, DTY-2).
- My own `same` trait: a variable template with a partial specialization, copied from the AUTO page example.

**sort.cpp**
- Comparator as a by-value template parameter. A function name decays to a pointer: DED-6. Calling it: PFX-17, PFX-7.
- A lambda with no captures converts to a function pointer: LAM-10.
- Generic comparator: LAM-15, DED-34. Capturing comparator: LAM-5, LAM-9.
- Sort by a key named by a pointer to data member: MPTR-2, MPTR-3.
- A pointer to member function as the comparator: MPTR-3, with the parentheses that MPTR-10 requires.

**compose.cpp**
- `compose` returns a closure that holds copies of f and g: RET-2, LAM-1, LAM-7.
- The `auto&&` parameter, forwarded with `static_cast<decltype(x)&&>`: LAM-15, AUTO-4, DED-8, PACK-P3, PACK-24.
- `decltype(auto)` keeps a reference result: RET-2, AUTO-10, DTY-2.
- Variadic `compose_all`: the pack rules.
- A field accessor from a pointer to data member: MPTR-3.

**dispatcher.cpp**
- Type erasure through a virtual base `Handler` with a template `Holder<F>`. CE-1 permits virtual calls and `new`/`delete` in constant evaluation. NEW-6 and CE-7 require every allocation to be freed. The holder is a literal type (glossary).
- A `void*` scheme is ruled out for `static_assert` by CE-4.
- Subscribing a member function: MPTR-2, MPTR-3, MPTR-10, with the object captured by reference (LAM-5).
- Handlers with reference captures: LAM-3. LAM-19 covers dangling captures.

## Findings

### W12-1. A forwarding `decltype(auto)` result can refer to a dead temporary
- Pages and rules: return-deduction (RET-P1, RET-2, RET-6); auto (AUTO-10); packs (PACK-P3); objects (OBJ-9, OBJ-2).
- Kind: needs example.
- What happened: `compose` returned `f(g(forwarded x))` with `decltype(auto)`. I composed it with an identity lambda that takes `auto&&` and returns `decltype(auto)`. `g(x)` made a prvalue `int` and identity bound to it, so the identity's result became `int&&` and so did compose's. The temporary ends at the end of compose's `return` statement. Constant evaluation rejected the call: GCC said "accessing outside its lifetime", and Clang said "read of temporary whose lifetime has ended". In a run-time-only probe, neither compiler warned, and the plain build exited 0. AddressSanitizer reported stack-use-after-return on both compilers.

  The pages had all the pieces. AUTO-10 says an xvalue gives an rvalue reference. OBJ-9 says a temporary ends with its full expression. RET-P1 warns only about local variables. PACK-P3 presents the forwarding cast together with `decltype(auto)` and says nothing about this risk.
- Suggestion: add to RET-P1, or to the Examples of return-deduction or lambdas, a case where a forwarding wrapper returns an rvalue reference to a temporary that a call inside the wrapper created. Say that a plain `auto` return avoids the problem when the caller does not need a reference.
- Fix: `id` now takes and returns its argument by value. A comment in compose.cpp records the trap.

### W12-2. The lambdas page does not say closures can be copied
- Pages and rules: lambdas (LAM-P4, LAM-11, LAM-26).
- Kind: missing.
- What happened: the LAM-P4 table has the column "default-constructed and assigned", and LAM-26 forbids both for capturing closures. No rule says a closure can be copy-constructed or moved. Every callback program here needs that: sort takes the comparator by value, compose captures f and g by copy, and counter.cpp copies a stateful closure. I assumed copying works because the page does not forbid it. A probe confirms that copying a capturing closure compiles on both compilers and assignment is rejected.
- Suggestion: state in LAM-P4 and in a rule that every closure type can be copied and moved when its captures can, and that the copy starts with the state of the original. Add a column to the LAM-P4 table.

### W12-3. Do explicit captures named only in unevaluated operands capture anything?
- Pages and rules: lambdas (LAM-18, LAM-20); decltype (UNEV-1).
- Kind: unclear.
- What happened: LAM-18 ends with "names `x` only in unevaluated operands, which capture nothing". I read this as saying an explicit `[x]` used only inside `decltype` captures nothing. My first counter.cpp did exactly that, and Clang warned `-Wunused-lambda-capture` twice. A probe with a large captured struct shows that the explicit capture still adds a member: `sizeof` of the closure is at least the struct's size on both compilers. The spec does not mention the warning.
- Suggestion: say that the "capture nothing" clause concerns implicit capture through a default, and that an explicit capture always makes a member. Note that Clang warns about an explicit capture the body does not use in an evaluated way.
- Fix: the lambdas in counter.cpp now also read and change `x` in evaluated code.

### W12-4. Storing callables of different types together
- Pages and rules: lambdas (LAM-2, LAM-10); virtual pages; new-delete (NEW-P4, NEW-6); constant evaluation (CE-1, CE-4); glossary "literal type".
- Kind: needs example.
- What happened: a dispatcher has to keep closures of different types in one table. LAM-2 makes each closure type unique, and the language-support page offers no callable wrapper. A function pointer table only takes lambdas without captures (LAM-10, LAM-25). A context pointer through `void*` cannot be checked with `static_assert`, because CE-4 rejects the cast back. I found the working design by combining a virtual base, a template holder, and `new`/`delete` in constant evaluation. No page links these together.
- Suggestion: add a note or example to the lambdas page listing the ways to keep closures of different types together without the library, with a short cross-reference to CE-4 for the `void*` route.

### W12-5. Unary `+` on a lambda
- Pages and rules: unary (UN-1); lambdas (LAM-10).
- Kind: hard to find.
- What happened: I wanted a short way to turn a lambda with no captures into a function pointer. UN-1 lists only arithmetic, unscoped enumeration and pointer operands for unary `+`. A probe `+[](int x){ return x; }` compiles cleanly on both compilers, because the closure converts to a pointer first. Neither the lambdas page nor UN-1 says this.
- Suggestion: add one example line to LAM-10 that shows unary `+` producing the function pointer, and cross-reference the overload-resolution rule that allows it.

### W12-6. Reference collapsing through `decltype`
- Pages and rules: declarators (DCL-4); packs (PACK-24); decltype (DTY-3).
- Kind: hard to find.
- What happened: my forwarding cast writes `decltype(x)&&`. DCL-4, the rule I looked up first, mentions collapsing only "from an alias or a template argument". The `decltype` case appears only in PACK-24 on the packs page.
- Suggestion: add `decltype` to the list in DCL-4, or have DTY-3 point to PACK-24.

### W12-7. The function-pointer conversion during constant evaluation
- Pages and rules: lambdas (LAM-10, LAM-12); constant evaluation (CE-1, CE-2).
- Kind: missing.
- What happened: sort.cpp converts a lambda to `bool (*)(int, int)` inside a `constexpr` function checked by `static_assert`. A probe declares `constexpr int (*p)(int) = [](int x){ return x + 1; };` and calls `p` in `static_assert`. Both compilers accept both. CE-2 allows only `constexpr` functions during constant evaluation. No rule says the conversion function, or the function the pointer reaches, is `constexpr`.
- Suggestion: state in LAM-10 or LAM-12 that the conversion is usable in constant evaluation, and that the resulting pointer can be called there when the call operator could be.

## Attempt log

1. Wrote notes.md with the rule IDs above.
2. counter.cpp, first version (saved as probe/counter-v1-unused-capture.cpp): both compilers built it and it exited 0, but Clang warned `-Wunused-lambda-capture` twice. I had read LAM-18 as allowing a capture used only in `decltype`. Changed the lambdas to use `x` in evaluated code. Clean on both, with and without sanitizers (W12-3).
3. sort.cpp: clean on the first build on both compilers, with and without sanitizers.
4. Probe overloaded-comparator.cpp: passing an overloaded function name as the comparator is rejected by both compilers. This matches DED-19 and is no finding.
5. Probe unary-plus-lambda.cpp: accepted by both compilers (W12-5).
6. compose.cpp, first version (saved as probe/compose-v1-dangling-id.cpp): both compilers rejected the `static_assert` because a temporary was read after its lifetime ended. Probe compose-dangling.cpp ran the same code at run time only: no warning, exit 0, and AddressSanitizer reported stack-use-after-return on both compilers. Changed `id` to return by value. Clean afterwards (W12-1).
7. dispatcher.cpp: clean on the first build on both compilers, with and without sanitizers.
8. Probes capture-only-in-decltype.cpp, default-comparator.cpp, constexpr-pointer-from-lambda.cpp and closure-copy-assign.cpp checked W12-3, LAM-11 with LAM-14, W12-7, and LAM-26. All results agree with the spec, except where the spec is silent as noted.
9. Final rebuild of all four programs: each exits 0 in all four configurations (2 compilers, with and without sanitizers).

## What worked well

- The LAM-P2 capture table and LAM-3 and LAM-4 made the counter variants (by copy, by reference, init capture with `mutable`) easy to get right the first time.
- The MPTR pages are precise. The `(o.*p)(args)` parentheses rule (MPTR-10) and key projection through `T C::*` worked on the first try.
- CE-4 stated plainly that `void*` erasure fails in constant evaluation, which steered the dispatcher to virtual dispatch before I wrote any code. CE-1 confirmed that virtual calls and `new`/`delete` are allowed there.
- PACK-P3 explains the forwarding cast with a table, and it mentions `auto&&` with `decltype(x)`. That is the Haskell programmer's missing piece for generic wrappers.
- Constant evaluation acts as a free UB checker. `static_assert` caught the dangling compose before any sanitizer run did.
- LAM-11 with LAM-14 allows a default comparator written as a `decltype` of a lambda, and both compilers accept it.
