# W5 report: functions

- Date: 2026-11-03
- Role: clean-room reader
- Persona: Go programmer
- Programs: commands.cpp, deleted.cpp, two_values.cpp, recursion.cpp

## Summary

In the final versions, all four programs build without warnings on `g++` 16.2.1 and `clang++` 22.1.8 with `-std=c++23 -pedantic-errors -Wall -Wextra`. All exit 0, both plain and with `-fsanitize=undefined,address`. Two programs needed one fix each.

The three biggest problems:

1. **The tuple protocol and a `const` hidden variable (W5-1).** The pages never say that the tuple protocol looks up `std::tuple_size` for the hidden variable's exact type, including its `const`. I declared `const auto& [lo, hi]` on a class that used the tuple protocol. Both compilers fell back to binding data members and complained that the members were private. A one-line note or example on SB-5 or LSUP-3 would have prevented this.
2. **No declaration shape for `tuple_size` and `tuple_element` (W5-2).** LSUP-3 lists what the specializations give, but it does not give the primary templates' shapes. It leaves out the kind of the index parameter and the type of `value`. Every other language-support type on that page has an example declaration. These two do not.
3. **Deleted functions inside requires expressions (W5-3).** I tested the rejected calls inside a passing program by using a concept. The pages never say directly whether a call that picks a deleted function makes a dependent requirement false. I had to combine CPT-26, CPT-30, UNEV-5 and DEL-1 to get there, and I was guessing.

## Construct-to-rule notes made before writing

commands.cpp (function table)
- Pointer-to-function declarator: DCL-P1, DCL-2, the declarators example `int (*c)(int)`, and the type-names alias example `using Handler = void(int);`.
- Call through a pointer: PFX-7, PFX-17.
- A function name used as a value becomes a pointer: AUTO-2, ICS-P2.
- A pointer carries no default arguments, so a call through it gives every argument: DEFARG-5, DEFARG-P2.
- Unnamed parameter in a handler that ignores an argument: FN-1.
- Constant evaluation of the table needs `constexpr` functions that are defined first: CE-P2, CX-P1, CE-1 (equality of function pointers). `constexpr` functions are inline: INL-3.
- A constant that holds a pointer must point to a function or to a static object: CX-P2.
- `__func__` is a static array, and its size is the name length plus one on the matrix: FN-5, FN-11.
- `main` returns 0 at its end: FN-8.

deleted.cpp (overloads with deleted conversions)
- A deleted overload takes part in resolution, and using it is then an error: DEL-1, DEL-P1, OVL-37.
- `= delete` goes on the first declaration. A later declaration without a body is allowed: DEL-2.
- Ranking: an exact match beats a promotion, and a promotion beats a conversion. Float to double is a promotion: ICS-P2, OVL-P3.
- A plain function beats a template when the sequences tie: OVL-27 (step 2c).
- A deleted converting constructor: faults are checked only on the chosen function: ICS-P1, ICS-8.
- Testing the rejects inside a passing program: CPT-24, CPT-26, CPT-27, CPT-30, CPT-31, UNEV-5.

two_values.cpp (function that returns two values)
- A function cannot return an array: DCL-P5. Return a struct instead.
- `auto` with a braced `return` is rejected: RET-7. So write the struct type as the return type.
- Structured bindings on data members: SB-3, SB-4. On the tuple protocol: SB-5, SB-17, LSUP-3.
- Structured bindings cannot be `constexpr`: SB-15. Check them inside a `constexpr` function instead.
- `decltype(auto)` on a member in parentheses gives a reference: RET-2, AUTO-10, DTY-2.
- Out-parameters through references: DCL-P2.

recursion.cpp (recursion with a deduced return type)
- A recursive call is valid after a `return` that fixes the type: RET-1, RET-3, RET-P2, and the `fact` example.
- All `return` statements must agree: RET-7.
- A forward declaration repeats the placeholder: RET-4.
- A call before the definition is invalid: RET-3.
- A default argument as an accumulator: DEFARG-1, DEFARG-3.
- `decltype` of a call checks the deduced type, since a prvalue gives `T`: DTY-2.
- Limits on recursion and steps in constant evaluation: CE-P5, CE-10.

## Findings

### W5-1. A `const` hidden variable needs its own `tuple_size` specialization
- Pages and rules: structured-bindings (SB-5, SB-4, SB-P2), language-support (LSUP-3)
- Kind: hard to find, needs example
- What happened: I specialized `std::tuple_size` and `std::tuple_element` for `Range` and bound `auto [lo, hi]` in one function. That worked. In `main` I wrote `const auto& [lo, hi]`, and both compilers rejected it. GCC said it "cannot decompose inaccessible member", and Clang said it "cannot bind private member". The hidden variable's type was `const Range`, and `tuple_size<const Range>` was incomplete, so SB-5 sent the binding to the data members, which were private. SB-5 does say that E is the hidden variable's type. Nothing on the page says that E keeps its `const`, or that a `const` binding needs separate specializations. SB-P2 says qualifiers are added to the element types, which suggests the class type is looked at without them. The error message points at access control and gives no hint about the tuple protocol.
- Suggestion: state in SB-5 that E includes the cv-qualifiers written on the declaration and those of the initializer, so `const auto&` looks up `tuple_size<const C>`. Add an example in which a `const` binding falls back to data members. Say in LSUP-3 that a program usually specializes for the `const` type too.

### W5-2. LSUP-3 gives no declaration shape for the tuple templates
- Pages and rules: language-support (LSUP-3, LSUP-P1), structured-bindings (SB-5)
- Kind: missing
- What happened: I had to guess the primary templates. I wrote `template <class T> struct tuple_size;` and `template <size_t I, class T> struct tuple_element;`, with `size_t` taken from `decltype(sizeof 0)`. I also guessed that `value` should be a `static constexpr size_t`. My guesses worked. The other language-support types on the page come with example declarations, but these two do not. The page also does not say what `tuple_element<I, const C>::type` should be. A probe showed that `decltype` of the binding is exactly that member type, and the hidden variable's `const` is not added. SB-4 adds cv-qualifiers only for array elements and data members, so a reader cannot tell that the two cases differ.
- Suggestion: add an example to LSUP-3 with both primary templates, specializations for a class and for its `const` form, and a member `get`. State that for the tuple protocol `decltype` of a name is the `tuple_element` type as written, with no qualifiers added.

### W5-3. A deleted function in a requires expression
- Pages and rules: concepts (CPT-26, CPT-27, CPT-30, CPT-31), decltype (UNEV-5), deleted (DEL-1)
- Kind: unclear, needs example
- What happened: I checked the rejected calls with `template <class T> concept Twiceable = requires(T v) { twice(v); };` and `static_assert(!Twiceable<double>)`. Both compilers made the requirement false, and the program stayed valid. To predict this I had to chain four rules. UNEV-5 says deleted functions are still checked in unevaluated operands. CPT-30 says requirements are unevaluated. CPT-26 says an invalid substituted expression makes the requirement false. DEL-1 says choosing a deleted function makes the program invalid. "Invalid" in DEL-1 and "not valid" in CPT-26 could mean different things. A probe without a template parameter, `!requires { twice(1.5); }`, was a hard error on both compilers. That result matches CPT-27.
- Suggestion: add a sentence to CPT-26 saying that overload resolution choosing a deleted function counts as not valid, so the requirement is false. Add the dependent case and the non-dependent case to the examples on the deleted page or the concepts page.

### W5-4. The step limit of constant evaluation is easy to reach with recursion
- Pages and rules: constant-evaluation (CE-10, CE-P5), return-deduction (RET-3 example)
- Kind: needs example
- What happened: `static_assert(fib(40) == 102334155)` with a naive double recursion failed on both compilers. GCC reported "operation count exceeds limit of 33554432", and Clang reported "hit maximum step limit". CE-10 gives a depth figure (400 accepted) and says that a loop of 100000000 steps is rejected. It gives no figure for steps that are accepted, so I could not tell where the limit is. The call depth here was only 40. The total number of calls was the problem.
- Suggestion: in CE-10, give a step count that every matrix compiler accepts, or say that the limit counts the total work of the evaluation, not the depth. A Go reader expects compile-time work to cost nothing, and a note on the return-deduction page would help when the `fact` example turns into `fib`.

### W5-5. Deleting one overload can make other calls ambiguous
- Pages and rules: deleted (DEL-1, DEL-P1, example), overload-resolution (OVL-P3), conversion-sequences (ICS-P2)
- Kind: needs example
- What happened: I wrote this from the rules and got it right, but the deleted page's example is too thin to plan from. With `twice(int)` kept and `twice(bool)`, `twice(char)` and `twice(double)` deleted, the results are as follows. `short` still calls `twice(int)` through a promotion. `float` picks the deleted `twice(double)` through a promotion. `long` is ambiguous, because every candidate is a plain conversion. A Go programmer has no implicit numeric conversions and would not expect any of these three results. I first planned to delete only a `double` overload beside a `long` one. Then I saw from ICS-P2 that a call with an `int` would have been ambiguous.
- Suggestion: extend the deleted page's example with the common pattern of keeping one overload and deleting the others. Show one call each that is fine, that is rejected as deleted, and that is rejected as ambiguous. Add the deleted catch-all template (`template <class T> R f(T) = delete;` beside a plain overload), with a pointer to OVL-27.

### W5-6. CE-1 does not list a call through a function pointer
- Pages and rules: constant-evaluation (CE-1), postfix (PFX-7)
- Kind: unclear
- What happened: CE-1 lists "calls to `constexpr` functions" and "equality comparison of function pointers". It does not say that a call through a pointer to a `constexpr` function is allowed during constant evaluation. My dispatch through `Handler*` in a `static_assert` worked on both compilers. I had to assume that "calls" covered it.
- Suggestion: add "a call through a pointer to a `constexpr` function" to the CE-1 list.

### W5-7. A local class as a deduced return type is not mentioned
- Pages and rules: return-deduction (RET-1, RET-7), members (CLS-12)
- Kind: hard to find
- What happened: RET-7 rules out `auto f() { return {a, b}; }`. The obvious workaround for "return two values" without naming a type outside the function is `struct Out {...}; return Out{...};` inside the body. A probe shows that both compilers accept this, and structured bindings work on the result. No page mentions this pattern, so I wrote a named namespace-scope struct in the main program.
- Suggestion: add an example to the return-deduction page that returns two values, once with a named struct and once with a local struct, and point to structured bindings.

## Attempt log

1. Read the function pages, auto, decltype, inline, structured bindings, the declarator principles, the overload-resolution and conversion-sequence principles, the concepts requirements, constant evaluation, and language-support. Wrote `notes.md` before writing any code.
2. Wrote all four programs and built them with both compilers, plain and with sanitizers.
   - commands.cpp: passed on both compilers, plain and sanitized.
   - deleted.cpp: passed on both compilers, plain and sanitized.
   - two_values.cpp: rejected by both compilers at `const auto& [lo, hi] = bounds(...)` (W5-1). I had read SB-5 as "the class has a tuple size", and I did not think about the `const` in E. The fix adds specializations of `tuple_size<const Range>` and `tuple_element<I, const Range>`. Attempt 1 is kept as `probe/two_values-attempt1.cpp`.
   - recursion.cpp: `static_assert(fib(40) == ...)` exceeded the step limit on both compilers (W5-4). CE-10 gave me a depth figure, so I assumed a depth of 40 was safe. The fix lowered the check to `fib(25)` and kept `fib(30)` at run time. Attempt 1 is kept as `probe/recursion-attempt1.cpp`.
3. Rebuilt: all four pass on both compilers, plain and with `-fsanitize=undefined,address`, all exit 0, with no warnings.
4. Probes to confirm what I had read:
   - `probe/req-nontemplate.cpp`: a non-dependent requirement that picks a deleted function is a hard error on both compilers, as CPT-27 says (W5-3).
   - `probe/mutual.cpp`: mutual recursion works when one function has a written return type. Both exit 0.
   - `probe/mutual-both-auto.cpp`: both functions with `auto` is rejected on both compilers, as RET-3 predicts.
   - `probe/const-tuple-elem.cpp`: for a tuple-protocol binding, `decltype` gives the `tuple_element` type unchanged, with no `const` added (W5-2).
   - `probe/auto-braced-return.cpp`: rejected on both compilers, as RET-7 says.
   - `probe/struct-auto-return.cpp`: a local struct as a deduced return type is accepted, and it works with structured bindings (W5-7).

## What worked well

- RET-3 and the `fact` example told me exactly where a recursive call may go. The mutual-recursion probes matched that prediction.
- DEFARG-5 stated plainly that a call through a pointer loses the default arguments. The `scale` handler behaved as predicted.
- The ICS-P2 diagram, with float to double listed as a promotion, let me predict every deleted-overload result in deleted.cpp correctly on the first try.
- OVL-P3 step 2c, a plain function over a template, made the deleted catch-all template easy to predict.
- FN-11 gave the exact `__func__` contents on the matrix, so `sizeof(__func__) == 6` held.
- RET-2 and AUTO-10 made the `decltype(auto)` reference return work on the first try.
- SB-15 warned me that structured bindings cannot be `constexpr`, so I put them in a `constexpr` function from the start.
- The decltype page's prvalue rule made the `same_type<decltype(fact(3)), int>` checks straightforward.
