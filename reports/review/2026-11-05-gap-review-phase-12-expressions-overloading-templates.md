# Gap review: phase 12 text on expressions, overloading, templates, and literal operators (RV5, part B)

Date: 2026-11-05

Role: source reader

Reference: the C++23 working draft sources

The review covers OPR-P3 on the literal operators page; COND-3, COND-8, COND-11, COND-16, and COND-P2 on the conditional operator page; NEW-21, NEW-22, NEW-27, NEW-29, and NEW-P5 on the new and delete page; ICS-11, ICS-16, ICS-17, ICS-26, and ICS-45 on the conversion sequences page; OVL-11; DED-29; DEP-P6; and ORD-13. It also gives a first reading of Q-0417, Q-0418, Q-0419, and Q-0420, and it reads LAM-6 and LAM-29 again for the two doubts that the RV4 review left (ledger rows RV4-1 and RV4-2). Later defect resolutions are named only where they may change an answer, and they are named by subject, not by number.

## Result

Most of the text agrees with the reference. Four items read against it: step 1 of COND-16 (with Q-0418), NEW-P5, ICS-11, and ICS-45. Two items add a condition: NEW-22 and ICS-17. On the conditional operator, the C++23 text itself looks defective for class operands that differ in cv-qualifiers, and Tidal follows the compilers there. The review records that as a doubt and does not count it against Tidal. Q-0420 is supported. Q-0417 and Q-0419 are supported with a doubt. Q-0418 reads against the reference. The RV4 doubt on LAM-6 still stands, and the text adds a second one on the term "explicit capture". The RV4 doubt on LAM-29 comes from a silence in the reference, and no Tidal text can remove it.

Gaps found in part B:

- Step 1 of COND-16 needs both operands to be lvalues. The reference needs only the other operand to be an lvalue, and it accepts any binding that is direct to a glvalue. A reference to const binds directly to an xvalue of a compatible class type, so an xvalue operand can take step 1 (RV5-B-1, Q-0418).
- NEW-P5 says that a new expression never adds an alignment argument. The reference adds one on the second try when the type is not over-aligned and the first try finds no function (RV5-B-4).
- ICS-11 says that a prvalue that is not of class type always binds a temporary. The reference calls that binding direct when the referenced type is compatible with the prvalue's type, so the sequence follows the rule for direct binding (RV5-B-6).
- ICS-45 leaves out explicit conversion functions whose result is a class derived from the target class, in the copy or move constructor case. The reference includes them (RV5-B-9).
- NEW-22 limits the sized global form to types that are not over-aligned. The reference picks the sized form for any single object of complete type, aligned forms included (RV5-B-5).
- ICS-17 requires that a known array bound is not less than the number of elements. The reference does not state that condition (RV5-B-7).

## Rules and principles

| Item | Verdict | Note |
|---|---|---|
| OPR-P3 | agrees | Suffixes without a leading underscore are reserved for the language, and suffixes with a double underscore are reserved for the implementation. A declaration that uses one is not valid with no diagnostic required. A token that is also an ordinary literal is read as the ordinary literal. The two parts of an integer suffix take their case independently. |
| COND-3 | agrees with a doubt | The rows follow the reference as the compilers read it. The doubt is RV5-B-2: the C++23 text lets more class operands convert than the table allows. |
| COND-8 | agrees | Both operands go through the conversion from glvalue to prvalue, so the copy or the move must be valid for both, also when one is not selected. A conversion found by the class step initializes a prvalue from the operand. |
| COND-11 | agrees | The built-in candidates for the operator are the pairs of promoted arithmetic types and one candidate for each pointer, pointer to member, and scoped enumeration type. The class operands are converted as the winning candidate says. |
| COND-16 | reads against | Step 1 is stricter than the reference (RV5-B-1). Two doubts on the table: RV5-B-2 and RV5-B-3. |
| COND-P2 | agrees with a doubt | The principle says that two lvalues of one class with a const and a volatile qualifier do not convert. That answer needs the later reading of RV5-B-2. Under the C++23 text both convert and the expression is not valid at once. |
| NEW-21 | agrees | The search for the deallocation function looks in the class unless the expression starts with the scope operator, and it falls back to the global scope when the class has no function with the name. NEW-24 covers the case of no single match. |
| NEW-22 | adds a condition | The sized global form is required for any single object of complete type. NEW-22 adds the limit on alignment, and UNW-3 then leaves the over-aligned case open (RV5-B-5). |
| NEW-27 | agrees | The value of the macro is the alignment that the unaligned global forms guarantee, and an alignment above it is the point at which the alignment argument is passed. The reference does not tie it to the largest fundamental alignment, so that sentence is a fact about the targets. |
| NEW-29 | agrees | For an over-aligned type the second try drops the alignment argument. The rule says nothing about the other direction, which NEW-P5 gets wrong. |
| NEW-P5 | reads against | When the type is not over-aligned and no function accepts the first call, the reference adds the alignment argument and tries again (RV5-B-4). |
| ICS-11 | reads against | A prvalue that is not of class type, of a type compatible with the referenced type, binds directly in the reference's terms (RV5-B-6). |
| ICS-16 | agrees with a doubt | The worst element conversion and the identity for an empty list match. A designated list has no sequence because the parameter is not an aggregate. The choice of the first user-defined element is a Tidal choice that the reference does not make (RV5-B-8). |
| ICS-17 | adds a condition | The reference needs a sequence from each element, and from empty braces for each element past the list. It does not say that the bound must be at least the number of elements (RV5-B-7). The element choice shares the doubt of RV5-B-8. |
| ICS-26 | agrees | The comparison leaves out the lvalue transformations, and the identity is a subsequence of any other sequence. A pointer conversion followed by a qualification adjustment has the pointer conversion alone as a proper subsequence. |
| ICS-45 | reads against | In the case of a temporary for the first parameter of a constructor, the reference permits an explicit conversion function whose result is the class or a class derived from it. ICS-45 names only the class (RV5-B-9). |
| OVL-11 | agrees | When a built-in candidate wins, only the class operands are converted, and the second standard part of their user-defined sequences is not applied. The built-in operator then applies with its own rules. |
| DED-29 | agrees with a doubt | The new paragraph on taking an address agrees with no doubt (see Q-0420). The RV4 doubt on losing call candidates stays (RV5-B-10). |
| DEP-P6 | agrees with a doubt | The row on a failing independent template-id now says no diagnostic is required, as RV4 asked. The doubt concerns the row on an independent static assertion (RV5-B-11). |
| ORD-13 | agrees with a doubt | The swapped case now replaces the positional checks with the reordering test, as RV4 asked. The doubt is the phrase that points at the conditions it replaces (RV5-B-12). |
| LAM-6 | agrees with a doubt | The RV4 doubt on "named capture" stands, because the text has not changed. A second doubt concerns the term "explicit capture" (RV5-B-14). |
| LAM-29 | agrees with a doubt | The reference never says that a closure type is not final. The conclusion follows from the list of properties that an implementation may change. No Tidal text can remove the doubt (RV5-B-15). |

## Questions

| Question | Verdict | Reason |
|---|---|---|
| Q-0417 | supports with a doubt | The second try happens only when no function is found by overload resolution. Resolution selects a deleted function or a function that is not accessible like any other, and the checks for deletion and access come after it. So the selected aligned form ends the search, and the program is not valid. The doubt is that the reference does not define what passes this test (RV5-B-13). |
| Q-0418 | reads against | The reference asks only that the other operand be an lvalue, and that the lvalue reference to its type bind directly to a glvalue. An xvalue is a glvalue, and a reference to const binds directly to an xvalue of a compatible class. So the xvalue operand converts to an lvalue, and both tests in the question get a const lvalue result in the reference. Every compiler gives a prvalue in the one-class test, so the compilers and the reference differ there too (RV5-B-1). |
| Q-0419 | supports with a doubt | When every element uses the same conversion function, the list sequences of the two overloads hold the same function, and their third parts decide. The reference supports the int form there, and GCC's ambiguity reads against it. When elements use different conversion functions, the reference names the worst conversion but never says which of two user-defined sequences with different functions is the worse one. So it is silent on the mixed lists, and the first element is a choice (RV5-B-8). |
| Q-0420 | supports | For a template whose result type is a placeholder, deduction from the target treats the result as a context that deduces nothing. When deduction succeeds, the reference says that the result type comes from instantiating the body. That step comes before the removal of template specializations in favor of a plain function, so the error in the body makes the program not valid. |

## RV5-B-1

Step 1 of COND-16 and Q-0418. In the reference, when the other operand is an lvalue, the target is an lvalue reference to the other operand's type. A sequence counts only if that reference binds directly to a glvalue. The operand being converted does not have to be an lvalue. When the other operand's type is const, a reference to it binds directly to an xvalue of the same class or of a derived class, and that binding is direct in the reference's terms. So a Derived xvalue against a const Base lvalue converts to a const Base lvalue, and the result is an lvalue, as Clang gives. In the one-class test, an A xvalue against a const A lvalue also converts by step 1, and the plain A lvalue reference for the other direction cannot bind. With the later reading of RV5-B-2, the result is a const A lvalue. Every compiler gives a prvalue there, so the compilers do not follow the reference in either test. An older text asked for an lvalue at this point, and the current text asks for a glvalue, which suggests the change was deliberate. The choice for Tidal is between the reference and the compilers.

## RV5-B-2

COND-16 table, COND-3, and COND-P2. In the C++23 text, the class step has three cases in order: the same class when the other type has at least the operand's cv-qualifiers, then a base class of the operand's class, then a fallback that uses the other operand's type after the usual decay conversions. A same class whose other type lacks some qualifier of the operand falls through to the fallback, and so does a base class operand against its derived class. In both cases a sequence can then often be formed. A const Box lvalue against a Box prvalue then converts both ways, and the expression is not valid. A Base operand against a Derived with a constructor from Base also converts both ways. All the compilers stop the conversion in these cases, as Tidal's table does. A later defect resolution on this operator restores that behavior for the same-class case, as far as the reviewer knows. The reviewer is not sure that it also covers the base class operand. Tidal can keep its table. The doubt is that the C++23 text reads against it, so the Evidence segment could say which reading the table follows.

## RV5-B-3

COND-16, the row for a base class target. In the reference, the target is the base class with the cv-qualifiers of the operand's own type. The other operand's qualifiers do not take part. Tidal's row uses the other operand's type and requires it to carry the operand's qualifiers. The two give the same final answer for a const Derived lvalue against a Base prvalue, since both make the expression not valid. The paths differ, though. The reference converts the Derived operand to a const Base prvalue and then finds no common type. The note in `class-base-less-qualified-rejected.cpp` describes the reference's path, and the table describes the other one. For a Derived prvalue against a const Base lvalue, the reference converts the Derived operand to a Base prvalue with no qualifier. Then two operands of one class that differ only in cv-qualifiers remain, and no later step of the reference joins them, so the text makes the expression not valid. The compilers accept it, and `class-derived-prvalue-const-base-lvalue.cpp` expects that. The reference looks defective here, as in RV5-B-2. The doubt is real, but Tidal's answer is the useful one.

## RV5-B-4

NEW-P5. The reference first tries the call with the alignment argument when the type is over-aligned and without it otherwise. If no function is found, it changes the argument list and tries once more. For an over-aligned type it removes the alignment argument, as NEW-29 says. For a type that is not over-aligned it adds an alignment argument after the size, with the type's own alignment. So a class that declares only an aligned allocation form is still called for an ordinary type. NEW-P5 says the program is then rejected, and that the expression never adds an alignment. Both statements read against the reference. The compilers may not implement the added argument, so a probe would show which behavior Tidal should record. The same doubt may reach the deallocation side, where the reference prefers the forms that match the type's alignment but keeps the others when no preferred form exists.

## RV5-B-5

NEW-22. For a global deallocation function, the reference first keeps the aligned forms for an over-aligned type and the unaligned forms otherwise. If two forms remain, it picks the sized one when the type is complete. For an array it also needs a class element type with a destructor that is not trivial. The alignment plays no part in that last test. So an over-aligned single object of complete type gets the sized aligned form. NEW-22 and UNW-3 limit the sized form to types that are not over-aligned and leave the over-aligned case open.

## RV5-B-6

ICS-11. In the reference, a reference to const, or an rvalue reference, binds directly to an rvalue whose type is compatible with the referenced type. The prvalue is first given the referenced type's top qualifier and then materialized, and the binding still counts as direct. Only the last case, which converts the initializer to a new temporary of the referenced type, is indirect. So a pointer prvalue that only gains qualifiers binds directly, and the sequence for direct binding is the identity, the same as for the lvalue of that type. ICS-11 gives the prvalue a qualification adjustment and the lvalue the identity. That difference can change a ranking between two reference overloads. A later defect resolution, on reference binding between similar types, may give both cases a qualification conversion. Under that reading, the lvalue case of ICS-11 changes. A probe with two reference overloads that differ only in qualifiers would show what the compilers do.

## RV5-B-7

ICS-17. The reference forms the sequence for an array of known bound from each element, and from empty braces for the elements past the list when the bound is larger. It says nothing about a list that is longer than the bound. Taken literally, the sequence exists, and the ranking rule for arrays would even prefer the smaller bound. The initialization would then fail after overload resolution. The condition in ICS-17 matches the evident intent and, most likely, the compilers. It is still a condition that the reference does not state.

## RV5-B-8

ICS-16, ICS-17, and Q-0419. The reference gives the list sequence as the worst element conversion. Two user-defined sequences are comparable only when they use the same function or initialize the same aggregate class. Elements that use different conversion functions therefore have no worst one. The reference does not say which of them the list sequence holds, or that the list sequences are then indistinguishable. Tidal's choice of the first such element is a rule that the reference does not contain. GCC's reading, that such list sequences tie, is also open under the reference for the mixed lists. When all elements share one function, the reference decides, and it supports Tidal.

## RV5-B-9

ICS-45. In the case where a temporary is made for the first parameter of a constructor, with a single argument, in the direct initialization of an object of the class, the reference also permits explicit conversion functions. The permitted results are the class itself and any class derived from it. ICS-45 names only the class or a reference to the class. The reference allows a constructor whose first parameter is a reference to the class, which covers the copy and move constructors, and the reviewer sees no gap there. An explicit conversion function that returns a derived class can serve in the reference, and ICS-45 leaves it out.

## RV5-B-10

DED-29. The RV4 doubt on losing call candidates stays. A note in the reference says that any use of a specialization with a deduced result type causes its instantiation. The reference does not say that being a candidate is a use, so the rule is a fair reading. The new paragraph on taking an address agrees with no doubt.

## RV5-B-11

DEP-P6, the row on a static assertion. The reference says that a static assertion evaluated in a template definition has no effect. It also leaves out failing static assertions when it asks whether any valid specialization exists. A separate case makes a program not valid with no diagnostic required when a hypothetical instantiation right after the definition fails because of an independent construct. That case does not leave out static assertions. Read literally, an independent false assertion in a template that is never instantiated falls under it. The intent of the change in C++23 was to allow such an assertion, and the compilers allow it. The principle follows the intent. The doubt is in the reference.

## RV5-B-12

ORD-13. The bullet for a swapped rewritten candidate says that it takes the place of "the two before it", and then names the conditions on the parameter types and on the heads. The two bullets directly before it are the ones on heads and on the ellipsis, and the bullet on parameter types is further up. A reader can pick the wrong pair. The meaning is right once the reader follows the named conditions. The reference replaces the positional head and type checks with the reordering test, and it keeps the check on the length of both parameter lists, which ORD-13 also keeps.

## RV5-B-13

Q-0417. The reference tries again only when the first search finds no suitable function. It does not define what counts as suitable for this step. The natural reading is that overload resolution has failed. Deleted functions and inaccessible functions take part in overload resolution like the others, and the problems they cause are found after the selection. Under that reading, the selected aligned form ends the search. GCC's reading treats a deleted form as not matching, which the reference does not support.

## RV5-B-14

LAM-6 (ledger row RV4-1). The RV4 note still applies. The reference limits a lambda outside a block scope, apart from a default member initializer, to no capture default and no simple capture. A simple capture covers a name, a name by reference, this, and a copy of the object, and it does not cover an init capture. LAM-6 still says "named capture", which the page does not define, and only LAM-7 makes clear that an init capture is allowed at namespace scope. A second doubt arises from the term "explicit capture". In the reference, only a simple capture explicitly captures an entity, and the rule on modes after a default reaches only simple captures. So an init capture by copy after a copy default, or by reference after a reference default, is allowed. LAM-7 calls init captures explicit captures. Under that use, the second bullet of LAM-6, and the last sentence of LAM-5, would forbid those init captures or give them the other mode. Other places on the page use "explicit capture" for the simple captures only. One meaning across the page would remove both doubts.

## RV5-B-15

LAM-29 (ledger row RV4-2). The reference describes the closure type as a unique class with listed members. It lets an implementation change only the size, the alignment, trivial copyability, and standard layout. Finality would change whether a class can derive from the closure type, which is observable, so a closure type cannot be final. The reference never says so in plain terms. The doubt comes from that silence, and LAM-29 states the right result. The writer can close the row on that basis.
