# Q-0447: Does a part that an explicit destructor call ended count as a part with no value in the result of a constant expression?

**Status:** provisional
**Affects:** CE-21, OBJ-2, `tests/foundations/constant-evaluation/result-ended-element.cpp`, `tests/foundations/constant-evaluation/result-union-ended-element.cpp`, `tests/foundations/objects/write-after-pseudo-destructor-element-constant.cpp`, `tests/foundations/objects/write-after-pseudo-destructor-variable-constant.cpp`, `tests/foundations/objects/write-after-pseudo-destructor-union-array-element-constant.cpp`, `tests/foundations/objects/write-after-pseudo-destructor-union-member-constant.cpp`
**Source:** X25-7 (`reports/clean-room/X25/report.md`, probe `a10.cpp`), rerun on the matrix in `reports/clean-room/roadmap-13/writer-probes/x25-rules.json`, and by the item F6 writer with `tools/probe_grid.py`.
**Waits for:** a review (doubt); newest families that agree

## Context

A class or a union has an `int` array member. The constructor, or a
`constexpr` function that returns the object, ends one element with a
pseudo-destructor call. A `constexpr` variable then holds the object.

| Form | Clang 18 and 22 | GCC 14 | GCC 16 | EDG 7.0 |
|---|---|---|---|---|
| a struct, the element ended in the constructor | rejects | accepts | rejects | rejects |
| a union, the element ended in the constructor | rejects | accepts | rejects | rejects |
| a struct or a union, the element ended in a function that returns the object | rejects | accepts | rejects | rejects |
| a struct whose element of class type ends by its destructor call | rejects | accepts | rejects | rejects |
| a struct, the `int` element ended and then assigned 3 | rejects | accepts | accepts | accepts |
| a local `int` variable ended and then assigned 3 | rejects | rejects | rejects | accepts |
| a union's `int` array member, an element ended and then assigned 3 | rejects | accepts | accepts | accepts |
| a union's struct member, its `int` member ended and then assigned 3 | rejects | accepts | accepts | accepts |
| a union's `int` member, ended and then assigned 3 through its name | accepts | accepts | accepts | accepts |

In the last two rows, Clang rejects the assignment after the end. GCC
rejects it for a variable and reads it as a new value for an array
element, and EDG reads it as a new value in both.

## Evidence

The tests in **Affects** and the probes in **Source**.

## Options

1. A part whose lifetime ended has no value, so the constant is not
   valid (Clang, GCC 16, EDG).
2. The part keeps its last value (GCC 14).

## Provisional answer

The ended part has no value, and the constant is not valid (option 1).
CE-21 says so. Every newest compiler agrees, and GCC 14 alone differs.

Kept open on 2026-11-11 under decision 0054 (decision 0064, item RV9). The gap review of the phase 16 text supports the answer with a doubt (RV9-9), and every newest compiler agrees. The review also reads the form of the table's last row: an assignment to the `int` element after its end uses the element outside its lifetime, which is undefined, so the evaluation is not constant. OBJ-2 gives the same reading, as Clang does. GCC 16 and EDG 7.0 accept the form, so two newest families differ in that form.

Rule text on 2026-11-11 (decision 0064, item RV9): OBJ-2 lists an assignment to a scalar that an explicit destructor call ended, and says that the assignment creates no new object, as the RV9-9 reading gives. `write-after-pseudo-destructor-element-constant.cpp` expects the rejection, with GCC and EDG as overrides, and `write-after-pseudo-destructor-variable-constant.cpp` expects it with EDG as an override (`reports/clean-room/roadmap-13/writer-probes/rv9-points.json`).

Kept open on 2026-11-11 under decision 0054 (decision 0064, item RV9). The read of the RV9 repair diff agrees with the records of RV9-9 and gives no new reading of the answer, so the doubt of RV9-9 stays. The read adds a condition to OBJ-2's item (RV10-R-3). An assignment through the name of a scalar union member that a destructor call ended creates a new object of the member's type, so the assignment is valid, as UNI-5 and UNI-14 give. OBJ-2's item holds for each form of the table, and no test holds the union member form. GCC 16 and EDG 7.0 still differ in the table's fifth row.

Rule text on 2026-11-11 (decision 0065, item F7): OBJ-2's item gains the condition of RV10-R-3. An assignment through the name of a scalar union member that a destructor call ended makes the member active and is valid (UNI-5, UNI-14), and every compiler accepts the form (`write-after-pseudo-destructor-union-member-constant.cpp`). An element of a union's array member does not count, so OBJ-2's item holds for it. `write-after-pseudo-destructor-union-array-element-constant.cpp` expects the rejection, with GCC and EDG as overrides (`reports/clean-room/roadmap-14/writer-probes/rv10-points.json`). The table gains the three union rows. No test holds the row for a struct member of a union. The answer and the **Waits for:** line do not change.

Kept open on 2026-11-11 (decision 0065, item RV10). The gap review of the phase 17 text reads OBJ-2's sentence on the element of a union's array member with a doubt (RV10-12). A plain reading creates no object, so the write reaches an ended element, as Clang says. The C++23 text does not say whether such a write counts as a change to the array outside its lifetime. If it does, the assignment creates a new array with no element values, and the write is valid, which fits GCC 16 and EDG 7.0. OBJ-2 now also names a union member of class type that a destructor call ended, and every compiler accepts that form. The answer and the **Waits for:** line do not change.

Kept open on 2026-11-11 under decision 0054 (decision 0065, item RV10). The read of the RV10 repair diff agrees with OBJ-2's new sentence on the chain, with a doubt (RV11-R-5), and agrees with `write-after-destructor-union-class-member-constant.cpp` and `write-after-pseudo-destructor-nested-union-member-constant.cpp`. OBJ-2 and UNI-13 name a class member whose default constructor is trivial, and leave out that the constructor must not be deleted. A default constructor that is deleted at its first declaration can still be trivial, and no test holds that form. When a destructor call ends only a scalar inside a union member of class type, the class member stays alive. The C++23 text does not say whether a write to the ended scalar changes the class member outside its lifetime. OBJ-2 gives the plain reading, where the write creates no object. The read names that doubt as the doubt of RV10-12 for a class member, and the doubt on the array element stays. GCC 16 and EDG 7.0 still differ in the table's fifth row. The answer and the **Waits for:** line do not change.

## Opinions

### Gap review, 2026-11-11 (phase 16 text)

The source-reader gap review of the phase 16 text (`reports/review/2026-11-11-gap-review-phase-16.md`) reads the question: supports with a doubt (RV9-9). The value of a constant needs each scalar part to have a value. A pseudo-destructor call in an evaluation ends the scalar's lifetime, and an object's properties hold only during its lifetime, so the ended part has no value that the constant could accept (option 1). The doubt is that the reference reaches the result only through that general statement on lifetimes, and does not say outright what value an ended scalar has. An element of class type that a destructor call ended follows the same reading. The review reads the last row of the table as decided: the assignment after the element's end is a use of the object outside its lifetime, no operation in the row creates a new object, and the undefined use makes the evaluation not constant. Clang's rejection matches, and GCC 16 and EDG read against.

### Read of the RV9 repair diff, 2026-11-11

The source-reader read of the RV9 repair diff (`reports/review/2026-11-11-read-of-rv9-repair-diff.md`) reads the gap review's entry and the two dated notes of decision 0064, item RV9: agrees for the forms of the table, and adds a condition to OBJ-2 (RV10-R-3). The entry and the first dated note state RV9-9 correctly, and the table's row for a local variable agrees with the reference. The second dated note says that OBJ-2 gives the RV9-9 reading. That claim holds for the forms of the table, where the left side of the assignment names an element of a struct's array member or a local variable. OBJ-2's item goes further than RV9-9, which read only the struct element. The reference has an exception for an assignment whose left side names a union member by member access, also under array subscripts. If a write to such a member would be undefined because its lifetime has not begun or has ended, the assignment first creates a new object of the member's type at that place. So an assignment through the name of a scalar union member that a destructor call ended is valid, as UNI-5 and UNI-14 also give. For an element of a union's array member, the exception looks at the whole array member, which is still alive, so no object is created and OBJ-2's item holds. The read gives no new reading of the question's answer, and the doubt of RV9-9 stays.

### Read of the RV10 repair diff, 2026-11-11

The source-reader read of the RV10 repair diff (`reports/review/2026-11-11-read-of-rv10-repair-diff.md`) reads OBJ-2's new sentence on the chain, the OBJ Note, `write-after-destructor-union-class-member-constant.cpp`, `write-after-pseudo-destructor-nested-union-member-constant.cpp`, and the note of decision 0065, item RV10: agrees with a doubt (RV11-R-5). The reference's chain collects each union member whose type is a scalar, a class whose default constructor is trivial and not deleted, or an array of such elements. OBJ-2 and UNI-13 leave out the condition that the constructor is not deleted. A default constructor that is deleted at its first declaration can still be trivial, so OBJ-2 covers one class that the reference leaves out, and no test holds that form. Both new tests agree. In the first, the ended union member of class type is on the chain, so the assignment creates a new object and the read of the written scalar is valid. In the second, the ended scalar union member is on the chain, and the struct member above it adds nothing. When a destructor call ends only a scalar inside a union member of class type, the class member stays alive and stays on the chain. The reference does not say whether a write to the ended scalar changes the class member outside its lifetime. OBJ-2 gives the plain reading, where the write creates no object. The read calls that doubt the doubt of RV10-12 for a class member, and calls it small. The note above states the RV10-12 doubt correctly. The read gives no new reading of the question's answer, and the doubt on the array element stays.
