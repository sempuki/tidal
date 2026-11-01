# Roadmap, round 2: what to do after phase 4, from a working programmer's view

Date: 2026-10-31. Decision 0029. This report comes from a clean-room reader in the role of a programmer who wants to write ordinary programs, generic code included, from Tidal alone. The reader read AGENTS.md, CLAUDE.md, `documentation/` (charter, roadmap, status, exercises, decision 0041), `specification/` (the templates chapter, objects, language-support, odr, new-delete, casts, unions, additive, concepts), `guides/`, `questions/` (titles), `reports/clean-room/` (the round-1 programmer report, W0, front-end round 7), and `tests/`. The reader counted with `grep` and `wc`.

The reader also ran three sketches through the front end's interpreter (`frontend/target/release/tidal-fe run /dev/stdin`). The reader ran one five-line check on the host GCC and Clang (`-fsyntax-only`, from stdin). No files were written. Where the report relies on general knowledge, it says so.

## What blocks programs today

Phase 4 closed the biggest gap from round 1. I can now write a function template with a concept, a class template with out-of-class members, `this->` for dependent bases, forwarding through `static_cast<T&&>` (packs page, PCK rules near line 331), and a generic lambda. The templates chapter has 358 rules in 11 pages and 961 tests.

Three things still stop ordinary programs:

- **Objects in raw storage.** A growable container or a `Result<T, E>` must build an object in storage it already holds, then destroy it, then reuse the storage. No rule tells me how to do that safely.
- **Templates across files.** ODR-7 and ODR-10 list classes, enumerations, `inline` functions, and `inline` variables. `odr.md` never mentions the word "template". From these rules I cannot tell whether a header holding a function template may be included in two units.
- **The vocabulary a generic program reaches for.** I can write `move`, `remove_reference`, and `same_as` myself. I cannot write "is trivially destructible" or "construct in place" without help from outside the core rules.

## The sketches

### Sketch 1: a generic sort with a concept

```cpp
template <class T> struct RemoveRef { using type = T; };
template <class T> struct RemoveRef<T&> { using type = T; };
template <class T> constexpr typename RemoveRef<T>::type&& move(T&& t) noexcept {
  return static_cast<typename RemoveRef<T>::type&&>(t);
}
template <class T> concept Ordered = requires (const T& lhs, const T& rhs) { lhs < rhs; };
template <Ordered T> void sort(T* first, T* last) { /* swap through move */ }
```

The concepts page (CPT-P1, CPT-P3, CPT-33), the specialization page, and the packs page gave me everything for this sketch. The interpreter ran it and returned 0.

I got stuck in two places:

- I wanted `{ a < b } -> std::convertible_to<bool>`. CPT-33 needs a concept after `->`, and Tidal has no library concepts. I wrote my own concept. Nothing tells me whether my concept matches what everyday code expects (general knowledge: the library's version is subtler than a plain conversion check).
- I wanted iterator concepts and `swap` found through argument-dependent lookup. I wrote pointer-only code and gave up on iterators. Lookup is specified, so this is a library gap. The core rules are fine here.

### Sketch 2: a growable vector

```cpp
using size_t = decltype(sizeof 0);
void* operator new(size_t, void* p) noexcept { return p; }   // where does this come from?
template <class T> class Vec {
  T* data = nullptr; size_t n = 0, cap = 0;
  void grow() {
    T* d = static_cast<T*>(::operator new(c * sizeof(T)));     // NEW-19
    new (d + i) T(static_cast<T&&>(data[i]));                   // d + i: what array?
    data[i].~T();                                               // DTOR-6
  }
};
```

I got stuck at each commented line:

| Step | What Tidal gives | Where I get stuck |
|---|---|---|
| Get raw storage | NEW-19: `::operator new` exists without a declaration | Nothing for over-aligned `T` (no aligned form, no `std::align_val_t`) |
| Build an object there | NEW-20: a placement `new` needs a declared placement form | Nothing gives that form. No test declares `operator new(size_t, void*)`. The usual form is a library form (general knowledge). Host GCC and Clang both accept my own definition, but no rule says I may write it. |
| Index into the storage | ADD-6 and ADD-10: arithmetic is defined only within an array | Storage from `::operator new` holds no array, so reading ADD-10 literally makes `d + i` undefined |
| Destroy an element | DTOR-6: an explicit destructor call ends the lifetime | Clear |
| Reuse the storage | OBJ-10: reuse ends the old object | Nothing says whether an old pointer may reach the new object |
| Move without throwing | UN-18 gives `noexcept(e)` | Clear. I can pick move or copy with `if constexpr`. |

The interpreter stopped at the cast from `::operator new` with "unsupported at line 10: cast of unknown operand".

### Sketch 3: an error layer, `Result<T, E>` with `map`

```cpp
template <class T> class Result {
  bool ok; union { T value; Error err; };
  ~Result() { if (ok) value.~T(); }
  template <class F> auto map(F f) const -> Result<decltype(f(value))>;
};
```

Throwing and catching follow the error-handling guide and THR and TRY rules without trouble. The type itself gets stuck:

- **Copy and assignment.** Copying, or assigning a value over an error, means making a nontrivial union member active. UNI-5 covers only the built-in or trivial assignment. A nontrivial member needs construction in place, which is the same gap as in sketch 2.
- **A trivial destructor when `T` has one.** CNS-19 and CNS-21 (and Q-0266) let me write two constrained destructors. The constraint needs "T is trivially destructible", and I cannot write that. CPY-19 and CLS-19 mark the property lists *(class properties referenced)*. No builtin is specified. `grep` finds no `__is_` or `__builtin` in `specification/` or `tests/`.
- **Translating library failures.** A layer that turns failures into codes catches allocation failure. The type that failure throws stays *(library referenced)* on the new-delete page (line 58) and on `language-support.md`.

The interpreter stopped with "class layout the front end does not model". The anonymous union inside a class template is the likely cause (inference).

## 1. Ranked items

| # | Item | Size | Risk | Changes scope |
|---|---|---|---|---|
| 1 | Objects in raw storage | about `objects.md` again (10 to 14 rules) | high | no (objects are in scope) |
| 2 | Language-support types for in-place construction and failure | about half of `language-support.md` | medium | yes |
| 3 | Templates across translation units | 3 to 5 rules | low | no |
| 4 | Write suites for generic code, and the pending suites | one session per suite | low | no |
| 5 | Class properties: the full trivial and standard-layout lists | about `types.md` | medium | no (lifts deferred S and T) |
| 6 | Front end, round 8, on the sketches | large | medium | no |
| 7 | Guides for generic code | about `resource-owner.md` each | low | no |
| 8 | Keep the preprocessor, I/O, and the general library out | none | low | yes (keeps out) |
| 9 | Status page catches up with phase 4 | tiny | none | no |

### Item 1. Objects in raw storage (new rules on `objects.md`, or a new Foundations page)

**What it is.** These rules cover how a program gets an object into storage it owns, and what a pointer to that storage may do. The scope is:

- a `new` expression with a storage argument creates the object, and its result points to the object,
- arithmetic over storage from an allocation function or a byte array (ADD-6 has only arrays),
- whether a pointer to an old object reaches a new object in the same storage (OBJ-10 says only that the old lifetime ends),
- reading an object through a glvalue of another type,
- reading any object as `unsigned char`,
- making a nontrivial union member active (UNI-5),
- allocation for over-aligned types.

**Evidence.**

- `objects.md` has 11 rules and none creates an object in given storage.
- CAST-6 says a `reinterpret_cast` to `T&` "refers to the operand as type `T`", and the casts page has no `undefined` rule about access through the wrong type. A reader concludes that reading an `int` as a `float` is fine. General knowledge says that read is undefined and that optimizers use the rule. The `-O2` report compilers (decision 0043) can show the effect.
- The test `classes/copy-move/trivially-copyable-bytes.cpp` reads an object as `unsigned char`, and no rule allows that read.
- No test declares the usual placement form.
- W8's task "a container that stays valid if an element constructor throws" and W4's "bump-pointer arena" both need these rules (`documentation/exercises.md`).

**What it unblocks.** Containers, optional and result types, arenas, the W4 and W8 suites, and the interpreter's model of storage.

**Size.** About `objects.md` again: 10 to 14 rules and about 40 tests.

**Risk.** High, for three reasons:

- The rules are subtle.
- Sanitizers do not report type-based access, so the evidence comes from constant evaluation and from `-O2` differences.
- Similarity risk is moderate, because this area of the reference has well-known phrasing (general knowledge).

A source reader's inventory before drafting fits the method controls on the roadmap page.

### Item 2. Language-support types for in-place construction and failure (scope change)

**What it is.** Decision 0041 would grow by a few rows of shapes that the core language depends on:

- the placement form `operator new(size_t, void*)`, and whether a program may declare it,
- the aligned allocation forms and their alignment type,
- the class that a failed allocation throws.

Item 2 would keep `<utility>`, `<type_traits>`, and `<concepts>` out, and say so on the page.

**Evidence.**

- `language-support.md` line 15 leaves the allocation failure type *(library referenced)*.
- NEW-20 makes placement `new` depend on a declaration that no page gives.
- 97 test files declare pieces of `namespace std`. 44 declare `initializer_list`, 21 declare the ordering types, and 2 declare `remove_reference` and `move`.
- 66 tests move with `static_cast<T&&>`, and 2 use `std::move`.

**The case for and against.**

| For | Against |
|---|---|
| These forms are how the core rules create and fail to create objects. Decision 0041's reasoning applies to them as it does to `initializer_list`. | Every row moves the line between core and library, and the charter keeps the library out. |
| Without these forms, sketch 2 and sketch 3 cannot be written from Tidal. | Whether a program may define the usual placement form is a library rule (general knowledge). Tidal would have to state a library rule to answer it. |
| Keeping `<type_traits>` and `<concepts>` out stays defensible, because I wrote `RemoveRef` and `Ordered` from the templates pages alone. | Some traits cannot be written in the core language (triviality, sketch 3), so programs still meet a wall. Item 5 is the cheaper answer. |

**Recommendation.** Add the three shapes. Keep the trait and concept headers out. Record the "may a program declare the placement form" doubt as a question.

**Unblocks.** Item 1's tests, sketch 2, and W8.

**Size.** About half of `language-support.md`.

**Risk.** Medium. Compilers check user-declared `std` shapes differently (LSUP-1's Evidence), so expect overrides.

### Item 3. Templates across translation units (`odr.md`)

**What it is.** These rules say which template definitions may appear in several units and must match: every kind of template (class, function, variable), the members that a class template defines (functions and static data), and the partial and explicit specializations.

**Evidence.**

- `grep -ci template specification/foundations/odr.md` gives 0.
- ODR-7 and ODR-10 list only classes, enumerations, `inline` functions, and `inline` variables.
- The instantiation page covers explicit instantiation across units (INST-P3, INST-35) and points of instantiation (INST-32).
- The specialization page says its tests "do not cover a use in another translation unit" (near line 557).

**Unblocks.** Any program with a header of templates, which is how generic code ships (general knowledge), and the "program across files" guide from round 1.

**Size.** 3 to 5 rules, with multi-unit tests (decision 0014).

**Risk.** Low. Compilers agree on the common cases, and the no-diagnostic class already exists.

### Item 4. Write suites for generic code, and the pending suites

**What it is.** A new suite W15, "Generic code", with these tasks:

- a container class template with an `initializer_list` constructor and range-for support,
- a sort constrained by a concept the program writes,
- a `Result<T, E>` with `map`,
- a type trait written by partial specialization.

Item 4 also means running W8 and W4, which stress item 1.

**Evidence.**

- `documentation/exercises.md` lists W0 to W14. None targets the templates pages.
- `reports/clean-room/` holds only the W0 report (`java-programmer/`) among the Write suites.
- 358 template rules have had no task-level use.

**Unblocks.** Findings that a task needs and no rule states (decision 0045), and the guides in item 7.

**Size.** One session per suite.

**Risk.** Low. The tasks may not use `#include`, so W15 depends on item 2's shapes.

### Item 5. Class properties: the full lists (deferred S and T)

**What it is.** The complete conditions for trivially copyable, trivial destructor, standard-layout, and implicit-lifetime types, plus a decision on how a program observes them.

**Evidence.**

- CLS-19 and CPY-19 say "others that this page does not list yet *(class properties referenced)*". These are 2 of the 13 remaining markers in `status.md`.
- CNS-21 and Q-0266 make triviality depend on constraints, so the templates chapter now leans on these lists.
- Sketch 3 needs "trivially destructible" in a constraint.
- The roadmap's Deferred section holds S and T until it has "a decision on how to observe library-dependent properties".

**Unblocks.** Conditionally trivial wrappers, byte copies (CPY-19), and item 1's rules for storage reuse.

**Size.** About `types.md`.

**Risk.** Medium. Observing a property needs either library traits (out of scope) or compiler builtins (not specified). The tests can observe triviality through unions and constant evaluation, as `tests/templates/constraints/destructor-trivial.cpp` already does.

### Item 6. Front end, round 8, measured on the sketches

**What it is.** Finish templates in the front end. Add the three sketches, or the W15 programs, as run tests for the interpreter.

**Evidence.**

- `reports/clean-room/rust-frontend/conformance.tsv` has 379 template rows. The front end passes 58 of them, with 168 unsupported, 141 missed rejects, and 8 false rejects.
- Sketch 1 ran. Sketch 2 stopped at a cast of `::operator new`'s result. Sketch 3 stopped at the class layout.
- Round 8 is in progress, and the working tree has uncommitted changes in `frontend/src/`.

**Unblocks.** A reader can check a generic program without a compiler. The interpreter can stop at undefined behavior in a container (decision 0048), which item 1 needs for evidence that sanitizers cannot give.

**Size.** Large.

**Risk.** Medium. The interpreter's model of storage must wait for item 1's rules, or it will encode guesses.

### Item 7. Guides for generic code

**What it is.** Three how-to guides, each about the size of `guides/resource-owner.md` (267 lines):

- writing your own container (round 1, guide 2),
- writing a constrained algorithm, with hand-written `move`, traits, and concepts,
- organizing templates across files (round 1, guide 7).

**Evidence.**

- `guides/index.md` lists five guides, none of them about templates.
- Decision 0045 says a guide waits for a rule that it needs.

**Unblocks.** The everyday path through 358 template rules. Sketch 1 needed rules from five pages.

**Size.** One session per guide.

**Risk.** Low. The container guide waits for items 1 and 2, and the files guide waits for item 3.

### Item 8. Keep the preprocessor beyond `#include`, I/O, and the general library out (scope, kept out)

**What it is.** Confirm the charter's exclusions for the next phase, and add one line to `exercises.md` that explains how exercise programs report results without them.

**The case for and against.**

| Topic | For adding | Against adding |
|---|---|---|
| `#define`, `#if`, `assert` | Real programs use `assert` and conditional compilation for platform switches (general knowledge). | The topic is large, has little to do with the language's meaning, and carries high similarity risk. Tests already report through `static_assert` and `main`'s return value. |
| I/O (`printf`, streams) | Programs print. | All library. A program can declare a C function under `extern "C"` (linkage-specifications page) if a task needs output. |
| `<type_traits>`, `<concepts>`, `<utility>` | Every generic program uses them (general knowledge). | Most of their contents can be written from the templates pages (sketch 1). The parts that cannot be written belong in item 5. |
| Coroutines, threads, modules | Common in newer code. | They need a library protocol, a memory model, or support that differs across the matrix (round 1, section 3). None of them blocked a sketch. |

**Recommendation.** Keep all four out.

**Size.** None.

**Risk.** Low.

### Item 9. The status page catches up with phase 4

**What it is.** Add rows for the 11 templates pages, and for the guides, to the table that `tools/status_table.py` generates.

**Evidence.**

- `grep -c templates/ documentation/status.md` gives 0.
- The table ends at the exceptions pages, so the counts of 358 rules and 961 tests do not appear.

**Unblocks.** A reader who judges coverage from `status.md`.

**Size.** Tiny.

**Risk.** None.

## 2. Suggested order

9 (now), then 3, 2, 1, 5, 4, 7. Item 6 runs beside the others. Its storage model waits for item 1. Item 8 is a decision to record with item 2. Items 3 and 9 can start at once, and W15 can start once item 2 gives the shapes.

## 3. Doubts for the clean-room writers

- Whether a program may define `operator new(size_t, void*)` itself. GCC and Clang accept the definition, and Tidal is silent.
- Whether `p + i` over storage from `::operator new` is defined. Read literally, ADD-10 says no.
- Whether a pointer to an old object reaches a new object in the same storage. OBJ-10 says only that the old lifetime ends.
- Whether reading an object through a glvalue of another type is undefined. CAST-6 reads as if the access is allowed.
- How an exercise program observes triviality. The test `destructor-trivial.cpp` observes it through a union. Programs have no general way.
