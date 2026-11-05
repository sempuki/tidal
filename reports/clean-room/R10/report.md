# Predict batch: objects

- Date: 2026-11-05
- Role: clean-room reader
- Persona: programmer new to C++
- Batch: `build/exercises/R10-objects` (61 programs for `foundations/objects`), without test names
- Score: 61/61 (`score.txt`); 60 of the 61 right outcomes cite one of the test's rules

The reader returned this report as a message, and the integrator saved
it here with the reader's `answers.json` (`answers-R10-objects.json`).

Outcome counts: runs 30, reject 21, run-fail 5 (p03, p04, p09, p35,
p43), undefined 4 (p31, p32, p37, p56), compiles 1 (p46, no `main`).
Where the specification records that the compilers differ (Q-0368 on
p01 and p02, Q-0115 on p12 and p33, Q-0374 on p42), the reader answered
`reject`, the outcome of the rules, and named the question. No answer
has a `principles` list, because a rule decided every outcome.

## Findings

### R10-1

- Program: p03
- Rules: OBJ-2, OBJ-3
- Needed: whether `-fsanitize=address` stops a read through a pointer to
  an automatic variable whose block has ended. The objects page does not
  say. The only statement of this sanitizer result is in the notes of
  LAM-19 (a lambda's reference capture). The answer `run-fail` relies on
  outside knowledge that the address sanitizer checks stack use after
  scope by default on Clang and GCC.
- Suggestion: add a note under OBJ-2 that the address sanitizer of Clang
  and GCC stops a read through a pointer to a variable whose block has
  ended, and name the test.

### R10-2

- Program: p35
- Rules: OBJ-2, OBJ-5
- Needed: whether `-fsanitize=address` stops a read of a member of an
  object after `delete`. The NEW-5 notes cover a double `delete` and a
  mismatched `delete[]` only. The answer `run-fail` relies on outside
  knowledge (a heap use after free report).
- Suggestion: add a note under OBJ-2 or NEW-5 that the address sanitizer
  stops a read through a pointer after `delete`.

### R10-3

- Program: p05
- Rules: OBJ-1, UNW-2
- Needed: when the lifetime of an object made by a delegating
  constructor begins. The OBJ-1 table says "That is the constructor,
  with the body of a delegating constructor". A new reader can take
  this to mean the lifetime begins only after the delegating body
  ends. UNW-2 says the object is complete once the target constructor
  completes, so its destructor runs if the delegating body throws. The
  two pages read as if they disagree.
- Suggestion: state in OBJ-1 that the object counts as complete when the
  target constructor completes, for the purpose of destruction, and
  cite UNW-2 there.

### R10-4

- Program: p12
- Rules: CE-22, CPY-12, OBJ-2
- Needed: whether the return of a local class object in a `constexpr`
  function copies or moves the object during constant evaluation, or
  whether copy elision skips the read. CE-22 says the move of a
  returned local reads each scalar part, which settles the rule. The
  Q-0115 split-test table lists only a plain copy of a member and of an
  array element. The page does not say whether GCC also accepts the
  return case.
- Suggestion: add the return of a local with an element that has no
  value to the Q-0115 notes, with each compiler family's result.

### R10-5

- Program: p31
- Rules: OBJ-11, OBJ-12
- Needed: whether taking the address of a `thread_local` object
  (`&watcher`) counts as a use that starts its deferred construction.
  OBJ-11 says "before the thread first uses the object". The answer
  assumes that taking the address is a use. If it were not, `watcher`
  would never be constructed and the program would be well defined.
- Suggestion: say in OBJ-11 or SINIT-6 which expressions count as a
  use, and that taking the address counts.

### R10-6

- Program: p40
- Rules: OBJ-11, SINIT-6
- Needed: whether OBJ-11 covers a `static thread_local` data member of a
  class. OBJ-11 speaks of a `thread_local` object "at namespace scope".
  The program accepts either construction point, so the outcome did not
  depend on the answer.
- Suggestion: state whether a static data member defined at namespace
  scope counts as a namespace-scope object for OBJ-11.

### R10-7

- Program: p34
- Rules: OBJ-2, CE-22, LSUP-10, LSUP-11
- Needed: the reject rule for a read of storage from `std::allocator`
  where no object has begun its lifetime. CE-22 covers a scalar object
  that has no value. In p34 no object exists at all in element 1. OBJ-2
  names the read as outside a lifetime, and OBJ-2 says constant
  evaluation rejects it, so the outcome was clear. The `reject` rule
  that a test would cite was not.
- Suggestion: name in CE-22, or in a CE rule beside it, the read of
  storage that holds no object, and cite it from LSUP-11.

### R10-8

- Programs: p01, p02, p12, p33, p42
- Rules: OBJ-1, OBJ-2, CE-22, CTOR-12
- Needed: the answer form for a program that a recorded divergence
  covers. The README allows `depends` or the outcome of the rules. These
  answers give `reject`, the outcome of the rules, and name the
  question in `why`. No gap in the rules.
- Suggestion: none for the specification. The README could say which of
  the two answers the key prefers.

### R10-9

- Program: p43
- Rules: RET-6, JMP-10
- Needed: the type that `decltype(auto)` gives `return (value);`. The
  `auto` page table says a parenthesized lvalue gives an lvalue
  reference. JMP-10 makes the name in a `return` an xvalue, also in
  parentheses, and RET-6 gives an rvalue reference. A reader of the
  `auto` page alone gets `int&`, and then the binding to `int&&` would
  be rejected.
- Suggestion: in the `decltype(auto)` row of the `auto` page table,
  point to JMP-10 and RET-6 for a `return` statement.
