# W6 report: initialization

- Date: 2026-11-03
- Role: clean-room reader
- Persona: Swift programmer
- Programs: config_table.cpp, default_members.cpp, ref_members.cpp, bindings.cpp

## Summary

I wrote all four programs from the specification. In the final versions, each one builds without a diagnostic on g++ 16.2.1 and clang++ 22.1.8 with `-std=c++23 -pedantic-errors -Wall -Wextra`, and each exits 0 both plain and under `-fsanitize=undefined,address`. The bindings program built without changes on the first try. The other three needed fixes for warnings.

The three biggest problems:

1. **A parenthesized initializer that looks like a parameter declares a function (W6-1).** I wrote `Holder h(Tracker(&live));` to show the REF-5 case for an aggregate initialized with parentheses. Both compilers read the line as a function declaration. The `static_assert` that checked the case still passed, because nothing was ever made. The forms page (INIT-3, INIT-P3) mentions only empty parentheses. The general rule is DCL-13, on the declarators page.
2. **The tuple protocol with `const auto` needs a specialization for the const type (W6-2).** In a program with no headers, `const auto [lo, hi] = range;` failed on both compilers. The binding fell back to the private data members. SB-5 is correct as written, since `E` is the type of the hidden variable. Still, LSUP-3 says only "specializes them for the class", and no page shows that `const E` needs its own `tuple_size` and `tuple_element`.
3. **Valid omissions and brace elision draw warnings under `-Wall -Wextra` (W6-3, W6-4).** LIST-P2, LIST-4, and LIST-8 present these forms as ordinary: leaving out a member, and leaving out inner braces. Both compilers warn about an omitted member that has no default member initializer. Clang also warns about elided braces. The pages do not mention either warning.

## Construct-to-rule notes (made before writing)

**config_table.cpp**
- `std::size_t`: LSUP-4. `std::initializer_list` shape: LSUP-1 and the example on its page.
- The row type is an aggregate (definition in the list page introduction). Values fill members in order: LIST-3. Missing members get the default member initializer, or zero: LIST-8, LIST-P2.
- Designators: LIST-5, LIST-14. Narrowing: LIST-2.
- `T table[] = {...}` takes its bound from the list: DCL-5, DCL-19.
- `constexpr` table: CX-1, CX-2, CE-23. Pointers to string literals are allowed because a literal lives for the whole program (introduction of the primary expressions page).
- A `constinit` table that `main` can change: CINIT-1, CINIT-2, SINIT-1.
- An earlier dynamic initializer reads a later `constinit` variable and gets its value: SINIT-1, SINIT-2, the SINIT-P1 example.
- Block-scope `static`: STO-2. `static_assert`: SA-1, SA-2.

**default_members.cpp**
- `=` or braces for a default member initializer, never parentheses: CLS grammar, CLS-9. A default member initializer can read other members: CLS-2.
- Aggregate list with missing members: LIST-8. Parentheses: LIST-3, LIST-9. Brace elision: LIST-4.
- Default initialization: INIT-9, INIT-P2, INIT-1.
- A member initializer replaces the default: CTOR-5. A `const` object with no initializer: CTOR-4. A constexpr aggregate gives every scalar part a value: CE-21.

**ref_members.cpp**
- Binding kinds: REF-1, REF-2, REF-3, REF-8 to REF-10. A temporary holds the converted value: REF-7.
- Life extension: REF-4. No extension: REF-5. A constructor binding a reference member to a temporary: CTOR-17.
- A `constexpr` destructor, to count objects at translation time: CE-23. Static `inline` members: CLS-3.

**bindings.cpp**
- Syntax and specifiers: SB-7, SB-11, SB-15. Array copy or reference: SB-1.
- Data members: SB-3. `decltype` of the names: SB-4.
- Tuple protocol: SB-5, SB-14, SB-17, LSUP-3. A temporary lives with the names: SB-6, SB-17.
- Names are lvalues: SB-12. Bit-fields: SB-10.
- A binding as a range-for loop variable: range-for page. A lambda capturing a binding: lambdas page.

## Findings

### W6-1. A parenthesized initializer that has the form of a parameter declares a function
- Pages: declarations/initialization/forms.md (INIT-3, INIT-P3), declarations/declarators.md (DCL-13), declarations/initialization/references.md (REF-5)
- Kind: hard to find, needs example
- What happened: REF-5 says a reference member of an aggregate initialized with parentheses does not extend the temporary. To show this, I wrote `Holder h(Tracker(&live));` in a constexpr function. Both compilers warned (`-Wvexing-parse`) that the line declares a function. No `Tracker` was made, so the count I asserted on was 0 for the wrong reason, and the `static_assert` passed. I relied on INIT-3 and INIT-P3, and both speak only of empty parentheses. DCL-13 covers the general case, but the forms page does not link to it.
- Fix: I wrote `Holder h(Tracker{&live});`. Clang then warned under `-Wdangling` (see W6-5), so I moved the case to probe/paren_holder.cpp.
- Suggestion: Extend INIT-P3 and INIT-3 to cover any parenthesized initializer that can be read as a parameter list, with a link to DCL-13. Add an example where a temporary of class type is the argument, and show the two fixes: braces, or an extra pair of parentheses. REF-5 could link to the same example, because REF-5 leads a reader straight into this form.

### W6-2. `const auto` with the tuple protocol needs a `const E` specialization
- Pages: declarations/structured-bindings.md (SB-5), foundations/language-support.md (LSUP-3)
- Kind: missing, needs example
- What happened: I specialized `std::tuple_size<Range>` and `std::tuple_element<I, Range>`, as LSUP-3 describes. `auto [lo, hi] = r;` and `auto& [lo, hi] = r;` worked. `const auto [lo, hi] = r;` failed on both compilers ("cannot decompose inaccessible member", "cannot bind private member"). The hidden variable has type `const Range`, `tuple_size<const Range>` was incomplete, and the binding fell back to data members under SB-3. SB-5 does say that `E` is the type of the hidden variable. A Swift reader takes "specializes them for the class" to cover the class in every form.
- Fix: I added specializations for `const Range`. `decltype` of the names is then whatever I put in `tuple_element<I, const Range>::type`.
- Suggestion: In LSUP-3, say that the hidden variable's type includes its cv-qualifiers. A program with no headers must therefore also specialize the templates for the `const` (and `volatile`) forms of the class that it binds. Add an example with a member `get` and both specializations. LSUP-3 has no example at present.

### W6-3. Omitting a member with no default member initializer draws a warning
- Pages: declarations/initialization/list.md (LIST-8, LIST-P2, LIST-5)
- Kind: missing
- What happened: `{.key = "mode", .enabled = false}` left out `value`, which LIST-8 makes zero. Both compilers warned under `-Wextra`: GCC with `-Wmissing-field-initializers`, Clang with `-Wmissing-designated-field-initializers`. A probe gave this pattern:
  - Positional lists warn too.
  - Empty braces `{}` do not warn.
  - A missing member that has a default member initializer does not warn.
  - A missing member of class type warns even when its own members all have defaults (`Size icon;` in default_members.cpp).
- Fix: I wrote every such member, as `.value = 0` and `.icon{}`, or as `{}` in a positional list.
- Suggestion: Add a sentence to the Evidence segment of the list page saying when GCC and Clang warn about an omitted member under `-Wextra`. A reader then knows that a clean build favors default member initializers, or `{}` written out.

### W6-4. Clang warns about brace elision under `-Wall`
- Pages: declarations/initialization/list.md (LIST-4)
- Kind: missing
- What happened: `{1, 2, 3.0, false, "t", 9, 4, 5}` uses LIST-4 to fill the nested `Size` member. Clang warned `-Wmissing-braces` under `-Wall`. GCC did not warn.
- Fix: I wrote the inner braces, `{4, 5}`.
- Suggestion: Note in the LIST-4 Evidence that Clang warns when braces are elided for a nested class member under `-Wall`.

### W6-5. The dangling-reference warnings differ by case and compiler
- Pages: declarations/initialization/references.md (REF-5, REF-15), classes/constructors.md (CTOR-17)
- Kind: unclear
- What happened: The compilers warned about different cases:
  - Aggregate in parentheses with a reference member bound to a temporary: Clang warns (`-Wdangling`). GCC is silent.
  - `const Note& w = pass(Note{7});`: GCC warns (`-Wdangling-reference`). Clang is silent. Both sanitizers report a stack use after scope at the read.
  - A constructor binding a reference member to a temporary: Clang rejects the program as an error. GCC warns under `-Wextra`. The CTOR-17 Evidence states this.

  The REF-15 Evidence names only the `new` case and then says that "each compiler warns about some other dangling cases". A reader cannot tell from that which forms will build clean.
- Suggestion: Add a small table to the REF-15 Evidence, with one row per REF-5 case and one column per compiler.

### W6-6. An array bound from a braced list is stated only on the declarators page
- Pages: declarations/initialization/forms.md (INIT-5), declarations/declarators.md (DCL-5)
- Kind: hard to find
- What happened: INIT-5 says "In parentheses, ... an array without a bound gets the number of values as its bound." Read alone, the sentence suggests that only parentheses supply a bound. I found the braced case in DCL-5, where "an initializer gives the number of elements". The forms example `int d[4] = {1, 2};` always writes the bound.
- Suggestion: In INIT-5, state that braces and parentheses both supply the bound, with a link to DCL-5. Add `int h[] = {1, 2, 3};  // int[3]` to the forms examples.

### W6-7. The hidden reference of a tuple-protocol name is not fully described
- Pages: declarations/structured-bindings.md (SB-17, SB-4, SB-5)
- Kind: unclear
- What happened: SB-17 says each name refers to "a hidden reference to its element type", initialized from the `get` result. The rule does not say whether this is an lvalue or an rvalue reference. When `get` returns a prvalue, I could not predict whether `auto& [a, b] = r;` would work. A probe showed that it builds on both compilers. SB-4 gives `decltype` only for array elements and data members. For the tuple case I had to infer from SB-5 that `decltype` is `tuple_element<i, E>::type`, and that no qualifiers are added.
- Suggestion: In SB-17, state the kind of the hidden reference and how it depends on the `get` result. In SB-4, add a sentence for the tuple case, and point back to W6-2 for cv-qualified `E`.

## Attempt log

1. **config_table.cpp, attempt 1.** Both compilers warned about the designated row that left out `value` (W6-3). I probed positional and designated omissions (probe/omit.cpp). I wrote `.value = 0`, and the program was clean on both compilers and both sanitizer builds.
2. **default_members.cpp, attempt 1.** I got the W6-3 warnings again for `icon` and `visible`, and Clang's `-Wmissing-braces` (W6-4). I wrote `.icon{}`, `{4, 5}`, and `.visible = false`.
3. **default_members.cpp, attempt 2.** Clean, but `wide` now set `area` itself and no longer tested that a later default sees an earlier value. I rewrote `wide` with designators so that `area` takes its default. Clean, and exit 0.
4. **default_members probes (probe/dm_probe.cpp).** The results matched CTOR-4, CLS-9, and CX-1:
   - `const Window cw;` was rejected.
   - `int x(5);` as a default member initializer was rejected.
   - `constexpr Window d;` in a block was rejected.
   - `constexpr Window cx{.width = 2};` was accepted, with the W6-3 warning.
5. **ref_members.cpp, attempt 1.** Both compilers warned `-Wvexing-parse` on `Holder h(Tracker(&live));` (W6-1). Clang also warned about an unused `h`.
6. **ref_members.cpp, attempt 2.** I changed the argument to `Tracker{&live}`. Clang then warned `-Wdangling`, which is a correct warning about the REF-5 case I meant to show. I moved the parentheses case to probe/paren_holder.cpp. The program was then clean, and exit 0 under both sanitizer builds.
7. **Dangling probe (probe/dangle.cpp).** GCC warned `-Wdangling-reference` and `-Wextra`. Clang rejected the reference members in the constructor. AddressSanitizer reported a stack use after scope (W6-5).
8. **bindings.cpp, attempt 1.** Clean, and exit 0 everywhere.
9. **Tuple probe (probe/tuple_const.cpp).** `const auto` failed until I added the `const Range` specializations (W6-2). `auto&` with a `get` that returns a prvalue worked (W6-7). I added the `const` case and a `decltype(c_alias) == int&` check to bindings.cpp. Clean, and exit 0.

## What worked well

- LIST-8 and the list page examples let me predict every member value in `Window`. That covers defaults, zeros, and a default that reads an earlier member which the list has replaced.
- The REF-4 and REF-5 lists were exact. Every count of live objects I predicted matched at translation time and at run time, including the destruction order in the REF-4 example and extension through `?:`.
- The SINIT-P1 diagram and the `constinit` examples made the early reader of a later `constinit` variable easy to write correctly.
- The SB-P1 diagram of a hidden variable made copy versus reference for arrays, and `decltype` with `const auto`, predictable on the first try. That includes the case where a reference member keeps its type without `const`.
- LSUP-1 and its example gave a working `std::initializer_list` with no header.
