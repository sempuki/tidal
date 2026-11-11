# Gap review: phase 15 text on expressions, overloading, and templates (RV8, part B)

Date: 2026-11-11

Role: source reader

Reference: the C++23 working draft sources

Commit read: e7e972d6b6e005bff03afcc42fded7c27b7cb9bb

The review is item RV8 of decision 0063, gap review part B. It reads the blocks on these pages that changed after RV7 read the tree: CNS-P3, CNS-13, DED-20, DEP-P1, DEP-P5, DEP-P6, DEP-4, DEP-6, DEP-30, DEP-40, INST-P1, INST-1, and INST-2. CNS-P3, CNS-13, and DED-20 are read in full. DEP-P5 is read as a whole block, with the tests of its rules. The review reads the text that item F5 changed on these pages in commit 232ac618: the class template deduction page, INST-1, INST-5, INST-P2, PRIM-2, and the new examples on the comparison, decltype, lambdas, and overload resolution pages. It gives a first reading of Q-0437, Q-0438, Q-0439, Q-0440, and Q-0441. The read of the RV7 repair diff is recorded apart (points RV8-R-1 to RV8-R-10). This review does not repeat those points and says where its reading differs. New points carry IDs of the form RV8-B-n. Later defect resolutions are named only by subject.

## Result

Most of the text agrees with the reference. Q-0437 and Q-0440 are supported. Q-0438 and Q-0441 are supported with a doubt. Q-0439 is supported for a static member that is not inline and reads against for an inline one.

Four items read against the reference:

- CTAD-2's new bullet and Q-0439, for an inline static data member with no initializer. The reference counts default initialization as a declaration that initializes, and an inline member defined in the class with no initializer is one. So deduction is allowed there, as GCC does (RV8-B-4).
- CTAD-17's bullet that frees the guides from the limit to standard sequences. The reference forms the guides as the constructors of a notional class and runs the usual initialization rules on them. Read as written, the limit on the first parameter in copy initialization reaches the guides, which is Clang 18's answer (RV8-B-5).
- CTAD-16's new sentence that keeps the guide with no parameters and the aggregate candidate with the base. The reference builds the inherited guides from the base's whole guide set, and that set holds both (RV8-B-6). The two new tests reject under either reading.
- DEP-P5, in the two rows that Q-0225 and Q-0226 already hold in `pending.md` row 4. The other rows agree (RV8-B-2).

The other points are doubts. The RV8-R points on these blocks stand. RV8-R-4 (the immediate invocation in INST-1, INST-2, and INST-P1), RV8-R-5, RV8-R-6, RV8-R-7, and RV8-R-8 still apply, because the text they name has not changed since. This review differs from the read of the RV7 repair diff in one place. That read finds the CNS-P3 row and the first bullet of CNS-13 in full agreement. This review adds a doubt on the scope of the no-diagnostic latitude (RV8-B-1).

## Blocks changed after RV7

| Item | Verdict | Note |
|---|---|---|
| CNS-P3 | agrees with a doubt | Each row matches the reference: viability in overload resolution, removal from an address set, a hard error for a single named function, the template-id with known arguments, the dependent template-id checked when made, the template template parameter case with no diagnostic, explicit instantiation and specialization, members declared and unusable, partial specialization matching, the lambda conversion, the destructor choice, eligibility of special members, and placeholder types. The closing paragraph matches the order of the reference: defaults of template parameters, then the constraint check, then the function type. The doubt is the condition "if the template is never instantiated" in the row for an independent template-id (RV8-B-1). |
| CNS-13 | agrees with a doubt | The three bullets match the reference, with its example of a template-id, an explicit instantiation, and an explicit specialization. The first bullet carries the same scope doubt (RV8-B-1). |
| DED-20 | agrees | A trailing function parameter pack takes the remaining arguments, and a pack before the end is a non-deduced position. A pack not otherwise deduced is empty. The sentence on a pack that only expands the enclosing class's pack agrees, since making the class makes the member template's declaration with that pack expanded. The two pack tests agree. |
| DEP-P1 | agrees with a doubt | The table rows agree. RV8-R-7 stands for the word "Only". |
| DEP-P5 | reads against | Two rows read against, as the questions record. The other rows agree. See RV8-B-2. |
| DEP-P6 | agrees with a doubt | RV8-R-8 stands. The row for an independent template-id agrees with the reference's own example. The sentence on a static assertion agrees with this draft, which leaves failed static assertions out of the test for a template with no valid specialization. RV8-B-1 applies to the "need not diagnose" rows. |
| DEP-4 | agrees with a doubt | The typeid case agrees. The list leaves out some value-dependent forms (RV8-B-3). |
| DEP-6 | agrees | An undeclared unqualified name in a template is an error at the definition, as the reference's example shows. The exception for DEP-17 carries RV8-R-8. |
| DEP-30 | agrees with a doubt | The bullets agree. The lead says that an implementation need not diagnose when no specialization is made. RV8-B-1 concerns that limit. |
| DEP-40 | agrees with a doubt | RV8-R-6 stands. |
| INST-P1 | adds a condition | RV8-R-4 stands for the class row. The other rows agree. |
| INST-1 | adds a condition | RV8-R-4 stands. F5's new words on a function definition agree: the parameter and return types of a definition must be complete, and for a member of a class template the definition is made only when a use needs it. |
| INST-2 | adds a condition | RV8-R-4 and RV8-R-5 stand. |

## F5 text, commit 232ac618

| Item | Verdict | Note |
|---|---|---|
| CTAD-P1, user guide with no template head, step 5 | agrees | A user guide without a template head is a plain function in the reference's guide set, and the rule that prefers a function that is not a template comes before the guide tie-breakers. The initialization after the choice picks its own constructor. The reference shows a guide that deduces a type whose constructors then clash. |
| CTAD-P3 | agrees | The four tie-breakers come after partial ordering and constraints, in the order of the table. The note on OVL-27 agrees. |
| CTAD-1 | agrees with a doubt | A new expression with no initializer is allowed, since the reference deduces as for an invented variable with the new initializer, if any. A variable with no initializer is allowed. The list leaves out an inline static data member with no initializer (RV8-B-4). The older sentence on a dependent qualifier stays with Q-0243 in row 4. |
| CTAD-2 | reads against | The bullet holds for a static member that is not inline. It reads against for an inline one (RV8-B-4). |
| CTAD-5 | agrees with a doubt | The reference forms a guide for each constructor of the class and does not limit that to declared ones (RV8-B-7). The conditional explicit sentence agrees. |
| CTAD-9 | agrees | A braced element for a member of aggregate class type forms a user-defined sequence through aggregate initialization, which is ICS-18. |
| CTAD-10 | agrees | A member or base whose type is not dependent has its braces elided as usual, because the reference turns elision off only for the three dependent cases. The older sentence on a string literal for an array of dependent element type stays with Q-0277, whose recorded review reads against it. |
| CTAD-14 | agrees | The guides of an alias come from the guides of the template it names, so an alias of an alias runs the steps from the inner one out. The constraints of each alias apply through the check that the alias's arguments can be deduced from the result. |
| CTAD-16 | reads against | The list of what the derived class takes agrees as far as it goes. The sentence that keeps the guide with no parameters and the aggregate candidate with the base reads against (RV8-B-6). The sentence on the inherited copy candidate agrees: the base's copy constructor is not a candidate for the derived class, so the initialization fails. |
| CTAD-17 | reads against | The first bullet agrees (Q-0437). The third bullet agrees, since the reference treats every notional constructor as public. The second bullet reads against (RV8-B-5). |
| CTAD-18 | agrees with a doubt | The two phases and the single-element exception agree. "Unlike ILC-3" agrees, because the matching rule for ordinary list initialization applies only to an aggregate. The definition and the empty-list sentence carry doubts (RV8-B-8). |
| CTAD-22 | agrees | The text did not change. It matches the last guide tie-breaker. |
| CTAD-25 | agrees | The constructor that runs is chosen again for the deduced type. |
| CTAD-26 | agrees | Tests line only. |
| INST-5 | agrees with a doubt | The declarations of member functions and static data members are made with the class, and their definitions wait. The doubts are the word "only" and the inline static member (RV8-B-9). |
| INST-P2 | agrees with a doubt | The two changed rows carry the RV8-B-9 doubts. |
| PRIM-2 | agrees | The numeric form passes the digits as an unsigned long long literal, which must have a type. The raw form and the template take the characters and have no limit (RV8-B-10). |

## Examples

| Example | Verdict | Note |
|---|---|---|
| CTAD-10 elision examples, the struct member and the fixed int array | agrees | Neither member is dependent, so the first two values fill it and the third gives the parameter. |
| CTAD-14 worked alias example | agrees | The match of the guide's result with the alias's target fixes the first parameter and maps the second to the alias's parameter. The new guide has that shape. |
| CTAD-25 and CTAD-26 reject example | agrees | The user guide wins the guide choice on exact matches. The two constructors of the deduced type then tie, each with one conversion on a different argument. |
| Comparison page, pointer equality and the character loop | agrees | Equality of two character pointers compares addresses. Whether equal literals share storage is unspecified. The loop compares the characters. |
| Decltype page, a declared function in an unevaluated call | agrees with a doubt | The operand is unevaluated, so no function is used and no definition is needed. The result of the call is a scalar prvalue, so decltype gives the plain type. The doubt is small. The phrase "a function template is not instantiated" means its definition, because the declaration of the specialization is still formed. |
| Lambdas page, the owner of one closure | agrees | Copy initialization from the closure picks the converting constructor template. Its result initializes the object directly, so the deleted copy constructor is never needed. The copy constructor is not viable there, since its parameter would need a user-defined conversion. The virtual destructor makes the delete through the base defined. |
| Overload resolution page | agrees | Only OVL-14's tests line changed. The new test matches OVL-14 and VC-P2 for the reason above. |

## Questions

| Question | Verdict | Reason |
|---|---|---|
| Q-0437 | supports | For a candidate whose explicit condition depends on template parameters, the reference deduces first. In a context that accepts only candidates that are not explicit, it then drops the specialization when the condition is true. A guide keeps the explicit condition of its constructor. So copy initialization drops the guide that is explicit for its own deduced arguments before the choice, as Clang and EDG do. |
| Q-0438 | supports with a doubt | An empty list with a default constructor skips the phase of initializer-list constructors, and an earlier rule value-initializes in that case. The guides act as the constructors of a notional class. A guide that takes no arguments is the natural default constructor of that class, which is Clang's reading. The reference does not say what a default constructor of the notional class is (RV8-B-8). |
| Q-0439 | supports for a non-inline member; reads against for an inline member | See RV8-B-4. |
| Q-0440 | supports | The numeric form uses the digits as an unsigned long long literal. A literal that no allowed type holds makes the program not valid. GCC's warning is a diagnostic, which is all that the reference demands of an ill-formed program. So GCC conforms too, and the split is in how the tests count a warning (RV8-B-10). |
| Q-0441 | supports with a doubt | Making the class makes the declaration of a static data member and not its definition. A non-inline static member declared in its class may have an incomplete type. The doubt is the inline member (RV8-B-9). |

## RV8-B-1

CNS-P3's row for an independent template-id inside a template, CNS-13's first bullet, the lead of DEP-30, and the matching DEP-P6 rows. Tidal says that an implementation need not diagnose such a construct if the template is never instantiated. That reading implies a required diagnostic once a specialization is made.

The reference gives the latitude through its rule on a specialization imagined right after the definition. That rule does not depend on whether the program later makes a specialization. Its own example for an independent template-id says only that the case needs no diagnostic. A note beside the rule says that the errors of a template that is made are diagnosed under the other rules, which supports Tidal's reading for an actual instantiation. So the reference leans toward Tidal but does not settle the point. The tests do not separate the readings: the tests of the case make no specialization, and every compiler rejects at the definition. The doubt is small. RV8-R found the CNS-13 hunks in full agreement, and this point is the one place where this review reads them differently.

## RV8-B-2

DEP-P5 as a whole, with the tests of DEP-19 to DEP-27 and DEP-34.

The rows for a dependent name read as a value outside a type-only position, for the keyword typename, for the less-than after a dependent name, for an object of the current instantiation, for the keyword template with an argument list, and for typename with template and a list all agree with the reference. The tests that cite those rows agree.

Two rows read against the reference, as earlier reviews found:

- The row for an argument list after typename, in a base list, or in an alias target, with no keyword template (Q-0225). In the reference, the last name in a position that allows only a type is taken as a template name, so the less-than opens an argument list. The three tests of that row expect rejection, and the reference makes all three forms valid.
- The row for typename with template and no list before a deduced class type (Q-0226). The reference's form for a deduced class type has no place for the keyword, and its qualifier must not be dependent. The test with that form expects acceptance.

The sentence on the middle column agrees as text, as RV8-R found. One small doubt is new. DEP-25, which the block cites, allows the keyword template with no list before a template template argument. The reference allows that form and marks it deprecated. Tidal states it with no mark.

## RV8-B-3

DEP-4's list. The bullets agree as far as they go. The reference makes some more forms value-dependent:

- the use of a concept with a dependent template argument,
- a name of a static member of the current instantiation whose meaning depends on the parameters: a data member with no initializer in the class, or a member function,
- the address of a qualified name of a dependent member of the current instantiation.

The variable bullet names a const integer variable and a constexpr variable. The reference's set also holds a const variable of enumeration type and a variable of reference type, when the initializer is value-dependent. None of these changes a test, and the doubt is small.

## RV8-B-4

CTAD-1, CTAD-2, and Q-0439, on a static data member with a deduced class type and no initializer.

The reference allows the placeholder in the declaration that sets up a variable's initialization. It counts default initialization in that. The in-class declaration of a static member that is not inline is not a definition, and with no initializer it does not set up the initialization. So the placeholder is not valid there, and CTAD-2 agrees for that form.

An inline static member with no initializer is defined in its class, and that definition default-initializes it. So the in-class declaration does set up the initialization, and the placeholder is valid. Deduction then uses no arguments, as for a variable with no initializer. GCC gives that answer. CTAD-2's bullet reads against for the inline form, and CTAD-1's list leaves it out.

The test `site-static-member-no-initializer.cpp` declares both members in one class. Its expectation of rejection holds through the member that is not inline alone, so the test does not check the inline form.

## RV8-B-5

CTAD-17, the second bullet. The reference forms the guides as the constructors of a notional class and runs the usual rules of initialization on them. Copy initialization from a value of another type then chooses among converting constructors in the usual way. In that path, the reference bars user-defined sequences for the first parameter of a candidate constructor. It states no exception for the guides. Read as written, a guide whose parameter needs a conversion function of the argument is not viable in copy initialization. That is Clang 18's answer.

Clang 22, GCC, and EDG accept the test `copy-init-guide-user-conversion.cpp`, and Tidal follows them. The limit exists to stop two user-defined conversions in one initialization. A guide's choice makes no conversion that runs, which may explain the newer compilers. The reference text does not make that distinction. The bullet reads against the text, and the Evidence note and Q-0020 should name the reading.

## RV8-B-6

CTAD-16, the sentence that keeps the base's guide with no parameters and its aggregate candidate with the base.

In the reference, a class template that inherits constructors takes the guides of an alias formed from its base. The guides of that alias come from the base's whole guide set. That set holds the guide with no parameters when the base declares no constructor. It also holds the aggregate candidate when the base is an aggregate, has no user guide, and the initializer is a non-empty list. Nothing in the reference removes either one for the inherited case. So the sentence reads against.

The two new tests reject under either reading:

- `inherited-no-guide-without-parameters.cpp`. The match of the alias replaces the base's parameter with the derived one, so the base's default does not reach the derived parameter. Nothing deduces it.
- `inherited-no-aggregate-candidate.cpp`. The inherited aggregate candidate would deduce the type. The initialization of the derived class from one integer then fails, since the derived class is not an aggregate and has no fitting constructor.

The error messages of GCC and EDG show that they do not use the inherited candidates. Clang has no inherited guides, so its result says nothing here. A program could separate the readings: a derived class whose own constructor template takes one integer and cannot deduce the class parameter. The inherited aggregate candidate would give the type, and that constructor would then initialize. No test or probe in the record has that shape.

## RV8-B-7

CTAD-5, "An implicitly declared constructor gives none." The reference forms one guide for each constructor of the class. Its rule for the guide with no parameters speaks of a class that declares no constructor, which suggests that implicit constructors do not count as declared. The rule for the constructor guides has no such limit. Read literally, implicit copy and move constructors give guides too.

The Evidence note says that no probe separates the readings. The reference agrees with that. Such guides give the same result type as the copy candidate, and the copy candidate's tie-breaker wins when they tie. The doubt is small.

## RV8-B-8

CTAD-18 and Q-0438.

The definition of an initializer-list guide asks for a first parameter of an initializer list type. For constructors, the reference also counts a first parameter that binds by reference to an initializer list type, with or without const or volatile. A guide from such a constructor, or a user guide written that way, falls outside CTAD-18's definition as written.

For an empty list, the reference value-initializes a class with a default constructor, and its two-phase rule also skips the first phase in that case. For the guides, the notional class needs a default constructor for either rule to apply. A guide that takes no arguments plays that role in Clang's reading, and the reference supports that reading by its general model. The reference does not say it. CTAD-18 also speaks of a guide with no parameters. A guide whose parameters all have defaults takes no arguments as well, and a default constructor in the reference includes that case.

## RV8-B-9

INST-5's new paragraph, the two changed INST-P2 rows, and Q-0441.

For member functions, the reference requires complete parameter and return types for a function definition. A call also needs a by-value parameter type and a class result type complete. A declaration needs neither. So the new text agrees on the class instantiation. INST-5 says the types need to be complete "only when the definition is made". A call that passes or returns the class by value also needs them, and INST-1 lists those calls. The sentence cites INST-1, so the conflict is only in the word "only". The doubt is small.

For static data members, the reference makes the class instantiation produce their declarations and not their definitions. A static member that is not inline may be declared in its class with an incomplete type. So that form does not need its type complete when the class is made, as Clang and EDG say.

For an inline static member, the declaration in the class is a definition outside templates, and a definition needs a complete type. The class instantiation still makes only its declaration. The reference treats an instantiated declaration that comes from a definition as a definition only for the checks of a redeclaration. So the text leans toward Clang's answer for the inline form too. Q-0231 covers the same ground. The doubt is whether making the declaration of an inline member must also check its type. GCC and EDG check it, and the reference does not say so.

## RV8-B-10

PRIM-2 and Q-0440. When the numeric form exists, the reference treats the literal as a call with the digits written as an unsigned long long literal. That literal must have a type in its list, and the list allows an extended unsigned integer type when one can hold the value. So on an implementation with no wider extended unsigned type, the value must fit unsigned long long. PRIM-2's sentence states that limit, which holds for Tidal's targets. The raw form and the template receive the characters, so the reference sets no limit on the digits there. That agrees with PRIM-2 and Q-0436.

GCC accepts the large literal with a warning. The reference requires a diagnostic for an ill-formed program, and a warning is one. GCC's result conforms, and the test's reject expectation for GCC is a matter of how Tidal counts a warning.
