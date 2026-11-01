# Predict report C: expressions batches R-assignment to R-member-pointers

- Date: 2026-11-01
- Role: clean-room reader
- Persona: programmer new to C++
- Inputs: AGENTS.md, and the six batch directories with their `specification/` copies (the test lists removed). Nothing was compiled or run.

## Batches and counts

| Batch | Programs | reject | compiles | runs | run-fail | depends |
|---|---|---|---|---|---|---|
| R-assignment | 32 | 13 | 7 | 11 | 0 | 1 |
| R-bitwise | 21 | 11 | 10 | 0 | 0 | 0 |
| R-comma | 24 | 5 | 6 | 13 | 0 | 0 |
| R-decltype | 51 | 14 | 26 | 8 | 0 | 3 |
| R-logical | 34 | 10 | 11 | 12 | 0 | 1 |
| R-member-pointers | 70 | 30 | 14 | 24 | 2 | 0 |
| Total | 232 | 83 | 74 | 68 | 2 | 5 |

No program got the outcome `undefined` or `link-fail`. The five `depends` answers come from divergences that the Evidence segments record: assignment p01 (ASN-14, braced list in a compound assignment), decltype p24 (UNEV-4), decltype p25 (DTY-5), decltype p44 (DTY-3), and logical p26 (LOG-12, GCC 14).

## Findings

**REC-1**
- Batch and program: R-assignment p07
- Rules: ASN-7, VOL-5
- Kind: hard to find, outside knowledge
- What I needed: I expected `v += 1` on a volatile to be deprecated. Only VOL-5 on the volatile page says compound assignment to a volatile is not deprecated. ASN-7 does not mention deprecation and does not link to VOL-5.
- Suggestion: Add a sentence to ASN-7 that compound assignment to a volatile object is valid and not deprecated, and link to VOL-5. Mention that the value of a volatile assignment used in a larger expression is deprecated.

**REC-2**
- Batch and program: R-assignment p16, p26
- Rules: ASN-2, PRIM-12
- Kind: needs example
- What I needed: ASN-2 rejects a const left operand. In a const member function, a member is const because of PRIM-12 on the primary-expressions page. The assignment page does not make that link.
- Suggestion: Add an example line to the assignment page showing that assigning to a member inside a const member function is rejected, and cite PRIM-12.

**REC-3**
- Batch and program: R-assignment p09
- Rules: ASN-11, CE-27
- Kind: hard to find
- What I needed: I needed to know if the unspecified order in ASN-11 still holds in a constant expression, so that a static_assert can accept either result. CE-27 answers this on the constant-evaluation page, but ASN-11 does not point to it.
- Suggestion: Link ASN-11 to CE-27, or add a constexpr line to the assignment examples.

**REC-4**
- Batch and program: R-bitwise p14, p16
- Rules: BIT-2, BIT-8
- Kind: missing, hard to find
- What I needed: BIT-8 says only that an implementation "should warn" about two different enumerations. The page does not say that the mix is deprecated, or that the warning is on by default. I found both facts only in the warning table in `foundations/translation.md` and in the flag name in the README.
- Suggestion: State on the bitwise page that operands of two different enumeration types are deprecated. Also state that Clang and GCC warn with no flag, so `-Werror` alone rejects the program. The multiplicative and additive pages may need the same note.

**REC-5**
- Batch and program: R-decltype p23
- Rules: LAM-18, DTY-1
- Kind: unclear
- What I needed: LAM-18 says `decltype((x))` is a reference to const "for a copy capture x". In p23, the lambda has `[=]` and names `x` only inside decltype, so `x` may not be captured at all. I could not tell from the text whether LAM-18 still applies. I answered `compiles` on the guess that it does.
- Suggestion: Say whether the rule covers a name that a copy default would capture, even when the body uses the name only in an unevaluated operand.

**REC-6**
- Batch and program: R-decltype p35
- Rules: PRIM-12, PRIM-25, UNEV-1
- Kind: missing
- What I needed: In p35, a member function of a nested class uses a non-static member of the enclosing class in an evaluated expression. PRIM-25 covers only uses "outside a member function", and p35 is inside one. PRIM-12 turns the name into `this->m`, but `this` points to the nested class, which has no `m`. No rule I found clearly rejects the program. I answered `reject` from general knowledge.
- Suggestion: Add a rule that an evaluated use of a non-static member needs an object of that member's class (or a derived class) through `this`. State that a nested class's member function does not have one for the enclosing class. Contrast the rule with UNEV-1, which allows such a name in sizeof.

**REC-7**
- Batch and program: R-decltype p42
- Rules: UNEV-1, COMMA-2
- Kind: unclear
- What I needed: UNEV-1 lets a decltype operand that is "a function call that gives a prvalue of class type" have a class with no usable destructor. I could not tell if the same holds when the call is the right operand of a comma, as in `decltype(0, make())`. I guessed that it does.
- Suggestion: Say whether the exception reaches the right operand of a comma operator, and add an example.

**REC-8**
- Batch and program: R-decltype p18
- Rules: UNEV-1, UNEV-5, UN-14
- Kind: hard to find
- What I needed: UNEV-5 lists what sizeof still needs (a usable destructor) but does not say that sizeof needs a complete type. The incomplete-type rule is UN-14 on the unary page. A reader of the decltype page sees only that decltype allows an incomplete type.
- Suggestion: Add "sizeof needs a complete type (UN-14)" to the UNEV-5 list.

**REC-9**
- Batch and program: R-logical p17
- Rules: LOG-9
- Kind: unclear
- What I needed: LOG-9 asks for a warning when `!` "applies only to the left operand of a comparison". In `(!a) == b`, the `!` still applies only to the left operand. The rule does not say that parentheses stop the warning, which BIT-6 and LOG-8 do say for their cases. I answered `compiles` from general knowledge.
- Suggestion: Say that parentheses around `!a` stop the warning, and add `(!a) == b` to the examples.

**REC-10**
- Batch and program: R-member-pointers p34, p46
- Rules: MPTR-6
- Kind: missing, outside knowledge
- What I needed: Both programs have undefined behavior under MPTR-6, and the README asks for `run-fail` when a sanitizer in the flags stops the operation. The member-pointers Evidence does not say if the address sanitizer stops a read through a null pointer to data member (p34). It also does not say if the vptr sanitizer stops a call through a member function pointer on an object of the wrong dynamic type (p46). I answered `run-fail` for both from general knowledge of the sanitizers.
- Suggestion: Record in the Evidence segment which sanitizer stops each MPTR-6 case on GCC and Clang, as OBJ-10 and STG-17 do.

**REC-11**
- Batch and program: R-member-pointers p16, p56
- Rules: MPTR-4, MPTR-11, CONV-9
- Kind: needs example
- What I needed: MPTR-4 and MPTR-11 refer the cv-qualifier rules back to ordinary pointers. I had to work out by hand that `int* S::*` to `const int* S::*` breaks the multi-level rule of CONV-9, and that `const int* const S::*` follows it.
- Suggestion: Add both conversions to the examples. Mark one as valid and one as rejected, and say which level of the pointer to member counts as the "level above" in CONV-9.

**REC-12**
- Batch and program: R-member-pointers p19
- Rules: MPTR-8, MPTR-9
- Kind: unclear
- What I needed: MPTR-8 forbids a pointer-to-member type whose member type is a reference. MPTR-9 lists the members that `&C::m` cannot name, and the list does not include a reference member. I inferred the rejection of `&S::r` from MPTR-8.
- Suggestion: Add "a member of reference type" to the MPTR-9 list, or cross-reference MPTR-8 there.

**REC-13**
- Batch and program: R-member-pointers p51
- Rules: MPTR-4, MPTR-11
- Kind: unclear
- What I needed: MPTR-11 says a conversion between a pointer to data member and a pointer to member function is not valid. The rule does not say if `reinterpret_cast` is also excluded. MPTR-4 only implies the answer, because it lists the two `reinterpret_cast` cases that are allowed.
- Suggestion: State in MPTR-11 that no cast, `reinterpret_cast` included, converts between the two kinds.
