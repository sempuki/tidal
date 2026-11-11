# Gap review: phase 15 text on classes, declarations, foundations, statements, and the glossary (RV8, part A)

Date: 2026-11-11

Role: source reader

Reference: the C++23 working draft sources.

Commit read: e7e972d6b6e005bff03afcc42fded7c27b7cb9bb.

The review is item RV8, part A, of decision 0063. It covers three groups of text on the classes, declarations, foundations, and statements pages and in the glossary:

- the blocks that changed after RV7 read the tree: DTOR-8, UNI-P3, UNI-14, REF-2, LSUP-6, STG-P2, and LOOP-9. Of these, REF-2, UNI-P3, DTOR-8, and UNI-14 were repaired in both the RV6 and the RV7 repairs, and so was the glossary entry for a direct binding. The review reads those five as whole blocks, with their tests.
- the text that item F5 changed on these pages in commit 232ac618: OPR-8, OPR-10, OPR-16, OPR-P3, VIRT-13, the example under OBJ-9, STG-28, VC-P2, the example under RFOR-2, RFOR-P1, and the glossary entries for closure type and range.
- first readings of Q-0435 and Q-0436, and a new reading of Q-0430 after the change to STG-28.

The read of the RV7 repair diff is recorded apart, with points RV8-R-1 to RV8-R-10. This review does not repeat those points. It says where its reading of a block differs from them. New points carry IDs of the form RV8-A-n.

## Result

Most of the text agrees with the reference. The five blocks repaired twice agree, each with a small doubt. Four of the five doubts are those that RV8-R-1 and RV8-R-2 already record.

One new sentence reads against the reference:

- STG-28 now says that a `T*` cast from a byte pointer at the start of element `k` of the allocated array points to element `k`, and that the `T*` reaches the object that placement `new` then builds there. For element 0, reached with no byte arithmetic, the sentence agrees. For a later element, the byte pointer must first step across elements, and the cast must then turn a byte position into a pointer to the element. The reference gives neither step a defined result (RV8-A-1). Q-0430 still reads against.

Two items add a condition:

- STG-28 gives a class with a constructor as an example of a type that is not an implicit-lifetime type. Such a class is often an implicit-lifetime type, by STG-7 and by the reference. The note of one new test makes the same slip (RV8-A-2).
- OPR-8, OPR-10, and OPR-P3 give the template form only for a pack of `char`. The reference also allows a literal operator template whose one template parameter is a constant of class type, called by a string literal. OPR-10 read as written rejects that form, and the option that Q-0435 adopts says that every other template head is not valid (RV8-A-3).

OPR-8 and OPR-16 agree on the choice among the forms, with a doubt on how a numeric form matches the kind of literal and where the conflict between the raw form and the template lies (RV8-A-4). Q-0435 is supported with that doubt. Q-0436 is supported (RV8-A-5).

VIRT-13 is a recommendation on warnings, which the reference does not cover. The other F5 text agrees.

## Blocks changed after RV7

| Item | Verdict | Note |
|---|---|---|
| DTOR-8 | agrees with a doubt | RV8-A-6. Read as a whole block with its six tests. The doubt is that of RV8-R-1, the unqualified base name on a derived object. |
| UNI-P3 | agrees with a doubt | RV8-A-7. Read as a whole block. The anonymous union, its members, and the static assertion match. The namespace sentence leaves out the unnamed namespace, which UNI-9 names. |
| UNI-14 | agrees with a doubt | RV8-A-8. Read as a whole block with its 13 tests. The doubt of RV6-A-13 stays. The union object itself must be alive for a new object to become its member. |
| REF-2 | agrees with a doubt | RV8-A-9. Read as a whole block with its seven tests. The three direct cases match. The doubts are those of RV8-R-2. |
| glossary, bind directly | agrees with a doubt | RV8-A-9. The doubt on the converting constructor is that of RV8-R-2. |
| LSUP-6 | agrees | The external declaration with no initializer is not a definition, as RV8-R read. The class with an explicit defaulted constructor and the four `noexcept` forms match. |
| STG-P2 | reads against | The sentence on the typed step is unchanged and keeps the known split of RV7-A-10. The citation of Q-0430 adds no claim. The rest of the principle agrees. |
| LOOP-9 | agrees with a doubt | The doubt is that of RV8-R-3. The review's reading does not differ. |

## Text that item F5 changed

| Item | Verdict | Note |
|---|---|---|
| OPR-8 | adds a condition | RV8-A-3, RV8-A-4. The pack of `char`, the characters it receives, and the choice of the numeric form match. The template form for a string literal is missing. |
| OPR-10 | adds a condition | RV8-A-3. The list now accepts the pack of `char` and still rejects a template with a class constant parameter, which the reference allows. |
| OPR-16 | agrees with a doubt | RV8-A-4. A numeric literal with only the raw form and the template is not valid. The reference places the fault in the literal and leaves the two declarations valid. |
| OPR-P3 | agrees with a doubt | RV8-A-3. The new sentence is right for numeric literals. A string literal can also pass itself to a template. |
| VIRT-13 | reference silent | The reference gives no warnings. A derived function hides every base function of its name, which the bullet assumes and which matches. GCC's wider warning is a choice of the implementation. |
| OBJ-9 example | agrees | A temporary bound to a reference parameter lives until the end of the full expression that holds the call. The full expression is the initialization of `s`, so the member reference dangles and the read is undefined. |
| STG-28 | reads against | RV8-A-1, RV8-A-2. The sentence on element 0 through a byte pointer agrees. The sentence on element `k` reached by byte arithmetic reads against. The example of a type that is not an implicit-lifetime type is wrong. |
| VC-P2 | agrees | In copy initialization from another type, the converting constructor's call is a prvalue of the destination's type, and that prvalue initializes the destination with no copy or move. A class with deleted copy operations can be initialized so. The new test agrees. |
| RFOR-2 example | agrees | The class has both member names, so the loop uses the members. The end type may differ from the iterator type, and the loop compares them with `!=`. The values 1, 4, and 9 and the sum 14 are right. |
| RFOR-P1 | agrees | The scope sentence on coroutines is Tidal's own. The rest follows RFOR-2. |
| glossary, closure type | agrees | Each lambda expression has a unique unnamed class type. |
| glossary, range | agrees | A range-based `for` takes an array, a class with both member names, or a type for which the free functions are found. |

## Questions

| Question | Verdict | Reason |
|---|---|---|
| Q-0430 | reads against | RV8-A-1. The typed step from a byte stays undefined. The new STG-28 case of element `k` through byte arithmetic also reads against. Option 3 stays the closest. Element 0 reached through a byte pointer with no arithmetic is defined. |
| Q-0435 | supports with a doubt | RV8-A-3, RV8-A-4. Option 1 matches for the pack of `char`. Its last sentence, that other template heads are not valid, is too wide, because a template with one class constant parameter is valid for string literals. |
| Q-0436 | supports | RV8-A-5. A literal with a suffix of its own is a separate kind of token. The rule that an integer literal must fit a type covers only the plain kind. The raw form and the template never need the value. |

## RV8-A-1

STG-28, the new sentences on a `T*` cast through a byte pointer.

The sentence has two cases, and the reference treats them differently.

Element 0, with no byte arithmetic. The allocation returns a pointer to one created object at the start of the storage. STG-28's earlier sentence reads that object as element 0 of an array of `T`, and earlier reviews accepted that reading. A cast of that pointer to a byte pointer keeps the pointer value, because no byte object is pointer-interconvertible with element 0. A cast back to `T*` through `void*` then finds element 0 itself. So the `T*` points to element 0, and the two new tests that cast at the start agree. Placement `new` at that address builds an object of the element's type that exactly fills the element's storage inside a live array, so the new object is element 0. The `T*` reaches it. That part of the sentence agrees.

Element `k` for `k` above 0, reached by byte arithmetic. Two steps are needed, and the reference defines neither.

- The step of the byte pointer across elements. If the byte pointer points to element 0 of an array of `T`, its type and the element type are not similar, so the addition is undefined. If the created objects instead include a byte array and the byte pointer points into it, the step is defined.
- The cast of the byte position to `T*`. When the byte pointer points to a byte, the cast keeps pointing to that byte, because a byte and the element whose storage holds it are not pointer-interconvertible. STG-16 lists the same cases and none of them joins a byte to an enclosing object.

No choice of created objects makes both steps defined. After placement `new` at the address, the new object is element `k`, but the `T*` that points to a byte does not reach it. Only the pointer that placement `new` returns does. So the sentence that the `T*` then reaches the new object also reads against for this case. The test with the step of a `long*` after a byte offset keeps the split of RV7-A-10.

This reading differs from RV8-R-10 in one place. RV8-R-10 says that under ADD-6, Tidal's own answer to Q-0304, part B's reading is consistent. ADD-6 lets a byte pointer step through the bytes of an object. The stepped pointer then points to a byte of the storage. For the cast to reach element `k`, Tidal's own STG-16 still needs that byte and element `k` to be pointer-interconvertible, and STG-16 gives no such case. So under Tidal's own rules the new sentence also needs a claim that neither ADD-6 nor STG-16 makes: that a byte pointer at the start of an element points to that element. The sentence's word "So" presents the claim as a consequence of the earlier sentence, and it does not follow from it.

## RV8-A-2

STG-28, the example of a type that is not an implicit-lifetime type.

The sentence names a class with a constructor. STG-7 lists a class with a trivial destructor that is not deleted and at least one trivial eligible constructor as an implicit-lifetime type, and the reference agrees. A class that declares only a converting constructor still has the implicit copy and move constructors, which are trivial and eligible when the members allow. With a trivial destructor, such a class is an implicit-lifetime type. So a class with a constructor is often an implicit-lifetime type, and the example misleads. A class with a user-provided destructor is a correct example.

The note of `allocated-storage-byte-pointer-class-placement.cpp` says that its `Cell`, which has a converting constructor and no destructor of its own, is not an implicit-lifetime type. By STG-7 and by the reference, that `Cell` is an implicit-lifetime type. The expected result is unchanged, since the allocation can create `Cell` objects or an array of them either way. The test with a user-provided destructor is correct as noted.

## RV8-A-3

OPR-8, OPR-10, OPR-P3, and Q-0435, on the forms of a literal operator template.

The reference gives two forms of a literal operator template, and both have an empty parameter list:

- one template parameter that is a pack of constants of type `char`, called by integer and floating literals with the literal's characters,
- one template parameter that is a constant of class type, called by a string literal when the string is a valid template argument for that parameter. When no such template applies, the string literal calls the form with a pointer and a length.

Every other template head makes the declaration not valid. OPR-8 states the first form correctly. The characters are the source characters of the literal without its suffix, so a prefix, a point, an exponent, and digit separators all count, as the page says and the tests check. A string literal with only the pack template finds no form to call, which matches the string test.

OPR-10 now accepts the template only in the form that OPR-8 gives, so read as written it rejects the second form. The Evidence note says that every compiler accepts the second form and that the page does not specify it. The rule needs the second form among the valid declarations, or a statement that the page leaves it out. The same holds for option 1 of Q-0435, whose last sentence rejects every other head. OPR-P3's new sentence is right as far as it goes.

## RV8-A-4

OPR-8 and OPR-16, on the choice among the forms for one suffix.

The reference works on the set of declarations that unqualified lookup finds for the suffix. For an integer literal, a form with an `unsigned long long` parameter wins if present. Otherwise the set must hold the raw form or the pack template, and must not hold both. A floating literal follows the same order with `long double`. OPR-8 and OPR-16 match this, with two doubts.

- OPR-8 says that a numeric form wins when it exists. The numeric form must match the kind of literal. A floating literal with only an `unsigned long long` form falls through to the raw form or the template, and fails without them. The table and `literal-integer-for-float-missing.cpp` imply this, and the sentence does not say it.
- The reference's limit on the raw form and the template falls on the literal. Declaring both for one suffix is valid, and a program that writes no integer or floating literal with that suffix has no fault. GCC rejects the second declaration. The test uses a literal, so its expected rejection matches. The Evidence note reports GCC's place of rejection as a fact, and a reader could still take it as the rule.

## RV8-A-5

Q-0436 and the PRIM-2 sentence on literals that call a literal operator.

In the reference, a literal with a suffix of its own is its own kind of token, built from the digits and the suffix. The rule that an integer literal must fit a type in its list covers the plain kind only. A literal that calls the raw form passes its digits as a string, and one that calls the pack template passes them as characters, so neither needs any integer type to hold the value. Q-0436's option 1 and the tests agree.

For the numeric form, the reference turns the literal into a call whose argument is the same digits written as a plain literal with the suffix for `unsigned long long`. That plain literal must fit its type, so a value too large is not valid, as PRIM-2 and Q-0440 say. One small doubt belongs to Q-0440. The reference lets an implementation give a plain literal that no listed type holds an extended integer type that holds it. On such an implementation, the argument has that type and converts to `unsigned long long` in the call. Q-0440 sits on an expression page, and part B reads it.

## RV8-A-6

DTOR-8 as a whole block. The reference requires the name after the tilde to denote the class whose destructor runs, or, for a scalar, the same scalar type with cv-qualifiers ignored. The block's sentence on a scalar type matches, and `pseudo-dtor-wrong-type.cpp` agrees. A decltype that gives a reference type names a different type, which `dtor-call-decltype-reference.cpp` checks. A destructor may be named only as the callee of a call, and its address may not be taken, which the address and no-call tests check. The qualified base form matches, and the reference shows such calls on a derived object and through a pointer. The wrong-type test names an unrelated class, which is not valid.

The only doubt is that of RV8-R-1: the earlier sentence still says that the name must denote the type of the object, and an unqualified base name on a derived object reads as allowed. The review's reading does not differ.

## RV8-A-7

UNI-P3 as a whole block. In the reference, an anonymous union defines a type with no name and an object of that type. Its member declarations may only define public data members that are not static, or be static assertions, and no nested type, anonymous union, or function may be declared inside it. The members' names are bound in the enclosing scope, and the members share storage. UNI-P3 states each of these. The member whose declaration defines an unnamed class declares a nested type, so its rejection matches.

The principle's last sentence says that an anonymous union at namespace scope needs `static`. The reference requires that only in a namespace with external linkage, so an anonymous union in an unnamed namespace needs no `static`. UNI-9 states that exception, and the principle does not. The word "plain" before the data members is also loose, since a member may have any object type that the union allows. Both doubts are small. The read of the RV7 repair agreed with the block, and this review adds only these two doubts.

## RV8-A-8

UNI-14 as a whole block. The reference makes an object created at the storage of a member into that member when three things hold: the containing object is alive, the new object exactly fills the member's storage, and the new object has the member's type with cv-qualifiers ignored. That is UNI-14's first sentence. It holds with no active member and with another active member, as the block says. An explicit destructor call on the active member ends its lifetime, so no member is active, which `destroyed-member-read-constant.cpp` checks.

A new object inside a member that is not alive, such as an element of an array member or a member of a member union, is not a subobject of that member, because the member's lifetime has not begun. So the member does not become active, at run time or in constant evaluation. The run-time sentence that the program can use the new object through the returned pointer matches RV6-A-14. The trivial copy and move sentence matches RV7-A-4, and both copy and move tests agree.

Two doubts. The first is that of RV6-A-13, which stays: the constant-evaluation allowance for the library's construction function needs storage from the library allocator or an object whose lifetime began in the evaluation, and an inactive member has not begun. Most constant tests here depend on that reading. The second is new and small. The first sentence does not say that the union object itself must be alive. All the tests meet that condition, including the union inside an element of an array from `new[]`.

## RV8-A-9

REF-2 and the glossary entry for a direct binding, as whole blocks.

In the reference, after the lvalue cases and the check that rejects a non-const or volatile lvalue reference, an rvalue binds directly in two cases. In the first, an rvalue that is not a bit-field has a type compatible with the reference. In the second, a class initializer of an unrelated type converts through a conversion function to an rvalue of a compatible type. A prvalue in either case is adjusted to the reference's cv-qualifiers and materialized first. REF-2's three bullets match. `const-ref-derived-xvalue-direct.cpp` and `const-ref-conversion-xvalue-direct.cpp` check the derived xvalue and the conversion function result, and both are direct bindings. `const-ref-binds-rvalue.cpp`, `bit-field-const-ref.cpp`, and the two converting constructor tests reach a temporary, as the lead and the last sentence say.

The glossary entry agrees with the same scope. The doubts are those of RV8-R-2: the first bullet does not exclude a bit-field xvalue, and the reference does not say outright whether a binding through a converting constructor counts as direct. The review's reading does not differ.
