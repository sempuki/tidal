# Front end update: round 10, part 2 (the interpreter)

Date: 2026-11-03. Decisions 0029, 0035, 0047, 0048, 0051 (item FE).
Role: clean-room reader. Inputs: the files under `specification/`,
`documentation/`, `tests/`, `questions/`, `frontend/`, and `tools/`,
the previous reports in this directory, and
`reports/clean-room/roadmap-3/implementer.md` (items 1, 4, and 7).
g++ and clang++ were run twice on short programs of my own: once to see
whether a call of a `noexcept` function through a pointer without
`noexcept` runs (finding G96), and once to see the `-Wall` warning for
`i++ + i++` while I tested the SEQ-4 check. No crates were added.

Part 2 covers items 1, 4, and 7 of the roadmap. Part 1 (report-10)
covered the check side.

## 1. Conformance

### Checks

`tidal-fe conform ../tests`. "Pass" counts a passed check, as in
report-10, so it includes "Differs from EDG only" and a run test whose
check passed, whatever its run gave.

| Run | Tests | Pass | False reject | Missed reject | Unsupported |
|---|---|---|---|---|---|
| start of this part (report-10, after) | 5480 | 5018 | 63 | 246 | 153 |
| after | 5480 | 5048 | 62 | 245 | 125 |

By top directory, tests whose check passes:

| Directory | Start of this part | After |
|---|---|---|
| classes | 888/960 | 895/960 |
| declarations | 932/1023 | 933/1023 |
| exceptions | 146/146 | 146/146 |
| expressions | 1107/1197 | 1118/1197 |
| foundations | 769/826 | 777/826 |
| statements | 215/224 | 216/224 |
| templates | 961/1104 | 963/1104 |

The check side gained 30 tests. Constant evaluation now runs pointers to
members, `new` with a placement form inside `std::construct_at`, and
contextual conversions of a class to `bool`.

`python3 -I tools/frontend_check.py` (without `--update`) reports:

```
frontend: 4918/5480 pass, 5048/5480 pass their check, 0 worse, 135 better, 0 new (0 not passing)
frontend: 2/2 interpreter-only expectations met
```

It exits 0. At the start of this part it reported 4783 and 5018.

### Run tests

| Run | Tests | Pass | Wrong result | Missed undefined behavior | Unexpected undefined behavior | Unsupported | Check did not accept |
|---|---|---|---|---|---|---|---|
| start of this part | 1242 | 986 | 6 | 4 | 2 | 225 | 19 |
| after | 1242 | 1092 | 1 | 0 | 0 | 131 | 18 |

By top directory, after:

| Directory | Tests | Pass | Wrong result | Missed undefined behavior | Unexpected undefined behavior | Unsupported | Check did not accept |
|---|---|---|---|---|---|---|---|
| classes | 291 | 236 | 0 | 0 | 0 | 53 | 2 |
| declarations | 193 | 176 | 0 | 0 | 0 | 16 | 1 |
| exceptions | 97 | 91 | 0 | 0 | 0 | 6 | 0 |
| expressions | 311 | 278 | 1 | 0 | 0 | 27 | 5 |
| foundations | 156 | 143 | 0 | 0 | 0 | 11 | 2 |
| statements | 88 | 77 | 0 | 0 | 0 | 10 | 1 |
| templates | 106 | 91 | 0 | 0 | 0 | 8 | 7 |

Changes by test, from the start of this part:

| Change | Tests |
|---|---|
| run unsupported, now passes | 95 |
| check unsupported, now passes | 29 |
| wrong result, now passes | 4 |
| missed undefined behavior, now passes | 4 |
| unexpected undefined behavior, now passes | 2 |
| missed reject, now passes | 1 |
| wrong result, now unsupported | 1 |
| false reject, now unsupported | 1 |

The wrong result that became unsupported is
`initializer-lists/list-ctor-preferred-over-better-match.cpp` (ILC-3).
The check chose the initializer-list constructor, and then an older path
built a call of another constructor with the same number of arguments.
The front end now stops there, because the interpreter does not model
the list object. The false reject that became unsupported is
`comparison/row-end-equals-next-row.cpp` (CMP-26), whose constant
evaluation stops at pointer arithmetic.

## 2. What the interpreter models now

### Bytes (item 1)

- **Object representation.** The value of each object stays the
  interpreter's record. A read of a byte computes the byte from the
  value of the scalar that holds it. A write of a byte gives that scalar
  the value of its new bytes. Integers are little-endian two's
  complement (STG-26, TYP-4), and `float` and `double` use their IEEE
  formats (MUL-20). Padding bytes, and the bytes of a scalar that has no
  value yet, are kept apart from the values, so a scalar written one
  byte at a time gets its value when its last byte arrives.
- **Byte pointers (STG-13).** A cast to `unsigned char*`, `char*`, or
  `std::byte*` gives a pointer to a byte of the complete object.
  Arithmetic, subtraction, and comparison use the bytes of the complete
  object as one array. A comparison of a byte pointer with another
  pointer compares their addresses.
- **Byte arrays that hold objects.** A read or write of an element of a
  byte array reaches the bytes of the object that placement `new` made
  there, if one covers the element (STG-4, STG-5). The old "bytes of
  storage that holds an object of another type" stop is gone.
- **Addresses (CAST-16).** The interpreter gives each object an address
  when a program first needs one, aligned for its type (ALN-4, TYP-2).
  A pointer converts to its address and back. An integer that is no
  object's address gives a pointer that holds only the address.
  Functions also get addresses. A pointer's bytes are its address.
- **`reinterpret_cast` (CAST-6, CAST-12, STG-16, CLS-19, STG-29).** A
  cast to another object type finds the pointer-interconvertible object
  of that type: the members of a union, and the first member and the
  bases of a standard-layout class, in a chain. Otherwise the result
  points to the same object. A `static_cast` through `void*` does the
  same. A reference cast acts on the place. A function pointer keeps its
  function through any pointer type. A cast of a pointer to member keeps
  its member. Constant evaluation still stops with CE-4.
- **Access through another type (STG-14, STG-23).** A read or write
  checks the type of the glvalue against the type of the object. The
  same type with any cv-qualifiers, the signed or unsigned counterpart,
  and the three byte types are allowed; the counterpart converts the
  value. Any other scalar type stops the run with STG-23, and so does a
  member access through an unrelated class. The four STG-23 tests stop
  with STG-23 (finding G95).
- **STG-8 and STG-28.** When storage from an allocation function first
  gives an array of one type and a later use needs another type, the
  interpreter makes the second set and carries the bytes that the
  program wrote. Storage cast to a class that is not an implicit-lifetime
  type gives an array whose elements begin their lifetimes when the
  program constructs them. A member call on an element that has not
  begun stops with STG-18.
- **STG-22.** A member call on storage whose type is not a class stops.
- **Byte copies.** `byte-round-trip.cpp`, `trivially-copyable-bytes.cpp`
  (CPY-19), and the STG-11 and STG-12 tests run. A copy of a byte with no
  value into `unsigned char` or `std::byte` keeps "no value" (INIT-1).

### Coverage and wrong results (item 4)

- **Allocation and deallocation functions (NEW-10, NEW-13, NEW-21 to
  NEW-24, NEW-31, NEW-32, NEW-35, NEW-36).** The check finds the
  allocation function of a `new` expression in the class or among the
  global functions the program declares, by overload resolution on the
  size and the placement arguments. It also finds the deallocation
  function that a throw from the initialization calls. The interpreter
  calls the function, makes the object in the storage it returns, and on
  a throw calls the deallocation function with the same placement
  arguments. A null result from a non-throwing function skips the
  initialization. A null result from another function stops with NEW-32.
  A `delete` expression calls the class's or the program's usual
  deallocation function with the storage. A call of `operator new` or
  `operator delete` by name chooses among the declared functions and the
  library's forms.
- **Library exceptions (LSUP-8).** A failed `dynamic_cast` of a
  reference throws `std::bad_cast`, an erroneous bound throws
  `std::bad_array_new_length`, and a failed allocation throws
  `std::bad_alloc`, when the program declares the class in `std`. The
  library's destructor ends the object and does nothing else.
- **Pointers to members (MPTR rules, CMP-22).** `&C::m` for data members
  and member functions, `.*` and `->*`, calls through a pointer to member
  function with virtual dispatch, conversions between base and derived
  classes, null values, `==` and `!=`, and conversion to `bool`. The
  value keeps the path of base subobjects, so the same member reached
  through two bases compares unequal. A null pointer to member, or an
  object without the member, stops with MPTR-6.
- **Contextual conversion to `bool` (CVF-3).** A class with an
  `explicit operator bool` converts in a condition.
- **The three missed stops.** DTOR-7: the end of a block after an
  explicit destructor call. VIRT-12: `delete` through a base whose
  destructor is not virtual. CV-5: a write to a `const` class object,
  except through a `mutable` member or while a destructor runs.
- **NEW-40.** `operator delete` on storage from `operator new[]`.
- **Wrong results fixed.** SINIT-1: a class object whose initializer is
  a constant expression is constant-initialized. SINIT-7: a static data
  member defined outside its class takes the place of its definition in
  the order. CMP-26: the end of one row equals the start of the next.
  LSUP-8: the `bad_cast` test. LOOP-3: the variable of a `for`
  condition ends after the step.
- **Union members and placement `new`.** An object made in a union
  member makes the member active (UNI-5). An explicit destructor call on
  a union member leaves no member active.

### Choice points and SEQ-4 (item 7)

- **Exploration.** Each choice point asks the run for an option. The
  first run takes option 0 everywhere. `tidal-fe run` then reruns the
  program with each other option, depth first, up to 64 runs. The
  options are SEQ-1 (left to right, or right to left), SEQ-8 (left or
  right operand first), PRIM-9 (one object or two, one choice per
  literal object), CMP-8 (either order, independently for each
  comparison), CMP-20 (equal or not, only when exactly one pointer is
  past the end), and MPTR-7. `run` prints the other outcomes on a second
  line, and says when the bound stopped the exploration. If another run
  stops at undefined behavior, that stop is the program's outcome.
- **SEQ-6.** The arguments of an overloaded shift, subscript, comma,
  `->*`, `&&`, or `||` keep the left-to-right order. Before the fix, the
  first version of the exploration offered SEQ-1 there, and three tests
  gave result 1 on the other order.
- **Survey.** 33 run tests pass a choice point: SEQ-8 18 times, SEQ-1 10
  times, PRIM-9 and CMP-8 twice each, CMP-20 and MPTR-7 once each. 29 of
  them need 2 runs, 3 need 4, and one needs 16. No test reaches the
  bound. A program of my own with eight choice points in a loop reached
  it. Every test except one gives the same result on every run. The
  exception is `comparison/pointer-unrelated-run.cpp` (finding G99).
- **SEQ-4.** While the operands of a built-in operator other than a
  shift run, the run records each read and write of an object in the
  caller's frame, by memory location. Adjacent bit-fields are one
  location. A write in one operand and a read or write of the same
  location in the other stops the run with SEQ-4. Accesses inside called
  functions do not count (SEQ-1). The check covers `i++ + i++`,
  `g(i++) + i`, and `(x.a = 1) + (x.b = 2)` with adjacent bit-fields. It
  does not stop two reads of one volatile object (finding G102).

## 3. What the interpreter does not model

### Item 1

- The bytes of a class with base classes or bit-fields, of a
  polymorphic class's hidden pointer, and of `long double`. Four run
  tests stop there: `base-bytes-shared-with-member.cpp` (STG-25),
  `bitfield-low-order-first.cpp` (BF-9), and two `new` tests whose
  storage holds a class with a base. Roadmap item 8 covers the missing
  layout tables.
- The objects that STG-8 creates at the start of a byte array's
  lifetime. Only storage from an allocation function creates them.
- Indirection through a pointer that holds only an address stops as
  unsupported (finding G94).
- The bound of a byte pointer is the complete object, also for a
  pointer to a member's first byte (finding G93).

### Item 4

- Unsupported run tests: 131. The largest groups:

  | Construct | Run tests |
  |---|---|
  | class layout the front end does not model (anonymous unions, inherited constructors, the `initializer_list` backing array) | 11 |
  | `new` with an over-aligned type (NEW-28, NEW-29) or forms the check does not resolve | 11 |
  | structured bindings with `get`, init captures | 8 |
  | virtual bases | 7 |
  | constructor calls the check does not build | 6 |
  | variables of an enclosing function in a local class or lambda | 5 |
  | the initializer-list constructor (ILC-3) | 4 |
  | overloaded `operator[]` with zero or several parameters, static `operator[]` | 5 |

- One wrong result remains: `floating-divide-by-zero-infinity.cpp`
  (MUL-11, MUL-20). The interpreter keeps `long double` as `double`
  (finding G98).
- `new` in constant evaluation outside `std::construct_at` still stops
  with NEW-37, and the over-aligned forms stay unsupported.

### Item 7

- SEQ-1 tries two orders of the arguments, not every permutation.
- The SEQ-4 check covers the operands of built-in binary operators. It
  does not cover the placement arguments and bound of `new` (NEW-18),
  the operands of an overloaded operator, or the volatile clause.
- The conformance harness uses the outcome of the explored runs, and
  does not yet list the tests whose runs give different results. A test
  whose runs differ appears only in the output of `tidal-fe run`.
- The precise SEQ-7 warning is not built.

## 4. Findings

### G91. The bytes of a pointer

- Page and rules: foundations/storage, STG-11, STG-15, STG-26;
  expressions/casts, CAST-16.
- Kind: missing.
- What happened: STG-26 fixes the byte order of an integer. STG-15 says
  a pointer represents an address. CAST-16 maps a pointer to an integer.
  No rule says what the bytes of a pointer object are. The interpreter
  needs an answer for a byte copy of a pointer, and for a byte copy of a
  class with a pointer member (STG-12, CPY-19). It uses the integer of
  CAST-16 in the order of STG-26.
- Suggestion: add to STG-26, as `impl-defined`, that on the matrix
  targets the object representation of a pointer is the integer that
  CAST-16 gives, in the same byte order as an integer.

### G92. Padding bytes, and a copy of a byte with no value into `char`

- Page and rules: foundations/storage, STG-11, STG-12;
  declarations/initialization/forms, INIT-1, Q-0039.
- Kind: ambiguous.
- What happened: STG-11 says a change to padding does not change the
  value. It does not say what a read of a padding byte gives, or whether
  a written padding byte keeps the value written. `byte-round-trip.cpp`
  copies the bytes of a class with padding through `unsigned char`. The
  run is defined only if a padding byte counts as a byte with no value
  that INIT-1 lets a program copy into `unsigned char`. STG-12 also lets
  a program copy the bytes into an array of `char`, but INIT-1's
  exception names only `unsigned char` and `std::byte`. So a byte copy
  of a padded object into `char` reads a byte with no value through
  `char`, which INIT-1 makes undefined. The interpreter treats padding
  as bytes with no value and keeps written padding bytes. It follows
  INIT-1 for `char` and stops.
- Suggestion: say that a padding byte has no value until the program
  writes it, and either add `char` to INIT-1's exception or limit STG-12
  to `unsigned char` and `std::byte` for objects with padding.

### G93. How far a byte pointer to a member can move

- Page and rules: foundations/storage, STG-13, Q-0304.
- Kind: ambiguous.
- What happened: STG-13 lets a byte pointer to the first byte of an
  object move over the bytes of that object. `padding-bytes-not-in-value.cpp`
  compares a byte pointer that walks the whole class object with byte
  pointers made from `&b.tag` and `&b.count`. Comparisons between
  pointers into different objects are not ordered by CMP-7 unless the
  bytes of the enclosing object count as one array for all of them. The
  text does not say whether a byte pointer made from a member's address
  can move past the member into the rest of the object. The interpreter
  treats every byte pointer as a pointer into the bytes of the complete
  object.
- Suggestion: say that a byte pointer made from the address of a
  subobject points into the bytes of the complete object, so it can move
  and compare over all of them.

### G94. A pointer made from an integer that is no object's address

- Page and rules: foundations/storage, STG-15; expressions/casts,
  CAST-16.
- Kind: missing.
- What happened: `integer-to-pointer-address.cpp` converts the integer
  `0x40001000` to a pointer and back. STG-15 lists four kinds of pointer
  value. The result is none of them: it is not the address of an
  object, not past one, not null, and not invalid, because no storage
  ended. The text does not say what indirection through it does. The
  interpreter keeps the address and stops as unsupported on indirection.
- Suggestion: add a fifth kind, or say that the result of CAST-16 for an
  integer that is no object's address is an invalid pointer value, with
  the round trip of CAST-16 kept.

### G95. The STG-23 tests cannot check the stop

- Page and rules: foundations/storage, STG-23; decision 0053.
- Kind: missing (test metadata).
- What happened: `access-unrelated-scalar.cpp`, `access-unrelated-class.cpp`,
  `access-signed-char.cpp`, and `byte-array-cast-reaches-byte.cpp`
  expect `accept`, because no sanitizer in the matrix stops them. The
  interpreter now stops each one with STG-23. An `accept` test does not
  run, so the harness does not check the stop.
- Suggestion: add `// tidal-interpreter: run-fail` to the four tests
  (decision 0053). `frontend_check.py` then counts the stop.

### G96. PFX-19 makes a call through a converted `noexcept` pointer undefined

- Page and rules: expressions/postfix, PFX-19; foundations/conversions,
  CONV-10; Q-0178.
- Kind: contradictory.
- What happened: CONV-10 lets a pointer to a `noexcept` function convert
  implicitly to a pointer to a function without `noexcept`. PFX-19 makes
  a call through a pointer whose function type is not the function's
  type undefined, and says that a difference only in `noexcept` counts.
  So `int (*p)() = g; p();` with `int g() noexcept` is undefined by the
  text. Q-0178 discusses the opposite direction, a cast that adds
  `noexcept`. I ran that program with g++ and clang++ (the latter with
  `-fsanitize=function`); both return normally. The interpreter follows
  PFX-19 and stops.
- Suggestion: limit the `noexcept` sentence of PFX-19 to a pointer type
  with `noexcept` and a function without it, so that the conversion of
  CONV-10 keeps calls defined.

### G97. When an allocation function fails

- Page and rules: expressions/new-delete, NEW-38, NEW-35;
  foundations/language-support, LSUP-8.
- Kind: missing.
- What happened: NEW-38 says a global allocation function that cannot
  allocate the storage throws `std::bad_alloc`.
  `exception-classes-allocation.cpp` expects requests for half and a
  quarter of the range of `size_t` to fail. No rule says which requests
  fail, so an implementation that allocates lazily could follow the text
  and give the storage. The interpreter fails every request above 2^40
  bytes.
- Suggestion: an `impl-defined` rule that on the matrix targets a
  request larger than the address space the target can map fails, with
  the sizes the test uses as the Evidence.

### G98. The format of `long double`

- Page and rules: expressions/multiplicative, MUL-20, MUL-11.
- Kind: missing.
- What happened: MUL-20 says `float`, `double`, and `long double`
  follow IEEE 754 on the matrix targets. It does not name the format of
  `long double`. `floating-divide-by-zero-infinity.cpp` needs a finite
  `long double` above `1.1e4932`, which `double` does not have. The
  interpreter keeps `long double` as `double`, so the run gives 4.
- Suggestion: name the format of each floating type on the matrix
  targets (binary32, binary64, and the x87 80-bit extended format in 16
  bytes on x86-64), with the size from the TYP table.

### G99. CMP-8 and `pointer-unrelated-run.cpp`

- Page and rules: expressions/comparison, CMP-8.
- Kind: contradictory (test against rule).
- What happened: CMP-8 says the results of `a < b` and `a > b` for
  unrelated objects need not agree. `pointer-unrelated-run.cpp` expects
  `lt != gt`, so it needs the two results to disagree. The exploration
  tries each choice for each comparison and finds runs where both are
  true or both false. Those runs give result 1.
- Suggestion: make the test accept either result, as
  `unrelated-pointers-either.cpp` does, or make CMP-8 say that one
  evaluation of `a < b` and `a > b` on the same values gives opposite
  results.

### G100. SEQ-6 and a call written as a function call

- Page and rules: foundations/evaluation-order, SEQ-6, SEQ-1;
  classes/operators, OPR-7.
- Kind: ambiguous.
- What happened: SEQ-6 says an overloaded operator keeps the order of
  the built-in operator for shifts, assignments, subscripts, the comma,
  and the pointer-to-member operators. It does not say whether the rule
  applies to a call written as `operator,(a, b)`, which is a function
  call under SEQ-1. The interpreter cannot tell the two forms apart, and
  keeps the order for both.
- Suggestion: say that SEQ-6 applies to the operator notation, and that
  an explicit call of the operator function follows SEQ-1.

### G101. STG-8 and pointers from earlier uses

- Page and rules: foundations/storage, STG-8, STG-24.
- Kind: ambiguous.
- What happened: STG-8 creates the set of objects that gives the
  program defined behavior, and that set can depend on a later use. In
  `implicit-object-first-use-member.cpp` the first use is an `int*`, and
  only a later use shows that the storage holds a `Pair`. An
  implementation that runs forward has already made an `int` object. The
  text does not say that the earlier `int*` then points to the member
  `first` of the `Pair`. The interpreter makes the second set and copies
  the bytes, and the earlier pointer keeps the first set's object, which
  has ended. A later use of that pointer would stop with OBJ-2.
- Suggestion: say in STG-8 that each pointer from an earlier use points
  to the matching object of the chosen set, so the choice does not
  depend on the order of the uses.

### G102. SEQ-4's volatile clause and seven run tests

- Page and rules: foundations/evaluation-order, SEQ-4.
- Kind: contradictory (tests against rule).
- What happened: SEQ-4 makes two unsequenced reads of the same volatile
  object undefined. These run tests read one volatile object in both
  operands of one operator:
  - `expressions/comparison/nan.cpp`, `nan-float.cpp`, and
    `three-way-floating.cpp` (`zero / zero`),
  - `expressions/conversions/nan-to-bool.cpp` and
    `expressions/multiplicative/floating-zero-by-zero-nan.cpp`
    (`zero / zero`),
  - `expressions/multiplicative/product-value-run.cpp`
    (`quarter * quarter`),
  - `expressions/multiplicative/signed-overflow-multiply-sanitizer.cpp`
    (`half * (half + argc)`).

  With the clause enforced, each run stops at SEQ-4. The interpreter
  does not enforce the clause, so these tests keep passing.
- Suggestion: rewrite the tests with two volatile objects or with a
  local copy (`double z = zero; z / z`), and then enforce the clause.
