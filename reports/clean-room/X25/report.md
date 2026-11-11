# X25: P27, an in-object array whose elements start their lives one at a time

Date: 2026-11-11. Commit read: `13e3f3c0`. Version: C++23 only. Kind: Propose (decision 0063, item E). Role: a clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone. P27 is in `reports/clean-room/roadmap-7/portfolio.md`. The rules below are this report's own.

The reader returned this report as a message, and the integrator saved it here with the reader's files.

P27 lets a class keep `N` objects of `T` inside itself as one array of `T`, and start each element's life when it adds the element. The proposal uses a form that Tidal already has, an array member `T items[N]` of a union. Today the union form fails in constant evaluation. The construction of one element of an array member that is not active does not make the member active (UNI-14, UNI-6, Q-0425). P27 makes that construction activate the array member, with only the built element in its lifetime. The other elements exist and wait for their own construction, as the elements of an allocated array do (STG-28).

The portfolio entry says that P27 does not change existing behavior. The design below changes it in three narrow ways (section 4). One test changes from `reject` to `accept`, a test of validity can see the change, and one run-time case becomes undefined. No program under `tests/programs/` changes. The report fills sections 4.3 and 4.4 in full.

Open questions that bear on P27:
- **Q-0425** (UNI-14, UNI-6, CE-6) is open. P27 answers its array form and leaves its member-union form as it is.
- **Q-0430** and G164 dispute the typed step through a byte array (STG-28, STG-29, ADD-14). P27 gives in-object storage a form that needs no such step. The dispute stays for byte arrays (section 3).
- **Q-0421** (LSUP-11, CE-9) rejects an assignment to an element of allocated storage before its construction. P27 takes the same answer for the new form.
- **Q-0117** (CE-21) keeps a union with no active member as a valid constant. P27 extends that answer to an element that is not in its lifetime (CE-21 amendment).

Inputs read:
- AGENTS.md, CLAUDE.md, `documentation/exercises.md` sections 4 and 9 (Propose), and decision 0063 (item E).
- The output of `python3 -I tools/propose_check.py` and `python3 -I tools/propose_check.py --corpus`, run before any earlier Propose report was read.
- `reports/clean-room/X23/report.md` as the model for this report's form. X23 has no refresh note beside it. X25 relies on no claim of X23. Also the P27 entry of `reports/clean-room/roadmap-7/portfolio.md`.
- In full: `specification/foundations/storage.md`, `specification/classes/unions.md`, and `specification/foundations/language-support.md`.
- `specification/index.md`, the start of `specification/foundations/translation.md` (TR-P1 and its class table), and the glossary entries "lifetime" and "implicit-lifetime type".
- In `specification/expressions/additive.md`: ADD-P1 to ADD-P6, ADD-6, ADD-7, ADD-10, and ADD-14.
- In `specification/foundations/constant-evaluation.md`: CE-P2, CE-P3, CE-6, CE-9, CE-21, CE-22, and the Divergences on Q-0115 and Q-0117.
- In `specification/foundations/objects.md`: OBJ-P1, OBJ-1, OBJ-2, OBJ-6, and OBJ-10. Also DTOR-6 and NEW-37.
- The rule tests that section 4 names, and the program files of the scans' hits.

Compiler runs: small programs built with g++ 16.2.1 and clang++ 22.1.8 (`-std=c++23 -pedantic-errors`). The probes are in `reports/clean-room/X25/p/`, with `construct.h`, which declares `std::construct_at` and the placement forms as LSUP-10 and LSUP-7 give them. The scans are `scan_unions.py` and `scan_targets.py` in `reports/clean-room/X25/`, with their output in `scan_unions.out` and `scan_targets.out`. No compiler implements P27. GCC 16 accepts the construction that P27 makes valid, and Clang rejects it, so the probes show the split of Q-0425. EDG was not run. The harness did not let this subagent save `report.md`, so the integrator saves the text. The change table of section 2 is also in `/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/X25-check/table-for-check.md`. `python3 -I tools/propose_check.py --strict` reads 31 rows there with 0 conflicts and 0 warnings.

Totals: 31 rule changes (3 new, 10 amended, 18 unchanged but must state their effect). Two of the amended rows are Examples segments and one is a glossary entry. 1 existing test changes its expected outcome, from `reject` to `accept`. No program under `tests/programs/` changes, out of 147. 9 findings, X25-1 to X25-9.

## 1. Design

### 1.1 The feature

The storage-form table beside the `FixedVec` example (`foundations/storage.md`, Examples) lists five forms for a sequence of `T`. No form inside the object gives constant evaluation, one array of `T` for pointer arithmetic, and a `T` with no default constructor together. The row closest to the goal is a union with an array member. It fails in constant evaluation, because the first `std::construct_at` on one element does not make the member active (UNI-14):

```cpp
template <class T, int N>
class FixedVec {
union { T items[N]; };                 // no active member at first (UNI-4)
int count = 0;
public:
constexpr FixedVec() {}
constexpr ~FixedVec() { while (count > 0) items[--count].~T(); }
FixedVec(const FixedVec&) = delete;     // the class writes its copy (UNI-3)
FixedVec& operator=(const FixedVec&) = delete;
template <class... Args>
constexpr T& emplace(Args&&... args) {  // the caller checks the capacity
  T* p = std::construct_at(items + count, static_cast<Args&&>(args)...);
  ++count;
  return *p;
}
constexpr T* data() { return items; }   // a pointer and a length
constexpr int size() const { return count; }
};
```

Today Clang rejects the first `emplace` in a constant evaluation, and Tidal's rules agree with Clang (UNI-6). GCC 16 accepts it (probe `a1.cpp`). At run time both compilers run the class with no sanitizer report, and UNI-14 says that the construction is valid there. UNI-14 does not say whether `items[i]` through the member name then reaches the element (X25-4).

Under P27 the class is valid in both settings. `items + count` points to an element of one array of `T`, so `data()` and `size()` give a view over the built elements. A sort over that view works on the elements themselves (probe `m1.cpp`). Today the program `W20/span_sort.cpp` sorts an array of pointers instead, because each row of its byte storage is a separate object (STG-P2, ADD-10).

P27 adds no syntax. The class above is valid C++23 text today, and only the rules on lifetime change.

### 1.2 Activation by one element (new rule X25:new-1; amended UNI-14, UNI-P1, CE-6, UNI-6)

X25:new-1 covers a union `U` with a member `m` of type "array of `N` `E`". Take a construction that creates an object of type `E`, ignoring cv-qualifiers, at the address of element `k` of `m`. The construction is a placement `new` with a form of LSUP-7, or `std::construct_at` (LSUP-10). Assume `m` is not active when the construction completes. Then these things happen:
- `m` becomes the active member of `U`. Any other member that was active ends, also when its bytes do not overlap element `k`. A union has at most one active member (UNI-P1).
- The lifetime of the array `m` begins.
- Element `k` begins its lifetime with the new object. The new object is element `k` (STG-9).
- Each other element of `m` exists and is not in its lifetime.

The rule holds at run time and during constant evaluation. It holds through each level of an array of arrays. A construction of an `E` at `cells[1][0]` of a member `E cells[2][2]` makes `cells` active, begins the lifetime of the row `cells[1]`, and starts only `cells[1][0]`. The rows `cells[0]` and `cells[1]` are arrays, and an array has no constructor to wait for.

The member becomes active when the construction completes, because the new object's lifetime begins then (OBJ-1). If the constructor throws, the union keeps its state, and no element begins.

X25:new-1 does not cover an object of another type at an element's address. That case stays with STG-6, as today. It does not cover a member of a member union either, the other form of Q-0425. That form keeps today's answer (section 1.6).

UNI-14 loses the array case from its constant-evaluation sentence. The sentence keeps "a member of a member union" and cites X25:new-1 for an element of an array member. The run-time sentence adds that the new element is reachable through the member name, since the member is now active. UNI-P1's sentence on constant evaluation changes in the same way. CE-6 adds X25:new-1 to the ways a member becomes active. UNI-6's last sentence narrows to a member of a member union.

### 1.3 The elements of an active array member (new rule X25:new-2)

X25:new-2 covers an array member that X25:new-1 made active. While the member stays active, these rules apply to its elements:
- An element begins its lifetime only by a construction of an object of the element type at its address, as in X25:new-1.
- An explicit destructor call on an element ends that element (DTOR-6). The array member stays active, also when no element is left in its lifetime. UNI-14's sentence on an explicit destructor call ends the active member only for a call on the member itself.
- A new construction at an element that has ended becomes that element (STG-9). So `items[k]` and every pointer to element `k` reach it.
- An assignment does not start an element. UNI-13 activates only a member that is not active, so an assignment to an element that is not in its lifetime writes outside a lifetime (OBJ-2, STG-22). The answer matches Q-0421 for storage from `std::allocator`, and it holds for every element type, also `int`.

The program steps pointers over all `N` elements, compares them and subtracts them, whatever their lifetimes (STG-10, ADD-6, ADD-7). A read, a write or a member call on an element that is not in its lifetime is undefined (OBJ-2, STG-22). Constant evaluation rejects it (CE-9). GCC 16 already gives these answers in constant evaluation (probes `a2.cpp` case 1, `a5.cpp` case 1). It accepts the assignment of `a2.cpp` case 2, which P27 rejects.

Nothing ends the array member except another member becoming active, a reuse of the union's storage (STG-6), or the end of the union. The union runs no destructor of a variant member (UNI-P2), so the class destroys each element it built, as `~FixedVec` does.

If the union is active through UNI-13, every element is in its lifetime, as today. An assignment such as `u.items[0] = 1` to a member with a trivial default constructor still starts the whole array. Only the activation by construction starts one element.

### 1.4 Copies and constants (new rule X25:new-3; amended CE-21)

A union whose element type has a non-trivial copy function has deleted copy functions (UNI-3). The class then writes its own copy, which constructs each built element in the target. For an element type with trivial copy functions, the union's trivial copy remains, and P27 must say what it carries.

X25:new-3 says that a trivial copy or move of a union whose active member is an array under X25:new-2 makes the same member active in the target. The target's elements in their lifetime are those of the source, with their values. The copy reads no other element. At run time the copy moves the bytes, as any trivial copy does. Today both compilers accept a trivial copy of a union whose active array member has an element with no value (probe `a3.cpp`, X25-5).

CE-21 says that a constant's value gives a value to each scalar part, and that each element of an array member is a part. P27 amends it. In an array member of a union, an element that is not in its lifetime is not a part, as a member that is not active is not a part (Q-0117). So a `constexpr` variable can hold a `FixedVec` with fewer than `N` elements. Today both compilers reject such a constant (probes `a7.cpp` case 1, `a10.cpp -DEND`). The amendment covers only array members of unions. A plain class with an array member still runs its elements' destructors, so an element outside its lifetime there has other faults.

### 1.5 Constant evaluation (amended LSUP-10)

LSUP-10 lets the placement `new` in `std::construct_at` create an object in two kinds of place: storage from `std::allocator` in the same evaluation, and an object whose lifetime began in the same evaluation. An element of an array member that is not active fits neither kind, and neither does the member. UNI-14 already accepts a construction at a member that is not active (X25-1). The amendment adds a third kind of place, a member of a union whose lifetime began in the same evaluation, or an element of an array member of such a union, at any depth (X25:new-1).

NEW-37 does not change. A placement `new` outside `std::construct_at` is not valid in constant evaluation, so `FixedVec` uses `std::construct_at`. The evaluation can change only its own objects (CE-P2), so a `FixedVec` declared outside the evaluation gains nothing.

### 1.6 Alternatives considered

- **A new declaration form.** A marker on a member such as `T items[N]` would say that its elements start one at a time. The form would have to settle again what the union form already settles: no automatic destruction (UNI-P2), deleted copy functions (UNI-3), and the default state. Q-0412 keeps attributes to placement rules, warnings, promises and layout, so an attribute marker cannot change lifetimes. A keyword is new syntax. Rejected.
- **Extend X25:new-1 to a member of a member union.** A construction of a member of a member union would activate the outer member. The outer member is a union, so the rule would have to say which of its members becomes active and what happens to the outer member's other parts. Q-0425 keeps both forms together. P27 needs only the array form. Deferred.
- **Let an assignment start an element of implicit-lifetime type.** `items[1] = 5` would start element 1 when the element type is `int`. The rule would split on the element type and differ from Q-0421's answer for allocated storage. Rejected.
- **Start every element of implicit-lifetime type at activation.** For `int items[N]`, the activation would start every element with no value, as UNI-13 does. The class could then read an element it never built and get no diagnostic in a run. The rule would also differ between `int` and a class type. Rejected.
- **Keep CE-21 as it is.** A `FixedVec` would serve transient constant evaluation only, and a `constexpr` variable of the class would need all `N` elements built. Both compilers already reject such a variable, so the alternative costs no compiler change. The amendment is the smaller part of P27 and can be dropped alone.
- **A library type.** Tidal gives only the shapes of library names that core rules use (decision 0041). A fixed-capacity container is a library type and has no core rule to attach to. Not a Tidal proposal.

### 1.7 Migration

A class with byte rows, such as `FixedVec` in `foundations/storage.md`, drops its rows and its array of pointers. It declares an anonymous union with `T items[N]`, a user-provided constructor and destructor, and builds each element with `std::construct_at`. The union is aligned for `T` (UNI-1), so the `alignas` goes too. A class with one union slot for each element, such as `FixedStack` (`classes/unions.md`), moves the array inside the union and gains a pointer and a length over its elements.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests affected" lists the existing tests whose expected outcome changes. New rules use this exercise's ID space.

| Rule ID | Kind of change | Old class | New class | Description | Tests affected |
|---|---|---|---|---|---|
| X25:new-1 | new | none | required | A construction of an object of the element type at an element of an array member that is not active makes the member active, ends any other active member, begins the array's lifetime and that element's, and leaves the other elements outside their lifetimes. Run time and constant evaluation. Applies at each level of an array of arrays (1.2). | `classes/unions/construct-at-array-member-element-constant.cpp` (reject to accept) |
| X25:new-2 | new | none | required | In an array member that X25:new-1 made active, an element begins only by construction, ends by an explicit destructor call, and the member stays active. An assignment starts no element (1.3). | none |
| X25:new-3 | new | none | required | A trivial copy or move of such a union carries the active array member and exactly the source's elements in their lifetime, and reads no other element (1.4). | none |
| UNI-14 | amended | required | required | The constant-evaluation sentence keeps only a member of a member union and cites X25:new-1 for array elements. At run time the new element is reachable through the member name. | `classes/unions/construct-at-array-member-element-constant.cpp` |
| UNI-6 | amended | undefined | undefined | The last sentence covers only a construction inside a member of a member union. | `classes/unions/construct-at-array-member-element-constant.cpp` |
| UNI-P1 | amended | principle | principle | The sentence on constant evaluation drops "one element of an array member" and adds that such a construction makes the array member active with one element (X25:new-1). | none |
| CE-6 | amended | reject | reject | Adds the construction of X25:new-1 to the ways a member becomes active. | none |
| CE-21 | amended | reject | reject | In an array member of a union, an element that is not in its lifetime is not a part of the value (1.4). | none |
| LSUP-10 | amended | required | required | Adds a third place: a member of a union whose lifetime began in the same evaluation, or an element of an array member of such a union (1.5). | none |
| OBJ-1 | amended | required | required | Adds a row for an array: its lifetime begins with its storage or by X25:new-1, and its elements can begin later (STG-28, X25:new-2). | none |
| STG-P2 | amended | principle | principle | The last paragraph adds that a class can keep one array of `T` in a union member and build its elements one at a time (X25:new-1). | none |
| Examples (storage.md) | amended | example | example | The storage-form table gains the row "a union with an array member, built one element at a time": inside the object yes, constant evaluation yes, one array yes, default constructor no. The sentence that no form gives all three goes, and `FixedVec` gains the version of 1.1. | none |
| Examples (unions.md) | amended | example | example | After `FixedStack`, a version with `T items[Capacity]` inside one union, with `data()` and `size()`. | none |
| glossary: lifetime | amended | glossary | glossary | The entry follows OBJ-1 for the end of a class object's lifetime (X25-3) and adds that an array's elements can begin after the array (STG-28, X25:new-2). | none |
| UNI-13 | unchanged, state effect | required | required | Activation by assignment still starts every element. An assignment to an element of an array member that is already active starts nothing (X25:new-2). | none |
| UNI-3 | unchanged, state effect | required | required | The union's special members stay deleted for a non-trivial element type, and the class writes them. | none |
| UNI-11 | unchanged, state effect | reject | reject | A use of a deleted special member of the union stays not valid. | none |
| STG-6 | unchanged, state effect | required | required | Covers a new object of another type at an element's address, which X25:new-1 does not cover. | none |
| STG-9 | unchanged, state effect | required | required | A construction at an ended element becomes that element, since the array is in its lifetime. | none |
| STG-10 | unchanged, state effect | required | required | Pointers over all `N` elements stay valid before activation and for elements outside their lifetimes. | none |
| STG-22 | unchanged, state effect | undefined | undefined | Covers a member call on an element that is not in its lifetime. | none |
| STG-28 | unchanged, state effect | required | required | The allocated array is the model: elements exist before their lifetimes. P27 gives the same model inside an object. | none |
| STG-29 | unchanged, state effect | required | required | A cast of a byte array's address still reaches a byte. P27 offers in-object storage that needs no such cast. | none |
| STG-P3 | unchanged, state effect | principle | principle | The growable array of the principle stays the form for storage outside the object. | none |
| ADD-10 | unchanged, state effect | undefined | undefined | The elements of the union's array member form one array, so stepping from element 0 to element 1 stays inside it. | none |
| ADD-14 | unchanged, state effect | undefined | undefined | A `T*` to an element has the element type, so ADD-14 does not apply and Q-0430 does not arise. | none |
| LSUP-11 | unchanged, state effect | required | required | `std::allocator` stays the form for storage outside the object in constant evaluation. | none |
| NEW-37 | unchanged, state effect | reject | reject | A placement `new` outside `std::construct_at` stays not valid in constant evaluation. | none |
| OBJ-2 | unchanged, state effect | undefined | undefined | A read or a write of an element that is not in its lifetime, an assignment included, is undefined. | none |
| CE-9 | unchanged, state effect | reject | reject | Constant evaluation rejects the accesses of OBJ-2 and STG-22 on such an element. | none |
| DTOR-6 | unchanged, state effect | required | required | An explicit destructor call on an element ends the element, and the array member stays active (X25:new-2). | none |

Counts: 31 changes. 3 new, 10 amended, 18 unchanged but must state their effect, 0 withdrawn, 0 split. Two amended rows are Examples segments and one is a glossary entry. 1 existing test changes its outcome, from `reject` to `accept`.

The changed test keeps its program. Its rules become UNI-14 and X25:new-1, with `tidal-expect: accept`. Its overrides for GCC 16, host GCC and EDG go, and Clang and GCC 14 move to the split table. The proposal needs these new tests under `tests/classes/unions/`:
- `array-member-element-activates-constant.cpp` (accept; probe `a1.cpp`, a `FixedVec` of a class with no default constructor),
- `array-member-element-activates-run.cpp` (run; `a1.cpp -DRUN`, with the reads through `items`),
- `array-member-element-ends-other-member-constant.cpp` (reject; probe `a4.cpp` case 1),
- `array-member-unbuilt-element-read-constant.cpp` (reject; `a2.cpp` case 1),
- `array-member-unbuilt-element-assign-constant.cpp` (reject; `a2.cpp` case 2),
- `array-member-element-rebuilt-constant.cpp` (accept; `a5.cpp` case 2) and `array-member-element-read-after-end-constant.cpp` (reject; case 1),
- `array-member-two-dimensions-constant.cpp` (accept; `a6.cpp`),
- `array-member-partial-constant-variable.cpp` (accept; `a7.cpp` case 1, CE-21),
- `array-member-element-trivial-copy-constant.cpp` (accept; `a2.cpp` case 4, X25:new-3),
- `array-member-element-view-sort-constant.cpp` (accept; `m1.cpp`),
- `array-member-element-validity-test.cpp` (run; `a9.cpp`, which returns 1 under P27).

## 3. Interactions

**Q-0425 (UNI-14, UNI-6, CE-6).** The question has two forms. P27 answers the array form, and the answer is the one GCC 16 and EDG give today. The member-union form keeps the current answer, which Clang and GCC give. `construct-in-member-of-inactive-member-union-constant.cpp` keeps `reject`.

**Q-0430 and G164 (STG-28, STG-29, ADD-14).** The reviews of phase 14 dispute whether a `T*` cast from a byte can step to the next position. `FixedVec` with byte rows does not need that step, but it cannot give a view over its elements. A class that wants both turns to the step or to storage outside the object. Under P27 a class gets one array of `T` inside the object, with pointers of type `T*` from the start, so the disputed step is no longer needed for that purpose. The question stays for byte arrays.

**Q-0421 (LSUP-11, CE-9).** Assignment before construction is rejected for allocated storage. X25:new-2 gives the same answer for the union array, for every element type.

**Q-0117 (CE-21).** A union with no active member is a valid constant. The CE-21 amendment applies the same idea to one element: no object, so no part. If Q-0117 is answered the other way, the amendment should go with it.

**Constant evaluation (CE-P2, LSUP-10, NEW-37).** The construction goes through `std::construct_at`, and the union must be an object of the evaluation. LSUP-10's amendment covers the place. CE-9 and OBJ-2 reject each access to an element outside its lifetime.

**Special members (UNI-3, UNI-11, CPY-8).** P27 changes no rule on special members. A class over a non-trivial `T` writes its destructor and copy functions, as `FixedStack` does today. X25:new-3 covers only the trivial copy that remains for a trivially copyable `T`.

**Alignment and size (UNI-1, ALN-P1).** The union has the alignment and at least the size of `T[N]`. The byte-row form needs `alignas(T)` for the same effect.

**Exceptions (OBJ-1).** A constructor that throws begins no element and activates no member (1.2). `emplace` then leaves `count` unchanged, because the increment comes after the construction.

**Not touched.** P27 leaves these alone:
- an array member of a class that is not a union,
- a member of a member union (Q-0425's other form),
- a construction of an object of another type at an element's address (STG-6),
- an element type with cv-qualifiers, which `std::construct_at` in the shape of LSUP-10 cannot take, because the cast to `void*` drops `const`,
- the launder operation and the library functions that start a lifetime without a constructor *(library referenced)*.

## 4. Impact on existing programs

`python3 -I tools/propose_check.py --corpus` counts 147 programs under `tests/programs/` (94 from the Write suites and 53 programmer sketches under `roadmap/`), 8 with more than one unit, and 19244 lines in 170 files, with extra units and headers.

Two lexical scans cover all 7107 `.cpp` files under `tests/` and the headers beside them:
- `scan_unions.py` lists each union, named or anonymous, with a member whose declarator has an array bound. For each one it says whether the file calls `std::construct_at` or a placement `new`, and whether it has a constant evaluation.
- `scan_targets.py` lists each `std::construct_at` and placement `new` whose first argument subscripts or adds, in a file that declares a union. The scan finds constructions at an element whatever the union's spelling, such as a union declared through an alias.

| Scan | Files | Hits |
|---|---|---|
| unions with an array member | 7107 | 19 unions in 16 files; 2 files construct anything |
| constructions at an element, in files with a union | 7107 | 9 lines in 8 files |

By hand:
- `classes/unions/construct-at-array-member-element-constant.cpp` is the one file that constructs at an element of an array member of a union. It changes (4.1).
- `programs/roadmap/r12-sso.cpp` and `programs/roadmap/r6-text.cpp` have a `char` array member in a union. They write it by assignment (UNI-13) and copy it element by element. Neither constructs an element. `r6-text.cpp` uses placement `new` on the whole `Str`, which X25:new-1 does not cover.
- The other hits of `scan_targets.py` construct a whole member at its address in a union slot: `classes/unions/slot-array-constant.cpp`, `classes/unions/placement-into-union-member-of-new-array-element.cpp`, and the programs `W15/stack.cpp`, `roadmap/r12-ring.cpp`, `roadmap/r12-ring-third-try.cpp`, `roadmap/r12-ring-fifth-try.cpp`, and `roadmap/r10-hash-map.cpp`. UNI-14 covers each of them today, and P27 changes none.
- The remaining 16 unions of `scan_unions.py`, in 13 files, are in rule tests of layout and of UNI-13 and UNI-14. None constructs an element. `chain-assign-array-element-nontrivial-constant.cpp` assigns to an element of a member whose default constructor is not trivial and keeps `reject` (UNI-13). `array-member-element-address-no-active-member.cpp` and `array-member-element-steps-no-active-member-constant.cpp` form and step pointers only, and keep their outcomes.
- The CE-21 amendment covers only unions. `constant-evaluation/result-unset-array-element.cpp` and `constant-evaluation/copy-unset-array-element.cpp` use classes that are not unions, and keep `reject`.

No program under `tests/programs/` changes, out of 147. Nine programs keep a sequence of `T` in the object in a form that P27 could replace:
- byte rows for each slot: `W20/static_vector.cpp`, `W20/span_sort.cpp`, `W16/textstats.cpp`, `roadmap/r6-scheduler.cpp`,
- one union slot for each element: `W15/stack.cpp`, `roadmap/r12-ring.cpp`, `roadmap/r12-ring-third-try.cpp`, `roadmap/r12-ring-fifth-try.cpp`, `roadmap/r10-hash-map.cpp`.

`W20/span_sort.cpp` notes in its header that it sorts pointers because its rows are separate objects (STG-P2, STG-29). These programs show the demand. Their meaning does not change.

### 4.1 Rejected becomes valid

- A constant evaluation that constructs an element of an array member that is not active. One test: `classes/unions/construct-at-array-member-element-constant.cpp`, from `reject` to `accept`. Its overrides say that GCC 16, host GCC and EDG already accept.
- A `constexpr` variable whose union array member has elements outside their lifetimes (CE-21 amendment). No existing test. Both compilers reject today (`a7.cpp` case 1, `a10.cpp -DEND`).
- An evaluation that copies such a union trivially (X25:new-3). No existing test.

### 4.2 Undefined becomes defined

At run time, a read of a built element through the member name, such as `items[i]` in `FixedVec`. Today UNI-14 names only the pointer that the construction gives, and the member is not active, so UNI-6 reads the access as a read of a member that is not active. The Evidence of `unions.md` gives two readings, and no test tells them apart (X25-4). Under P27 the read is defined. No existing test reads through the member name after such a construction.

### 4.3 Valid programs that change meaning

A valid program can change meaning in one way. It has no instance in `tests/`.

**A test of validity.** A `requires` expression, or a substitution in a function template's type, can test whether an expression is a constant. Take a template argument computed by a constant evaluation that builds an element of an array member that is not active. Under Tidal's rules today, the test gives `false`. Under P27 it gives `true`. Probe `a9.cpp` returns 0 on Clang and 1 on GCC 16 today, the split of Q-0425, and would return 1 on both. No file under `tests/` constructs an element of a union's array member inside such a test. The only construction at such an element is the one test of 4.1, which uses `static_assert`.

### 4.4 Valid programs that become invalid

One kind of run becomes undefined. Take a union with an active member `head` and an array member `items`. A program constructs element `k` of `items` at bytes that `head` does not occupy, then reads `head`. Today the read is defined under the reading that STG-6 gives, because the new object takes no byte of `head`. Under X25:new-1, `items` becomes active and `head` ends, so the read is undefined. Both compilers run probe `a4.cpp -DRUN` and read 3 from `head` with no sanitizer report. GCC 16 in constant evaluation already treats `items` as active and rejects the read (`a4.cpp` case 1). The scans find no instance in `tests/`. The one file that constructs at an element has no other member.

No test changes from an accepted outcome to a rejection. No program under `tests/programs/` becomes invalid, out of 147. The counts for the alternatives of 1.6:
- A new declaration form: no test changes, and the one test of 4.1 keeps `reject`.
- Keep CE-21 as it is: no change in the counts above, and the item of 4.1 on `constexpr` variables goes.

### 4.5 Evidence

Each probe uses `construct.h` unless it says otherwise.
- `a1.cpp`: `FixedVec<Num, 4>` over an anonymous union, `Num` with no default constructor. GCC accepts the constant, and Clang rejects it ("construction of subobject of member 'items' of union with no active member"). With `-DRUN` and the address and undefined sanitizers, both run it with status 0.
- `a2.cpp`: after `std::construct_at(&r.items[0], 1)` on an `int` array member. GCC rejects case 1 (a read of `items[1]`, "accessing uninitialized array element"), and accepts case 2 (an assignment to `items[1]`), case 3 (a read of `items[0]`) and case 4 (a trivial copy). Clang rejects all four at the construction.
- `a3.cpp` (no header): activation by `r.items[0] = 1` (UNI-13), then a trivial copy of the union. Both accept. `a3b.cpp`: the same copy of a class that is not a union. Clang rejects it ("subobject 'items' is not initialized"), and GCC accepts it (Q-0115). `a3c.cpp`: a read of `items[1]` in the copy of `a3.cpp`. Both reject.
- `a4.cpp`: `head` active, then a construction at `items[1]`. Case 1 reads `head`. GCC rejects ("accessing 'U::head' member instead of active 'U::items' member"), and Clang rejects at the construction. Case 2 reads `items[1]`. GCC accepts, and Clang rejects. With `-DRUN`, both read 7 from `items[1]` and 3 from `head`, with no sanitizer report.
- `a5.cpp`: two elements built, element 0 ended and built again. GCC rejects case 1 (a read after the end) and accepts case 2. Clang rejects both at the construction.
- `a6.cpp`: a construction at `cells[1][0]` of a member `int cells[2][2]`. GCC accepts, and Clang rejects.
- `a7.cpp`: a `Vec` with 2 of 4 elements built. Case 1, a `constexpr` variable: both reject, GCC on the incomplete value and Clang at the construction. Case 2, transient use: GCC accepts, and Clang rejects.
- `a9.cpp`: a `requires` expression over a constant template argument from `build<int>()`. Status 0 on Clang and 1 on GCC.
- `a10.cpp` (no header): activation by two assignments, then the pseudo-destructor on `items[1]` through an alias, then a `constexpr` variable. Without `-DEND` both accept. With `-DEND` both reject ("subobject 'items' is not initialized" on Clang).
- `m1.cpp`: `FixedVec<Word, 8>` with `data()` and `size()`, and an insertion sort over the view. GCC accepts the constant, and Clang rejects it. With `-DRUN` and the sanitizers, both run it with status 0.
- `scan_unions.out` and `scan_targets.out` hold the scans' output.

## 5. Findings

### X25-1

Kind: wrong. Pages: `foundations/language-support.md`, `classes/unions.md`. Rules: LSUP-10, UNI-14, NEW-37.

What happened: LSUP-10 lets the placement `new` in `std::construct_at` create an object during constant evaluation only at storage from `std::allocator` or at an object whose lifetime began in the same evaluation. UNI-14 accepts `std::construct_at` at a union member that is not active, and its Evidence note says that such a member has not begun its lifetime. The member fits neither place of LSUP-10, so the two rules disagree on the `FixedStack` example and on `slot-array-constant.cpp`. The Evidence says that the compilers read the lifetime of the union as enough, and UNI-14 follows them. LSUP-10 does not say so.

Suggestion: LSUP-10 adds a third place, a member of a union whose lifetime began in the same evaluation, and cites UNI-14.

### X25-2

Kind: missing. Pages: `foundations/objects.md`. Rules: OBJ-1, STG-28, STG-7.

What happened: OBJ-1's table gives when the lifetime of a class object and of a scalar object begins and ends. It has no row for an array. STG-28 says that the elements of an allocated array exist before their lifetimes begin, so the lifetime of an array and the lifetimes of its elements can differ. A designer cannot find when an array's own lifetime begins, which P27 needs.

Suggestion: OBJ-1 adds a row for an array. Its lifetime begins when its storage is obtained, or when an operation of STG-8 creates it, and each element begins as its own type says. The row cites STG-28 for elements that begin later.

### X25-3

Kind: wrong. Pages: `glossary.md`, `foundations/objects.md`. Rules: OBJ-1, "lifetime".

What happened: the glossary entry "lifetime" says that the lifetime of a class object ends when its destructor completes. OBJ-1 says that it ends when the destructor call starts. The entry also says that a scalar's lifetime starts with its storage, and OBJ-1 adds "and its initializer completes" for a scalar with an initializer.

Suggestion: the glossary entry follows OBJ-1 on both points.

### X25-4

Kind: unclear. Pages: `classes/unions.md`, `foundations/storage.md`. Rules: UNI-14, UNI-6, STG-6, STG-9.

What happened: at run time, UNI-14 says that a construction inside a member that is not active is valid, and that the program can use the new object through the pointer that the construction gives. The rule does not say whether the member name, as in `u.items[1]`, reaches the new object. It does not say whether another active member stays active when the new object takes none of its bytes. The Evidence gives two readings and says that no test tells them apart. Both compilers read 7 through the member name and 3 from the other member (`a4.cpp -DRUN`). In constant evaluation GCC 16 treats the array member as active (`a4.cpp` case 1).

Suggestion: UNI-14 states which reading Tidal takes for a run. If the member does not become active, the rule says that a read through the member name is a read of a member that is not active (UNI-6). Add a run test that reads through the member name, or a note that no test can observe the difference.

### X25-5

Kind: unclear. Pages: `foundations/constant-evaluation.md`, `classes/unions.md`. Rules: CE-22, CE-P3, OBJ-2, UNI-14.

What happened: CE-P3 says that copying an object reads each of its parts, and CE-22 says that an implicit copy reads each scalar part of its source. UNI-14 says that a trivial copy of a union makes the source's active member active in the target. No rule says whether a union's trivial copy reads the parts of its active member. Both compilers accept a trivial copy of a union whose active array member has an element with no value, and the target's element then has no value (`a3.cpp`, `a3c.cpp`). Clang rejects the same copy of a class that is not a union (`a3b.cpp`).

Suggestion: CE-22 states that a trivial copy or move of a union copies the active member's parts without reading them, so a part with no value stays without one in the target. Add `a3.cpp` and `a3c.cpp` as tests beside `copy-unset-array-element.cpp`.

### X25-6

Kind: hard to find. Pages: `foundations/storage.md`, `classes/unions.md`, `foundations/language-support.md`. Rules: STG-P2, UNI-14, LSUP-P3.

What happened: the storage-form table that states P27's gap is in the Examples segment of `storage.md`, after the `Maybe` and `FixedVec` examples. STG-P2 describes the byte-row and allocation forms and does not point to the table. The union-slot form is in the Examples of `unions.md`, and the allocator form is in LSUP-P3. A designer finds the table only by reading the Examples to the end.

Suggestion: STG-P2 cites the table, or the table moves into STG-P2 with a link from UNI-14 and LSUP-P3.

### X25-7

Kind: needs example. Pages: `foundations/constant-evaluation.md`. Rules: CE-21, DTOR-6, Q-0117.

What happened: CE-21 says that each element of an array member is a part and that only the active member of a union is a part. The rule does not say what happens when an element of an active member has ended by an explicit destructor call. Both compilers reject such a constant (`a10.cpp -DEND`), as CE-21 implies if an ended element counts as a part with no value. The rule's words "give a value to each of its scalar parts" read as covering only parts that exist.

Suggestion: CE-21 adds an example with an ended element, and says that an element outside its lifetime counts as a part with no value. Under P27 the sentence changes for array members of unions (1.4).

### X25-8

Kind: unclear. Pages: `reports/clean-room/roadmap-7/portfolio.md`, `documentation/exercises.md` section 4. Rules: none.

What happened: the portfolio entry for P27 says that P27 does not change existing behavior, because it adds a form. A design that adds a form inside the object must take a construct that is not valid today. The obvious construct is a union with an array member, which is valid text whose rules change. So the rating "no" holds only for a design with new syntax. The brief then asks for sections 4.3 and 4.4 "if the proposal changes existing behavior". A reader cannot tell from the portfolio which kind of design it expects. Section 4's table also stops at X23.

Suggestion: the portfolio entry says "no for a new form; yes, narrowly, if the union form gains the rule". Add the rows for X24 and X25 to the table of `documentation/exercises.md` section 4.

### X25-9

Kind: outside knowledge. Pages: none. Rules: none.

What happened: the reader knows from general experience that fixed-capacity vectors with in-object storage are common in practice, and that later versions of the language revisit the rules for unions. The design takes its rules only from Tidal, and the report uses no text or rule from such a version. A writer who checks P27 against later versions does so outside the clean room.

Suggestion: none for the specification.

## Host-only claims

Each claim below rests on g++ 16.2.1 and clang++ 22.1.8 on the host. A writer probes it on the matrix before acting.
- GCC 16 accepts, and Clang 22 rejects, a constant evaluation that constructs an element of an array member that is not active (`a1.cpp`, `a6.cpp`, `m1.cpp`, `a7.cpp` case 2).
- Both run `a1.cpp -DRUN` and `m1.cpp -DRUN` with the address and undefined sanitizers and no report.
- After such a construction, GCC 16 rejects a read of an element never built (`a2.cpp` case 1) and a read after an element's end (`a5.cpp` case 1), and accepts an assignment to an element never built (`a2.cpp` case 2).
- GCC 16 rejects a read of the formerly active member after a construction at an element of the array member (`a4.cpp` case 1).
- At run time, both read the new element through the member name and the old member's value, with no sanitizer report (`a4.cpp -DRUN`).
- Both accept a trivial copy of a union whose active array member has an element with no value (`a3.cpp`) and reject a read of that element in the copy (`a3c.cpp`). Clang rejects the same copy of a class that is not a union (`a3b.cpp`), and GCC accepts it.
- Both reject a `constexpr` variable whose array member has an element outside its lifetime (`a7.cpp` case 1, `a10.cpp -DEND`).
- `a9.cpp` returns 0 on Clang and 1 on GCC 16.
