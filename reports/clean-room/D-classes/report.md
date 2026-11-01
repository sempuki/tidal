# Diagnose exercise: D-classes

- Date: 2026-11-01
- Role: clean-room reader
- Persona: programmer new to C++
- Inputs: files under `specification/` and the four batch directories.
  Fixes were compiled with `g++ -std=c++23 -pedantic-errors` (GCC 16.2.1).

## Batches

| Batch | Programs | Fixes written | Fixes that compile |
|---|---|---|---|
| D-virtual | 54 | 54 | 54 |
| D-handlers | 19 | 19 | 19 |
| D-unions | 46 | 46 | 46 |
| D-conversion-functions | 24 | 24 | 24 |
| Total | 143 | 143 | 143 |

The only program with `main` is D-conversion-functions p13. Its fix runs
and exits with status 0.

For one program, D-conversion-functions p13, the specification says the
program is valid (DC-14).

## Findings

### DC-1

- Batch and program: D-virtual p28
- Rules: VIRT-10, CE-8
- Kind: hard to find
- Needed: the reject rule for a failed `dynamic_cast` of a reference
  during constant evaluation. VIRT-10 says the cast "is not a constant
  expression", but VIRT-10 is class `required`. The reject rule is CE-8
  on the constant-expressions page, and VIRT-10 does not link to it.
- Suggestion: in VIRT-10, link the sentence about constant evaluation to
  CE-8.

### DC-2

- Batch and program: D-virtual p30
- Rules: VIRT-12, CE-9
- Kind: message mismatch
- Needed: GCC reports a call to a function that is not `constexpr` and
  names a runtime library function for pure calls. The specification
  does not name that function. The message points at CE-2, but the
  program breaks VIRT-12 (a pure virtual call from a constructor) through
  CE-9.
- Suggestion: in the VIRT-12 Evidence, say that GCC reports this case as
  a call to a library function that is not `constexpr`.

### DC-3

- Batch and program: D-virtual p37
- Rules: VIRT-12, CE-9
- Kind: message mismatch
- Needed: GCC reports a change to heap storage "from outside current
  evaluation". That wording points at CE-5. The program does `delete[]`
  of an array of `D` through a `B*`, which VIRT-12 makes undefined.
- Suggestion: add the GCC wording for this case to the VIRT-12 Evidence
  or to the CE-9 table, so a reader can map the message to VIRT-12.

### DC-4

- Batch and program: D-virtual p19
- Rules: VIRT-2
- Kind: message mismatch
- Needed: the first error line names the base function `B::f` and its
  column points into the base class. The broken rule is the last
  paragraph of VIRT-2 (a static member with the name and parameters of
  a base virtual function). VIRT-2 opens with a list of things that must
  not be declared virtual, so the static case is easy to miss.
- Suggestion: make the static-member case its own bullet or its own rule,
  and give a short example.

### DC-5

- Batch and program: D-virtual p01, p29
- Rules: VIRT-9
- Kind: message mismatch
- Needed: GCC calls `= 0` an "initializer" for a function. The
  specification calls it a pure specifier, and the glossary has no entry
  that connects the two words.
- Suggestion: add a sentence or an Evidence note saying that compilers
  can report a misplaced `= 0` as an initializer.

### DC-6

- Batch and program: D-virtual p34, p43, p45
- Rules: VIRT-15, VIRT-4
- Kind: needs example
- Needed: VIRT-15 rejects a return type that is "not covariant", and the
  conditions for covariant are in VIRT-4, a `required` rule. For p34 the
  condition "accessible in the class of the override" took thought: the
  base `B` is private in the returned class `X`, and the override is in a
  third class `D`.
- Suggestion: give VIRT-15 one example for each failed VIRT-4 condition:
  an inaccessible base, an ambiguous base, and an added cv-qualifier.

### DC-7

- Batch and program: D-handlers p10, p18
- Rules: TRY-5
- Kind: unclear
- Needed: the first sentence of TRY-5, "a complete type that an object or
  a pointer can have", is hard to parse. A pointer to `void` is valid
  (TRY-2 matches it), but `void` is not complete, so the sentence seems to
  contradict TRY-2.
- Suggestion: drop the first sentence and keep the bullet list, or
  say "an object type, a pointer, or an lvalue reference" and then list
  the exceptions.

### DC-8

- Batch and program: D-handlers p10, p18 (probe: a handler for a
  reference to an incomplete class)
- Rules: TRY-5
- Kind: missing
- Needed: TRY-5 forbids an incomplete class by value or by pointer. TRY-5
  does not say what happens to an lvalue reference to an incomplete class.
  GCC rejects that handler with "invalid use of incomplete type".
- Suggestion: add "or by reference" to the incomplete-class bullet if the
  matrix agrees, or record a question.

### DC-9

- Batch and program: D-handlers p02
- Rules: TRY-5
- Kind: message mismatch
- Needed: GCC reports a parse error ("expected unqualified-id"), and the
  rule names a qualified name in the handler parameter.
- Suggestion: in the TRY-5 Evidence, say that GCC reports a qualified
  handler name as a syntax error.

### DC-10

- Batch and program: D-unions p11
- Rules: BF-11, BF-7
- Kind: wrong
- Needed: BF-11 says the initializer must not need a narrowing conversion
  "from the declared type of the bit-field". The conversion goes from the
  initializer's type to the declared type. The sentence names the wrong
  direction.
- Suggestion: say "a narrowing conversion to the declared type of the
  bit-field".

### DC-11

- Batch and program: D-unions p02, p26, p38
- Rules: CE-6, CE-9, UNI-6
- Kind: unclear
- Needed: a read of an inactive member during constant evaluation is
  rejected by CE-6 directly, and also by CE-9 through UNI-6. The unions
  page names only UNI-6 and does not link to CE-6. I could not tell which
  rule a reader should name.
- Suggestion: say in UNI-6 that CE-6 is the specific reject rule for a
  read during constant evaluation, or merge the two.

### DC-12

- Batch and program: D-unions p04
- Rules: UNI-13, OBJ-2, CE-9
- Kind: hard to find
- Needed: UNI-13 says the parts of a new active member that the
  assignment does not write "have no value". UNI-13 does not say what a
  read of such a part does. The answer is OBJ-2, on the objects page.
- Suggestion: in UNI-13, link "have no value" to OBJ-2.

### DC-13

- Batch and program: D-unions p19, p21, p25
- Rules: UNI-4, UNI-6, UNI-13, CTOR-23
- Kind: message mismatch
- Needed: for p19 and p25 GCC says "accessing uninitialized member". The
  specification says the union has no active member. For p21 GCC reports
  a read of `W::n` "instead of initialized `W::i`". The specification
  says the assignment `w.n.x = 4` does not make `n` active, because the
  default constructor of `N` is not trivial (CTOR-23), and the assignment
  uses an object that does not exist (UNI-6). To see this I needed
  CTOR-23 from the constructors page.
- Suggestion: add an Evidence note that GCC calls a union with no active
  member "uninitialized". In UNI-13, give an example with a member whose
  default member initializer blocks activation.

### DC-14

- Batch and program: D-conversion-functions p13
- Rules: CVF-1
- Kind: message mismatch
- Needed: GCC 16 rejects `operator auto()` under `-pedantic-errors`.
  CVF-1 says `auto` is a valid target type (Q-0059), so the
  program breaks no Tidal rule. The batch says every program breaks a
  rule.
- Suggestion: none for the specification text. The batch builder should
  skip or mark programs that a rule of class `required` says are valid.

### DC-15

- Batch and program: D-conversion-functions p04, p06, p14
- Rules: CVF-12, ICS-7, ICS-8
- Kind: unclear
- Needed: CVF-12 says "An ambiguous conversion is not valid". ICS-7 says
  the ambiguous sequence still makes a candidate, and ICS-8 rejects only
  when the chosen function has it. In p04 the call `take(Source{})` picks
  `take(double)` over `take(...)`, and only then fails. Read alone,
  CVF-12 suggests the candidate would be dropped and `take(...)` would
  win.
- Suggestion: reword CVF-12 to say the program is rejected when the
  chosen function needs an ambiguous conversion, and add the p04 shape
  as an example.

### DC-16

- Batch and program: D-conversion-functions p14
- Rules: ICS-7
- Kind: outside knowledge
- Needed: to decide that `Source::operator Target()` and
  `Target(Source&)` tie for the lvalue `s`, I compared the two sequences
  with my own sense of reference binding. I did not read ICS-37 or
  OVL-27 to OVL-30, which ICS-7 names for a tie.
- Suggestion: add an example to CVF-12 with a constructor and a
  conversion function that tie, and say which rule breaks the tie or
  fails to.

### DC-17

- Batch and program: D-conversion-functions p20, p21
- Rules: CVF-11, CVF-2
- Kind: message mismatch
- Needed: GCC says "no match for operator+" and "no match for
  operator==". CVF-11 says other conversions "must not use" the
  `explicit` function. The message talks about operator lookup, and the
  link to the conversion function is not visible.
- Suggestion: add an Evidence note that a compiler reports a built-in
  operator with an `explicit` conversion as a missing operator.
