# X3: Automatic objects start at zero (proposal P5)

Date: 2026-10-31. Exercise X3 (decision 0037). Role: clean-room reader, language-designer persona.

Inputs read:

- AGENTS.md
- decision 0037 and the Propose section of `docs/exercises.md`
- P5 and the cross-cutting needs in `reports/clean-room/roadmap/designer.md`
- these spec pages:
  - `declarations/initialization/forms.md`, `list.md`, `static.md`
  - `declarations/storage-linkage.md`, `constexpr.md`, `declarators.md` (CV-3), `attributes.md`, `volatile.md` (principle only)
  - `expressions/new-delete.md`, `expressions/assignment.md` (ASN-11)
  - `foundations/objects.md`, `constant-evaluation.md`, `translation.md` (TR-P5, TR-7, TR-8)
  - `classes/constructors.md`, `derived.md` (DER-8), `unions.md` (UNI-4, BF rules), `copy-move.md` (CPY-8)
  - `statements/jumps.md`, `switch.md`, `if.md` (IF-18 to IF-24)
  - the glossary
- questions Q-0013, Q-0039, Q-0097, Q-0115, Q-0117
- the tests named below and their rows in `tests/RESULTS.md`

I also compiled small probes with the host Clang 22 and GCC 16 (`-std=c++23 -pedantic-errors`) in `/tmp/claude-1000/x3/`. The feature and the attribute name are my own design.

## 1. Design

### 1.1 The feature

Default initialization of a scalar writes zero. "Zero" means the value that INIT-2 gives for empty braces, extended to every scalar type (X3-5).

These objects get zero:

1. An automatic variable of scalar type declared with no initializer.
2. Each element of an automatic array with no initializer, when the element is a scalar or is covered by item 3.
3. Each scalar member, at any depth, that default initialization of a class object leaves without a value today. This covers:
   - a class with an implicit or defaulted default constructor,
   - a member that a user-provided constructor does not name in its member initializer list and that has no default member initializer,
   - the members other than the inherited base under DER-8,
   - such class objects with any storage duration, including static.
4. An object that `new T` or `new T[n]` creates with no initializer, and each element and member under items 2 and 3.

```cpp
int x;               // 0
int a[3];            // 0, 0, 0
struct P { int v; double d; int* p; };
P s;                 // v == 0, d == 0.0, p == nullptr
int* q = new int;    // *q == 0
```

The zero is written when control passes the declaration. Each pass writes it again (STO-1). The same holds for every iteration of a loop body. For `new`, the write is part of the initialization step that NEW-9 places after the allocation.

### 1.2 What stays as it is

- **Objects with an initializer.** P5 changes default initialization only. A variable that its own initializer reads still has no value at that read (INIT-1, `self-initialization.cpp`). A member that a later `mem_init` initializes still has no value before its turn (CTOR-12, `init-order-reads-later.cpp`).
- **A skipped declaration.** A jump that skips the declaration of a scalar with no initializer (JMP-6, SW-5) also skips the zero. The object then has no value until the program writes it. A read before that write stays undefined. Section 3 explains why (X3-2).
- **Unions.** P5 makes no union member active. A union with no active member stays as UNI-4 and CE-21 describe it (X3-8).
- **`const` objects without an initializer.** CV-3, CTOR-4, NEW-11, and the scalar case of CX-1 keep rejecting them. Section 3 gives the reason (X3-12).
- **Static and thread storage.** STO-3 already gives zero. P5 changes these objects only through item 3. A class object with static storage whose constructor leaves a member to default initialization now gets zero written there, so the result has a value in every part.

### 1.3 The opt-out attribute

`[[uninitialized]]` restores today's state for code that measures the cost of the zero.

- It can go on the declaration of an automatic variable with no initializer.
- It can go on a non-static data member that a default member initializer does not set. Default initialization then leaves that member with no value, wherever the class is default-initialized.
- It is not valid on a declaration that has an initializer, on a variable with static or thread storage, or on a member with a default member initializer.
- An object that the attribute covers has no value until its first write, as INIT-1 says today. A read before the write is undefined. Constant evaluation rejects it (CE-9). An implicit copy of a class with such a member reads it (CE-22).
- P5 has no opt-out for `new` (X3-7).

```cpp
[[uninitialized]] char buf[4096];   // no value until written
int n [[uninitialized]] = 0;        // rejected: has an initializer
```

The attribute obeys ATTR-P2 in this sense. Take a valid program that uses it, and remove it. The program does the same thing, because a valid program writes such an object before it reads it. Adding the attribute can make a valid program undefined, as `[[assume]]` can (ATTR-10).

An implementation that does not know the attribute ignores it (ATTR-1). The program then gets zero, which is a valid execution of every program that uses the attribute correctly.

### 1.4 Constant evaluation

Constant evaluation runs default initialization as well. So the zero exists there too, and these evaluations become valid:

- a read of a default-initialized scalar,
- an implicit copy of an object whose parts were left to default initialization (CE-22),
- a constant whose result has such parts (CE-21).

```cpp
struct Pair { int first; int second; constexpr Pair() : first(1) {} };
constinit Pair pair;                 // valid: second == 0
```

## 2. Rule changes

The IDs of new rules are placeholders. The integrator numbers them.

| ID | Change | Old class | New class | Description | Tests whose expected outcome changes |
|---|---|---|---|---|---|
| INIT-P2 | amended | principle | principle | An automatic object with no initializer starts at zero. Only the opt-out and a skipped declaration leave it without a value. | none |
| INIT-1 | split | undefined | undefined (narrowed), plus INIT-new-a | The undefined part keeps three cases: self-initialization, objects under `[[uninitialized]]`, and objects whose declaration a jump skipped. The `unsigned char` copy exception (Q-0039) applies only to these cases. | `declarations/initialization/forms/read-uninitialized.cpp` (reject to accept), `declarations/initialization/forms/read-uninitialized-array.cpp` (reject to accept; EDG override removed), `declarations/initialization/forms/copy-indeterminate-unsigned-char.cpp` (reject to accept) |
| INIT-new-a | new | none | required | Default initialization of a scalar writes zero when control passes the declaration. The same goes for array elements. | the three INIT-1 tests above move to this rule |
| INIT-new-b | new | none | required | Says what default initialization does to a class, its members, its bases, and an array of classes (D12, X3-1). | none |
| INIT-2 | amended | required | required | The definition of zero covers every scalar type, including enumerations and pointers to members. Other rules cite it as "the zero of the type". | none |
| NEW-P1 | amended | principle | principle | An object that `new` leaves without an initializer starts at zero. | none |
| NEW-3 | reclassified | undefined | required | `new T` with no initializer gives zero, and the same holds for each element of `new T[n]`. | `expressions/new-delete/uninitialized-read.cpp` (reject to accept) |
| OBJ-P4 | amended | principle | principle | The list of objects with no value shrinks. A scalar before its first write and a member that no initializer gave a value become cases of the opt-out or a skipped declaration. | none |
| OBJ-2 | amended | undefined | undefined | The "no value yet" case covers only the cases left by INIT-1. | `foundations/objects/read-before-init-constant.cpp` (reject to accept), `foundations/objects/uninitialized-member-read-constant.cpp` (reject to accept), and the two CE-22 tests below |
| CTOR-P2 | amended | principle | principle | The reason for the `const` object rule changes. A value now always exists, so the rule asks the program to state it. | none |
| CTOR-12 | amended | undefined | undefined | The rule drops "a member that default initialization leaves without a value", except under the opt-out. The `mem_init` order case and the base call case stay. | `classes/constructors/defaulted-leaves-members.cpp` (reject to accept) |
| CX-1 | amended | reject | reject | A `constexpr` default constructor that leaves a scalar member to default initialization now gives that member a value (D14). The clause for an implicit constructor stays tied to CTOR-4. | none |
| CE-21 | amended | reject | reject | A scalar part has no value only under the opt-out, or as an inactive union member. | `expressions/constant-expressions/result-unset-member.cpp` (reject to accept), `expressions/constant-expressions/result-unset-array-element.cpp` (reject to accept) |
| CE-22 | amended | reject | reject | Same narrowing. Q-0115 then covers only members under the opt-out. | `expressions/constant-expressions/copy-unset-member.cpp` (reject to accept; GCC override removed), `expressions/constant-expressions/copy-unset-array-element.cpp` (reject to accept; GCC override removed) |
| JMP-P2 | amended | principle | principle | A scalar declaration with no initializer now does work (it writes the zero). The principle keeps the exception for compatibility and says that the skipped object has no value. | none |
| JMP-6 | amended | reject | reject | The rule adds that an object whose declaration is skipped has no value until written. | none |
| SW-5 | amended | reject | reject | Same addition as JMP-6. | none |
| STO-1 | amended | required | required | "Each pass creates a new object" includes default initialization at that pass. | none |
| ATTR-P2 | amended | principle | principle | A new item says that `[[uninitialized]]` changes nothing in a program that writes before it reads. | none |
| ATTR-new-a | new | none | required | The meaning of `[[uninitialized]]`, as in 1.3. | none (new tests) |
| ATTR-new-b | new | none | reject | Where `[[uninitialized]]` can go. It has no arguments. | none (new tests) |
| glossary | new | none | term | "default initialization" becomes a glossary term with a pointer to INIT-new-b. Today it is only a word in a Syntax note. | none |

Totals:

- 22 rows: 17 existing IDs changed, 4 new rules, and 1 new glossary term.
- 11 tests change their expected outcome, all from reject to accept. Three of them lose a compiler override.
- `copy-indeterminate-unsigned-char-run.cpp` keeps its outcome. Its note and Q-0039 no longer apply to it.

These tests keep their outcome but need a second look:

- `declarations/initialization/forms/self-initialization.cpp` stays reject under the narrowed INIT-1.
- `classes/constructors/init-order-reads-later.cpp` and `base-init-calls-member-early-constant.cpp` stay reject under CTOR-12.
- `statements/jumps/goto-past-uninitialized-scalar.cpp` and `statements/switch/jump-past-uninitialized.cpp` stay valid under JMP-6 and SW-5.
- The CV-3, CTOR-4, NEW-11, and CX-1 tests stay reject:
  - `const-requires-initializer.cpp`
  - `const-object-no-init.cpp`
  - `const-object-defaulted-in-class.cpp`
  - `const-object-base-leaves-member.cpp`
  - `const-scalar-needs-init.cpp`
  - `variable-no-initializer.cpp`
  - `variable-default-initialized-no-value.cpp`
  - `variable-implicit-constructor-member-unset.cpp`
- The generated "Undefined rules" table in the constant-evaluation Evidence loses the INIT-1 and NEW-3 cases that flip.

New tests the proposal needs:

- the zero of each scalar kind,
- a member that a user constructor skips,
- `new T[n]`,
- a loop body that writes the zero again on each pass,
- a read after a skipped declaration (undefined, rejected in constant evaluation),
- each placement of the opt-out, valid and invalid,
- a read under the opt-out (rejected in constant evaluation).

## 3. Interactions

**Lifetime and storage** (OBJ-P1, OBJ-1, OBJ-P4, OBJ-2, STO-1, glossary "automatic storage"). OBJ-1 starts a scalar's lifetime with its storage, before it has a value. P5 keeps that and fills the value at the declaration. The proposal must say when the zero is written relative to the storage. The glossary says automatic storage exists "from the declaration", but JMP-6 lets a program write a variable whose declaration it skipped. The proposal ties the zero to the pass through the declaration (STO-1) and leaves a skipped object with no value (X3-2).

**Jumps** (JMP-P2, JMP-6, SW-5, Q-0097). JMP-P2 lets a jump skip a declaration that "has nothing to skip". Under P5 the zero is something to skip. I considered three designs:

1. Write the zero at block entry. This conflicts with STO-1, which creates the object at each pass through the declaration. It would also make a backward `goto` past the declaration skip the rewrite.
2. Reject such jumps. This breaks the valid tests `goto-past-uninitialized-scalar.cpp`, `jump-past-uninitialized.cpp`, and `goto-past-trivial-class.cpp`.
3. Keep the jump valid and leave the skipped object with no value.

The proposal takes option 3. It costs one remaining undefined case, and no valid program changes. Q-0097 (a non-trivial destructor) is not affected.

**Class default initialization** (CTOR-P2, CTOR-2, CTOR-3, CTOR-4, CTOR-12, DER-8, LIST-8). The proposal needs a rule for which members default initialization reaches. Tidal has none (X3-1). CTOR-3 and CTOR-20 keep deleting the default constructor for a `const` or reference member, because a reference has no zero and the `const` rule stays. LIST-8 already gives zero to members that a braced list omits, so aggregate initialization and default initialization now agree.

**`const` objects** (CV-3, CTOR-4, NEW-11, CX-1, CTOR-P2). Under P5 the reason in CTOR-P2 ("nothing can give it one later") no longer holds for scalars. The proposal keeps the rejections, for these reasons:

- Today's tests show that the rejections do not depend on a missing value. `const int x;` and `const S s;` at namespace scope already have the zero from STO-3, and every compiler rejects them.
- The opt-out can still leave a member with no value, so CTOR-4 still has work to do.

The proposal must restate the reason (X3-12). Relaxing these rules would be a separate proposal. It would flip the eight tests listed in section 2.

**Constant evaluation** (CE-P2, CE-P3, CE-9, CE-21, CE-22, CX-1, TR-P5, TR-8, Q-0013, Q-0115, Q-0117). Constant evaluation runs the abstract machine, so it sees the zero. CE-21 and CE-22 stay as reject rules, with fewer cases. CE-P3's sentence about copies stays true. Q-0013 and the default-initialization cases of Q-0115 close as moot, because every compiler would then have to accept. Q-0115 still matters for members under the opt-out. Q-0117 is unchanged, because P5 activates no union member.

**Static initialization and `if consteval`** (SINIT-1 to SINIT-4, IF-18, IF-P4). An initializer of a static variable is constant if its evaluation succeeds (SINIT-1). P5 makes some evaluations succeed that fail today. Those variables move from dynamic to constant initialization. That change can be seen in two ways:

- An `if consteval` in the evaluated function takes its other branch.
- An earlier dynamic initializer sees the final value instead of zero (SINIT-2, SINIT-3, SINIT-4).

Section 4 shows both. The proposal must list this as a change of meaning.

**Copies** (CPY-8, CPY-12, CE-22, lambdas). An implicit copy reads each part. Under P5 these parts have values, so copies of default-initialized objects become defined. A capture by copy is a member that the enclosing variable initializes. Capturing a variable that has no value is undefined today and defined under P5.

**`new`** (NEW-P1, NEW-1, NEW-2, NEW-3, NEW-9, NEW-11, NEW-14). The zero is part of the initialization step after allocation (NEW-9, NEW-12). `new T[n]` writes n zeros, and n can be known only at run time. That is the largest cost. Tidal gives no way to attach the opt-out to a `new` expression, and no rule for a direct call of an allocation function (X3-7).

**Attributes** (ATTR-P1, ATTR-P2, ATTR-1, ATTR-2, ATTR-10). The opt-out is a standard attribute with a fixed place and no arguments, like those in ATTR-2. ATTR-1 makes an unaware implementation ignore it, which is safe. ATTR-10 is the precedent for an attribute that adds undefined behavior to a program that misuses it. Tidal cannot tell me whether a later version uses the chosen name (the roadmap's version-marking need).

**Volatile** (VOL principle, TR-7). A `volatile int v;` in a block now gets a zero write. TR-7 makes each write of a volatile object observable. The proposal says that the zero is part of initialization and is not an observable access. Tidal does not settle whether initialization counts as a volatile write (X3-6).

**Bit-fields and enumerations** (BF-1, BF-3, ENUM range rule at enumerations.md line 256). A default-initialized bit-field gets zero. Zero is in the range of every enumeration, because the range is the bits that the enumerators need. INIT-2's list of zeros needs entries for enumerations and pointers to members (X3-5).

**Optimization** (TR-P5, TR-7). An implementation can drop a zero that the program overwrites before any read, because nothing observable depends on it. Today it can also assume that a read before a write never happens. P5 removes that assumption. A cost section is outside the spec (X3-9).

**Evaluation order** (ASN-11, SEQ rules). No interaction. P5 adds no operand and no ordering.

## 4. Impact on existing programs

### Programs that become valid

Any program that reads a default-initialized scalar, at run time or in constant evaluation. Today these programs are undefined (INIT-1, NEW-3, OBJ-2, CTOR-12) or rejected in constant evaluation (CE-9, CE-21, CE-22). The 11 tests in section 2 are the evidence. All seven compilers reject each of them today, with three exceptions:

- EDG accepts `read-uninitialized-array.cpp` (Q-0013).
- GCC 14 and 16 accept `copy-unset-member.cpp` and `copy-unset-array-element.cpp` (Q-0115).

These compilers already give the answer that P5 makes required. Programs that rely on those three behaviors become portable.

The host compilers show that today's run-time reads have no portable result. My probe returns a variable that is written on one path only, and returns a member of a default-initialized struct:

| Compiler | `-O0` exit | `-O2` exit |
|---|---|---|
| Clang 22 | 0 | 232 |
| GCC 16 | 0 | 5 (it folds the read to the value of the other path) |

Under P5 the result is 0 everywhere. No existing test observes a run-time read of this kind (X3-11).

### Valid programs that change meaning

1. **An `if consteval` branch in a static initializer** (SINIT-1, IF-18).

   ```cpp
   constexpr int m() { int x; if consteval { return x + 1; } else { return 2; } }
   int v = m();
   ```

   This program is valid today. Constant evaluation of `m()` reads `x` and fails, so `v` is dynamically initialized and the else branch runs. Clang 22 and GCC 16 give `v == 2` at `-O0` and `-O2`. Under P5 the evaluation succeeds and `v == 1`. The same holds for any constant-evaluation failure that P5 removes (CE-21, CE-22). When the failure is in a copy (Q-0115), GCC already gives the P5 answer and Clang does not. I confirmed this with a probe in which GCC gives 1 and Clang gives 2, though that probe is undefined at run time under today's rules.

2. **The order of static initialization** (SINIT-2, SINIT-3, SINIT-4). Take a namespace-scope `Pair pair;` with a `constexpr` constructor that sets only `first`. Today it is dynamically initialized. An earlier initializer that reads `pair.first` gets 0. Both host compilers give 0 at `-O0` and `-O2`. Under P5, `pair` is constant-initialized and the read gives 1. This counts as a change of meaning only if the read is valid today. SINIT-2 says that the variable holds zero, and OBJ-2 says that a read before the constructor completes is outside the object's lifetime (X3-4).

3. **Performance.** Programs keep their observable behavior (TR-P5). They can get slower, chiefly through large automatic arrays and `new T[n]`.

### Programs that become invalid

- None through the zero itself.
- A program that uses the name `[[uninitialized]]` today has it ignored (ATTR-1). Under ATTR-new-b, such a program becomes invalid if the attribute has arguments, is on a declaration with an initializer, or is on a static variable. Tidal gives no way to measure how common that is.
- A program that puts the attribute where it is valid and reads before writing stays undefined, as it is today.

### Programs that do not change

- `const` objects without an initializer stay rejected (CV-3, CTOR-4, NEW-11, CX-1).
- Self-initialization and early reads in a member initializer list stay undefined.
- Jumps past declarations keep their validity.
- Static and thread objects with no constructor keep STO-3.

## 5. Findings

**X3-1.** Page: `declarations/initialization/forms.md`, `classes/constructors.md`. Rules: INIT-P2, CTOR-12, DER-8, CTOR-4. Kind: missing.

- Needed: a rule that says what default initialization does to:
  - a class with an implicit constructor,
  - a member that a user constructor does not name,
  - a base,
  - an array of classes.
- The term exists only in a Syntax note. DER-8 uses it without a definition. This is D12 again, and the proposal needs it for item 3 of 1.1.
- Suggestion: a required rule in forms.md, with a glossary entry.

**X3-2.** Page: `statements/jumps.md`, `declarations/storage-linkage.md`, glossary. Rules: JMP-6, SW-5, STO-1, OBJ-1. Kind: contradictory.

- The glossary says that automatic storage exists from the declaration. STO-1 creates the object at the pass through the declaration.
- JMP-6 and `goto-past-uninitialized-scalar.cpp` let a program skip the declaration and then write the variable.
- Tidal does not say whether the object exists after the skip, or when its storage starts. This is D13.
- Needed: the state of an object whose declaration a jump skips.
- Suggestion: one rule saying that the object exists from block entry (or from the label) with no value, and a glossary fix.

**X3-3.** Page: `declarations/constexpr.md`, `foundations/constant-evaluation.md`, `declarations/storage-linkage.md`. Rules: CX-1, CE-21, STO-3, SINIT-2. Kind: missing.

- `result-unset-member.cpp` declares `constinit Pair pair;` at namespace scope. STO-3 zeroes it first, yet every compiler rejects the constant because `second` has no value.
- No rule says that a constructor's default initialization of a member removes the zero that STO-3 wrote, or that the zero does not count as a value for CE-21. This extends D14.
- Suggestion: say so in CE-21 or in the new default-initialization rule.

**X3-4.** Page: `declarations/initialization/static.md`, `foundations/objects.md`. Rules: SINIT-2, SINIT-3, SINIT-4, OBJ-1, OBJ-2. Kind: ambiguous.

- SINIT-2 says that a variable holds zero before its initializer runs. For a class type, OBJ-1 starts the lifetime when the constructor completes, and OBJ-2 makes a read outside the lifetime undefined.
- Needed: whether a dynamic initializer can read a member of a class-type static object before its constructor runs. Impact claim 2 depends on it.
- Suggestion: a sentence in SINIT-2 that limits "holds zero" to scalars, or one that extends it to class members.

**X3-5.** Page: `declarations/initialization/forms.md`. Rules: INIT-2. Kind: missing.

- The list of zeros covers integers, floating-point types, `bool`, and pointers. It omits enumerations, pointers to members, and `std::nullptr_t`.
- P5 needs the zero of every scalar type.
- Suggestion: complete the list, or define "the zero of a scalar type" once and cite it.

**X3-6.** Page: `declarations/volatile.md`, `foundations/translation.md`. Rules: TR-7, the volatile principle, VOL-2. Kind: ambiguous.

- TR-7 makes each write of a volatile object observable. The volatile page says that a class object is volatile only once it exists. It says nothing about the initialization of a volatile scalar.
- Needed: whether initializing a volatile scalar, by an initializer or by P5's zero, is an observable write.
- Suggestion: a rule on the volatile page.

**X3-7.** Page: `declarations/attributes.md`, `expressions/new-delete.md`. Rules: ATTR-P1, the attribute Syntax note, NEW-P3, NEW-10. Kind: missing.

- The Syntax note places an attribute before a declaration or statement, or after a name in a declarator. A `new` expression has neither.
- ATTR-P1 says that an attribute can apply to a type, but no rule says where it is written.
- No rule says that a program can call an allocation function directly to get storage with no object in it.
- So P5 cannot offer an opt-out for `new`.
- Suggestion: a list of attribute positions, and a rule for a direct call of `operator new`.

**X3-8.** Page: `classes/unions.md`. Rules: UNI-4, CE-21, Q-0117. Kind: missing.

- UNI-4 covers list initialization, designators, default member initializers, and `{}`.
- It does not say what default initialization does to a union, or to a union member that a constructor does not name.
- P5 chose to activate no member, without a rule to cite.
- Suggestion: add the default-initialization case to UNI-4.

**X3-9.** Page: none (D32). Rules: TR-P5, TR-7. Kind: outside the spec.

- A proposal needs a cost section. The cost here is the zero writes, mostly for large arrays and `new T[n]`, less the writes that an implementation can drop under TR-P5.
- Tidal has no cost model.
- Suggestion: the front end could count default-initialized scalars per function as a stand-in.

**X3-10.** Page: `declarations/initialization/forms.md` (Evidence). Rules: INIT-1. Kind: outside knowledge.

- I knew from outside Tidal that Clang and GCC have an option that zero-fills automatic variables (`-ftrivial-auto-var-init=zero`). I used it in one probe.
- With the option, Clang gives 0 at `-O2`. GCC still gives 5 at `-O2` for the one-path-write probe.
- So the vendor option is a hardening mode. It is not a language rule, and GCC's optimizer still treats the read as undefined.
- Suggestion: an Evidence note on INIT-1, if the maintainer wants implementation options recorded.

**X3-11.** Page: `declarations/initialization/forms.md`, `expressions/new-delete.md`, `classes/constructors.md`. Rules: INIT-1, NEW-3, CTOR-12, OBJ-2. Kind: needs example.

- Every test of these rules is a constant-evaluation probe. `copy-indeterminate-unsigned-char-run.cpp` cannot observe anything.
- No test shows what a run-time read does today, so the impact section rests on my own probe: Clang 22 gives 232 and GCC 16 gives 5 at `-O2`.
- Suggestion: an Evidence note with the `-O2` divergence, or a run test that accepts any result and records it.

**X3-12.** Page: `classes/constructors.md`, `declarations/declarators.md`. Rules: CTOR-P2, CTOR-4, CV-3, NEW-11. Kind: ambiguous.

- CTOR-P2 says that a `const` object needs a value because nothing can give it one later.
- The tests reject `const int x;` and `const S s;` at namespace scope, where STO-3 already gives zero. So the stated reason does not explain the rule.
- No reason is given for CV-3 at all.
- Needed: the real reason, so that P5 can decide whether to keep these rejections.
- Suggestion: a sentence in CTOR-P2, and a principle for CV-3, saying that a `const` object must state its value.

**X3-13.** Page: `declarations/initialization/static.md`, `statements/if.md`. Rules: SINIT-1, IF-18. Kind: needs example.

- The run-time meaning of a static initializer depends on whether its constant evaluation succeeds.
- So any change to a constant-evaluation rule changes which `if consteval` branch runs, and when the variable is initialized.
- Tidal states both rules, but no example or note connects them. A designer finds this change of meaning only by building it.
- Suggestion: an example under SINIT-1 with `if consteval` in the initializer.
