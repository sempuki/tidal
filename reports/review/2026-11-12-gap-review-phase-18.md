# Gap review: phase 18 text and the old gaps (RV12)

Date: 2026-11-12

Role: source reader

Reference: the C++23 working draft sources.

Commit read: 40f0a273dafb779217b172ca265910c720ad5b01.

The review is item RV12 of decision 0066. Part A covers five groups of text:

- the specification text of item F8 (commit e4561158) and its 18 new tests,
- the specification text of item GR2 (commit d784bf57) and its six new tests,
- ICS-16, which no review has read since phase 12,
- the rule tests of commit 3bb416e8,
- Q-0424, Q-0427, Q-0432, and Q-0435, each read with the principle that it waits for.

Part B gives a one-line judgment on eight points that the text keeps against the reference with no question.

## Result

Three items read against the reference. Each one is a known split that a question or a row 4 item already holds:

- UNI-13's new sentence on a default constructor deleted at its first declaration. The reference lets an assignment activate a union member of class type only when the class's default constructor is trivial and also not deleted. So the deleted form activates nothing, and GCC's rejection of the new test conforms. Q-0455 holds the split (RV12-8).
- TARG-16's new sentence on a braced list for a parameter of scalar type, the matching words in the arguments paragraph, and `braced-scalar-arguments.cpp`. The reference's template argument has no braced form at all, as RV10-6 found for the class case (RV12-10).
- The new sentence of the casts paragraph on a deduced class template name after a dependent qualifier, and the Q-0226 Divergences line. The reference requires the qualifier before a deduced name to be independent, and its typename form takes no `template` keyword before a plain name. Q-0243, Q-0226, and the row 4 item "casts paragraph, CTAD-1" hold these (RV12-5).

The rest of the F8 text agrees. Four items agree with a doubt:

- the list of discarded-value places in the decltype Syntax segment and the glossary entry, which do not limit the comma to the built-in operator (RV12-1),
- UNEV-2's list of the forms that read a volatile object, which is shorter than the reference's (RV12-1),
- TPL-4's condition on the friend with two template heads, which is narrower than the reference's condition (RV12-4),
- OBJ-2's new clause, which carries the doubt of RV10-12 (RV12-9).

UNEV-3, COMMA-5, and STMT-4 have no counterpart in the reference. Its only recommended warning for a discarded value is the one for a call marked `nodiscard` (RV12-2).

The GR2 text agrees in full (RV12-11, RV12-12). ICS-16 agrees, and its second paragraph covers a choice that the reference leaves open (RV12-13). The eleven rule tests of 3bb416e8 agree. One test agrees with a doubt (RV12-14).

Each of the four principles answers its question, and each answer agrees with the reference. DEP-P2's reason carries a small doubt. OPR-P3 answers only part of Q-0435's option 1 (RV12-15).

## Text of item F8 (commit e4561158)

| Item | Verdict | Note |
|---|---|---|
| UNEV-P2, the temporary bullet | agrees | RV12-1. |
| UNEV-2 | agrees with a doubt | RV12-1. The list of volatile reads is short. |
| decltype Syntax, the list of discarded-value places | agrees with a doubt | RV12-1. The comma item applies only to the built-in operator. |
| glossary, discarded-value expression | agrees with a doubt | RV12-1. The same doubt applies. |
| LOOP-3 and the LOOP Note | agrees | RV12-1. |
| COMMA-1 | agrees | RV12-1. |
| UNEV-3 | reference silent | RV12-2. |
| COMMA-5 | reference silent | RV12-2. |
| STMT-4 | reference silent | RV12-2. |
| decltype Examples block and Evidence table | agrees | RV12-2. The comments match UNEV-3. |
| PRIM-28 and the PRIM Example | agrees | RV12-3. |
| TPL-4 and the TPL Note | agrees with a doubt | RV12-4. |
| casts paragraph, the alias template | agrees | RV12-5. |
| casts paragraph, the dependent qualifier | reads against | RV12-5. Q-0243, row 4a. |
| casts Divergences, the Q-0240 line | agrees | RV12-5. |
| casts Divergences, the Q-0226 line | reads against | RV12-5. RV10-10 stands. |
| ATTR-2, the using-directive item | agrees | RV12-6. |
| ATTR-19, the using-directive row | agrees | RV12-6. |
| AUTO Note (AUTO-11) | agrees | RV12-7. |
| UNI-13, the deleted default constructor | reads against | RV12-8. Q-0455. |
| OBJ-2, the member of a union's class member | agrees with a doubt | RV12-9. |
| TARG-16, the scalar sentence, and the arguments paragraph | reads against | RV12-10. RV10-6 stands. |
| Q-0454 line | agrees | RV12-10. The rejection holds under either reading. |

## New tests of item F8

| Test | Verdict | Note |
|---|---|---|
| `void-cast-temporary-lives-to-full-expression-constant.cpp` | agrees | RV12-1. |
| `for-iteration-temporary-constant.cpp` | agrees | RV12-1. |
| `for-iteration-name-warns.cpp` | reference silent | RV12-2. The program is valid. The test checks a recommended warning. |
| `unused-left-literal-warns.cpp` | reference silent | RV12-2. |
| `unused-left-member-warns.cpp` | reference silent | RV12-2. |
| `discarded-effects-no-warning.cpp` | agrees | RV12-2. The program is valid. |
| `fold-comma-overloaded-class.cpp` | agrees | RV12-3. |
| `friend-two-heads-member-template.cpp` | agrees | RV12-4. |
| `friend-two-heads-fixed-qualifier.cpp` | agrees | RV12-4. |
| `functional-cast-alias-template-deduced.cpp` | agrees | RV12-5. |
| `functional-cast-dependent-qualifier-deduced.cpp` | reads against | RV12-5. |
| `deprecated-on-using-directive.cpp` | agrees | RV12-6. |
| `nodiscard-on-using-directive.cpp` | agrees | RV12-6. |
| `decltype-auto-before-scope.cpp` | agrees | RV12-7. |
| `chain-assign-deleted-default-constructor-constant.cpp` | reads against | RV12-8. GCC's rejection conforms. |
| `write-after-pseudo-destructor-union-struct-member-constant.cpp` | agrees with a doubt | RV12-9. |
| `braced-scalar-arguments.cpp` | reads against | RV12-10. |
| `braced-integer-for-floating.cpp` | agrees | RV12-10. The program is not valid under either reading. |

## Text of item GR2 (commit d784bf57)

| Item | Verdict | Note |
|---|---|---|
| `union_def` | agrees | RV12-11. |
| UNI-17 | agrees | RV12-11. |
| UNI-P1, the new sentence | agrees | RV12-11. |
| TSP-32 | agrees | RV12-11. |
| LIST-4 and its example | agrees | RV12-12. |
| `union-qualified-definition.cpp` | agrees | RV12-11. |
| `member-union-template-outside.cpp` | agrees | RV12-11. |
| `union-key-union-template.cpp` | agrees | RV12-11. |
| `no-elision-conversion-function.cpp` | agrees | RV12-12. |
| `no-elision-conversion-too-many.cpp` | agrees | RV12-12. |
| `no-elision-string-literal-member.cpp` | agrees | RV12-12. |

## ICS-16 and the tests of commit 3bb416e8

| Item | Verdict | Note |
|---|---|---|
| ICS-16, first paragraph | agrees | RV12-13. |
| ICS-16, second paragraph | reference silent | RV12-13. Q-0419 and Q-0423 stand. |
| `closure-argument-converting-constructor-template.cpp` | agrees | RV12-14. |
| `closure-lvalue-converting-constructor-template-thunk.cpp` | agrees | RV12-14. |
| `generic-reference-capture-closure-parameter.cpp` | agrees | RV12-14. |
| `generic-reference-capture-member-pointer.cpp` | agrees | RV12-14. |
| `generic-reference-capture-object-call.cpp` | agrees | RV12-14. |
| `generic-reference-capture-template-argument.cpp` | agrees | RV12-14. |
| `static-constexpr-member-calls-own-member.cpp` | agrees with a doubt | RV12-14. |
| `class-static-constexpr-member-calls-own-member.cpp` | agrees | RV12-14. |
| `explicit-specialization-body-sees-namespace.cpp` | agrees | RV12-14. |
| `constexpr-requirement-too-many-values-discards-binding.cpp` | agrees | RV12-14. |
| `requirement-too-many-values-nested-aggregate.cpp` | agrees | RV12-14. |
| `constant-parameter-class-template-string-literal.cpp` | agrees | RV12-14. |

## Questions read with their principles

| Question | Principle | Answers the question | Verdict | Note |
|---|---|---|---|---|
| Q-0424 | UNI-P3 | yes, option 1 | supports | RV12-15. |
| Q-0427 | DEP-P5 | yes, option 1 | supports | RV12-15. |
| Q-0432 | DEP-P2 | yes, option 1 | supports with a doubt | RV12-15. The doubt is on the reason that DEP-P2 gives. |
| Q-0435 | OPR-P3 | in part | supports | RV12-15. OPR-P3 rules out option 2. It does not give the choice among forms. |

## RV12-1

UNEV-P2, UNEV-2, the decltype Syntax list, the glossary entry, COMMA-1, LOOP-3 with its Note, and the two new constant tests.

The reference names three places where an expression is evaluated only for its effects: an expression statement, the left operand of the comma operator, and the operand of an explicit conversion to `void` (in any of the cast notations). The reference defines the `for` loop by a rewrite into a block with a `while` loop. In that rewrite the iteration expression becomes an expression statement at the end of the inner body. So the iteration expression is a discarded-value expression too, and its full expression is the iteration expression alone. The new list item, LOOP-3's new sentence, and the glossary's four places agree.

In a discarded-value expression, the reference turns a prvalue of any object type into a temporary. The temporary ends with the full expression, as every temporary does. The UNEV-P2 bullet and UNEV-2 name the class case only. For a prvalue of scalar type the difference has no visible effect, so the text agrees. Both constant tests follow from these rules. In the cast-to-void test, both temporaries live until the end of the whole comma expression and end in reverse order. In the loop test, the temporary of the iteration expression ends before the condition runs again, and a temporary of a comma's left operand inside the iteration expression lives to the end of that expression.

Two doubts remain:

- The comma item. The reference makes the left operand a discarded value only for the built-in comma operator. With an overloaded comma, the left operand is an argument of a call, and nothing discards it. COMMA-1 sits on a page where COMMA-7 covers the overloaded case, so COMMA-1 reads as the built-in case. The Syntax list and the glossary entry name the comma with no such limit.
- The volatile read. The reference reads a volatile object in a discarded value only when the expression has one of a closed set of forms: a name, a subscript, a member access, an indirection, a pointer-to-member access, any of these in parentheses, a conditional whose second and third operands both have such a form, or a comma whose right operand has such a form. UNEV-2 lists the first four forms and then says that a volatile operand of an operator is read too. That sentence is looser than the reference. For a conditional or a comma, the reference reads the volatile result because of the form rule. UNEV-2's sentence would also cover an operand that the operator itself reads, which every evaluation reads anyway. The gap is older than F8. The 2026-10-31 review of expressions and declarations already noted the forms.

## RV12-2

UNEV-3, COMMA-5, STMT-4, the decltype Examples block, the Evidence table, and the four warning tests.

The reference does not recommend a warning for a discarded value with no effect. Its only recommendation for a discarded value covers a call to a function or a constructor marked `nodiscard`, or a call that returns a type so marked, unless the program casts the result to `void`. ATTR-5 states that separately. UNEV-3 counts every call as an effect, and that does not contradict ATTR-5, since ATTR-5 is its own recommendation. So UNEV-3, COMMA-5, and STMT-4 have no counterpart in the reference.

The reference's notion of a side effect is narrower than UNEV-3's list. A call counts there only if the called function does something with a side effect. UNEV-3 is a recommendation, and its list does not have to match.

Each program in the four warning tests is valid in the reference. The three reject tests check the recommended warning with warnings turned into errors, which is Tidal's own practice for a recommended rule. `discarded-effects-no-warning.cpp` is a valid program. Each line in the Examples block is a valid statement, and the comments follow UNEV-3.

## RV12-3

PRIM-28, the PRIM Example, and `fold-comma-overloaded-class.cpp`.

The reference expands a fold into a nest of binary expressions with the fold's operator, grouped to the right for a right fold and to the left for a left fold. Each binary expression is then an ordinary use of the operator, so an operand of class or enumeration type finds its operator function in the usual way. PRIM-28's new sentence agrees. In the example, the left fold over three elements pairs the first two and then pairs that result with the third, as the comment says. In the test, the right fold pairs the last two first. Both expected values follow.

## RV12-4

TPL-4, the TPL Note, and the two friend tests.

In the reference, a declaration in a class's member list may carry only one template head. A second head is not valid there. The reference makes one exception for a friend. A friend template may name a member of a dependent specialization. The qualifier must end in a template-id that names a class template, and the friend's own template parameters must be deducible from that template-id. A member of each specialization whose arguments the template-id matches is then a friend. When the member is itself a template, the friend carries a second head for that member. So TPL-4's new sentences agree in the common case, and both tests agree. In the reject test the qualifier uses no parameter of the first head, so nothing can be deduced and the friend is not valid.

The doubt is TPL-4's second condition. The reference asks only that the parameters can be deduced from the template-id, so the template-id may give a pattern of the parameters, such as a pointer to the parameter. The friend then covers the specializations that match the pattern. TPL-4 asks for the parameters themselves as the arguments, which leaves out these patterns. The reference also requires the qualifier to end in the template-id, so a member of a class nested inside the specialization cannot be named this way. TPL-4's first condition fits that limit. The reference forbids a friend template in a local class, which TPL-4 does not repeat here.

## RV12-5

The casts paragraph, its Note, the Divergences lines for Q-0240, Q-0243, and Q-0226, and the two new cast tests.

In the reference, a type specifier made of a template name with no argument list, after an optional qualifier and an optional `typename`, stands for a class type whose arguments are deduced. The template must be a class template or an alias template whose defining type is a template-id of a certain simple shape. Such a placeholder may be the type of a functional cast. So the paragraph's alias template case and `functional-cast-alias-template-deduced.cpp` agree, and Clang 18's rejection (Q-0240) does not conform.

The reference also requires the qualifier of such a placeholder to be independent. A dependent qualifier before a template name with no argument list does not make a deduced type. So the new sentence on a dependent qualifier, the Q-0243 line's answer for the cast form, and `functional-cast-dependent-qualifier-deduced.cpp` read against the reference, as the read of the RV10 repair diff said. Every compiler accepts the form. `pending.md` row 4a holds the case. The text describes the split correctly.

The Q-0226 line says that `template` can come before a class template name with no argument list in a typename form. As RV10-10 found, the reference's typename form takes the keyword only before a template-id, so the line reads against the reference. Only EDG accepts the form.

## RV12-6

ATTR-2's item, ATTR-19's new row, the Evidence lines, and the two new reject tests.

In the reference, an attribute list at the start of a using-directive applies to the directive. A standard attribute that applies to an entity outside its list makes the program not valid. The list for `deprecated` names classes, aliases, variables, data members, functions, namespaces, enumerations, enumerators, and specializations. The list for `nodiscard` names functions, classes, and enumerations. A directive is on neither list. So the item, the row, and both tests agree, and Clang's and GCC's acceptance is a missing diagnostic. The reference's using-declaration has no place for a leading attribute list, so the Evidence line on the using-declaration agrees.

## RV12-7

The AUTO Note and `decltype-auto-before-scope.cpp`.

In the reference, a qualifier may start with `decltype` of an expression. The placeholder form is a separate specifier and may not appear there. So the test's program is not valid and the Note agrees.

## RV12-8

UNI-13's new sentence, the UNI Note, the Q-0455 line, and `chain-assign-deleted-default-constructor-constant.cpp`.

In the reference, an assignment can activate a union member on the left side's chain if the member is a scalar, an array of scalars, or a class (or an array of such a class) whose default constructor is trivial and not deleted. A default constructor deleted at its first declaration is not user-provided, so it can be trivial. The reference still excludes it by the second condition. So the member in the test does not join the set, the assignment creates no object, and the write reaches a member that is not active. In constant evaluation that is not a constant expression. GCC's rejection conforms, and the test's expected acceptance reads against the reference. Q-0455 holds the split and states the review's reading correctly.

UNI-13's first bullet and OBJ-2's item both say only that the default constructor must be trivial. Each omits the condition that it is not deleted.

## RV12-9

OBJ-2's new clause, the OBJ Note, and `write-after-pseudo-destructor-union-struct-member-constant.cpp`.

The reference collects only union members along the left side's chain. In the test, the last member of the chain is a member of a struct, so it does not join the set. The struct member of the union does join, since its class has a trivial default constructor. The struct member is still alive, because ending one of its subobjects does not end the struct. A plain reading therefore creates no new object, and the write reaches an `int` whose lifetime has ended. That use is undefined, and constant evaluation rejects it. OBJ-2's clause and the test agree with that reading.

The doubt is the same as RV10-12. The reference does not say whether a change to a live object counts as outside its lifetime when the change reaches one of its ended subobjects. If it does, the assignment would create a new struct there, and the write would be valid. GCC and EDG accept the program, which fits that reading. Q-0447 holds both forms.

## RV12-10

TARG-16's new sentence, the arguments Syntax paragraph, the Q-0454 line, and the two new tests.

The reference's template argument is a constant expression, a type, or a name. It has no braced form, so a braced list for a scalar parameter reads against the reference, as RV10-6 found for a class parameter. `braced-scalar-arguments.cpp` expects acceptance and reads against. Every compiler accepts it.

`braced-integer-for-floating.cpp` expects a rejection. The reference rejects it because the braced form is not valid at all. Under Tidal's own reading, the value converts as an unbraced argument would. The reference's conversion for a constant template argument does not allow a change from an integer type to a floating-point type, so that reading also rejects. Q-0454's answer holds either way, and Clang's acceptance does not conform.

## RV12-11

`union_def`, UNI-17, the new UNI-P1 sentence, TSP-32, and the three new union tests.

In the reference, a union is a class whose key is `union`, and its head takes the same name forms as any class head: an optional qualifier and then an identifier or a template-id. A `final` marker may follow a name. Only a head with a name can take it. The reference's class head also has a place for a base list. A semantic rule then forbids bases for a union, so `union_def` derives the same valid programs. Attributes come right after the key in both grammars.

A union may be a member template defined outside its class. A member union of a class template may have an explicit specialization. A union template may have partial and explicit specializations. The key of a specialization must agree with the primary template's, and `union` agrees only with `union`. So UNI-17, TSP-32, and the three tests agree.

## RV12-12

LIST-4, its new example, and the three new list tests.

In the reference, every implicit conversion counts when a value initializes an element of an aggregate. If the value can initialize the element, it does. Otherwise, if the element is itself an aggregate or an array, the braces are taken as omitted, and the value goes to the element's first member. An aggregate member has no constructors of its own beyond copy and move. So the values that can initialize such a member are these:

- an object of its type or of a derived class,
- an object of a class that converts to it,
- a string literal for a character array.

The list in LIST-4 matches. A value that converts both to the member and to the member's first element initializes the whole member, as the new sentence and the example say. In the too-many test, each value initializes a whole member, so the third has no member. In the string literal test, the literal cannot initialize the outer struct member. The braces of that member are taken as omitted, and the literal initializes its character array. Each expected result follows.

## RV12-13

ICS-16.

For a parameter of the library's initializer list type, the reference forms a sequence when every element converts to the element type. The sequence is the worst element conversion, or the identity for an empty list. It allows a user-defined conversion for an element even when the call is to a constructor that takes the initializer list. A designated list converts only to an aggregate, and the initializer list type is not one, so it has no sequence. The first paragraph agrees.

The reference does not say which user-defined sequence a list holds when the worst kind is user-defined. It compares two user-defined sequences only when they use the same conversion function, and then by their second standard parts. So the second paragraph makes a choice that the reference leaves open. Q-0419 and Q-0423 hold that choice.

## RV12-14

The eleven rule tests of commit 3bb416e8.

- The two closure tests. A constructor template that is not explicit is a converting constructor. A closure argument converts through it by one user-defined conversion, and a lambda with no captures converts to a function pointer. Both programs are valid.
- The four generic lambda tests. A default capture by reference captures each local that the body uses: a closure parameter, a pointer to member function, and a class object. A captured object of class type may be called. A later change to the captured local shows through the reference. Constant evaluation follows the same rules, since every object involved begins its lifetime in the evaluation. All four agree.
- `static-constexpr-member-calls-own-member.cpp`. In the reference, a member function body is one of the places where the class counts as complete, and the initializer of a static data member is not. So the body in effect comes after the whole class, and at the initializer the function has no definition yet. A call of a function with no definition is not a constant expression, and a `constexpr` variable needs one. The test agrees, and every compiler rejects. The doubt is that the reference reaches this result only through the rule on complete-class places. It does not say in words that the function counts as undefined at that point.
- `class-static-constexpr-member-calls-own-member.cpp`. When a specialization of a class template is made implicitly, its static data members get their declarations and not their initializers. The initializer is made at the member's first use, after the class is complete. The test agrees.
- `explicit-specialization-body-sees-namespace.cpp`. An explicit specialization may be declared with a qualified name in an enclosing namespace. Names used after a qualified class name are looked up in the class and then in the namespace that the qualifier names. The test agrees.
- The two tests on too many braced values. The braces of a last member that is an aggregate or an array are taken as omitted, so one value fills it, and a further value has no element. That error is in the immediate context of the requirement, so the requirement is false. In the `if constexpr` test, the discarded branch of the template is not instantiated, so its structured binding with the wrong count does no harm. Both agree.
- `constant-parameter-class-template-string-literal.cpp`. A class template name with no argument list may be the type of a constant template parameter, and the argument deduces it. The class has public members of structural type, so it is a valid parameter type. The object copies the characters and keeps no pointer into the literal. The test agrees.

## RV12-15

The four questions that wait for a use, each read with its principle.

- Q-0424 with UNI-P3. The reference makes the anonymous union an unnamed type with one unnamed object. It allows only public data members and static assertions inside it, and it forbids nested types, nested anonymous unions, and functions there. A member whose declaration defines an unnamed class declares a nested type. UNI-P3 now says each of these points, so it rules out option 2 and gives option 1. The answer agrees with the reference, and the doubt of RV7-A-3 is closed.
- Q-0427 with DEP-P5. The name after a dot or an arrow is looked up in the class of the object. When that class is the current instantiation, the lookup runs at the definition and also searches the bases that are not dependent. If it finds a template, the following `<` opens an argument list. When the class is dependent in any other way, no lookup runs at the definition, and the `<` is the less-than operator. DEP-P5's two rows for the member access say exactly this. The row for the current instantiation gives a template-id, so DEP-P5 gives option 1, and the answer agrees with the reference.
- Q-0432 with DEP-P2. The reference makes `sizeof` of an expression or a type, `alignof`, `noexcept`, and `typeid` value-dependent only when the operand is type-dependent or the type is dependent. DEP-P2 says that a value-dependent operand does not make `sizeof`, `alignof`, or `noexcept` value-dependent, so it rules out option 2 and gives option 1, as the reference does. The doubt is DEP-P2's reason, that only the type of the operand decides the value. For `noexcept`, the value depends on which operations the operand performs and whether each of them can throw. Those are fixed once the operand is not type-dependent. DEP-P2 also does not name `typeid`, which DEP-4 now names.
- Q-0435 with OPR-P3. The reference lets an integer or floating literal with a suffix call a literal operator template whose head is one pack of `char`. The source characters before the suffix become the arguments. A string literal calls only the form with one constant parameter of class type. OPR-P3 says this, so it rules out option 2 and answers which literals call the template. OPR-P3 does not give the rest of option 1. Those parts are the choice of the numeric form when it exists, the call that is not valid when the raw form and the template both exist, and the rejection of other heads. The reference decides all three as option 1 says, and OPR-8, OPR-10, and OPR-16 state them. So the principle answers the question's first part, and the rules answer the second.

## Part B: points kept against with no question

- RV10-6 on TARG-16: yes. A whole grammar arm and several tests rest on the point, and Q-0454 already depends on it, so a question with options would let Q-0454 cite its premise. The form may also be the subject of a later change to the language, which `pending.md` row 7 keeps out of scope.
- RV9-7 on LSUP-10 and UNI-14: no new question. Every compiler accepts, no test carries an override, and the row 4a item records the split. Q-0425 already holds the narrower forms that rest on the same allowance, so it is the natural place for a line if one is wanted.
- RV8-B-5 on CTAD-17: yes. Clang 18 differs from the newer compilers, the review reads the reference as Clang 18 does, and Q-0020 holds Clang 18's override as a missing feature. That record does not fit a compiler whose answer the review reads as conforming.
- RV8-B-6 on CTAD-16: no. GCC and EDG agree on the separating test, Clang has no inherited guides (Q-0239), and no test carries an override for this point. The row 4a item is enough. Q-0408 sits next to it if the maintainer wants the cases together.
- RV6-A-4 and RV7-B-4 on UNW-3: no. UNW-3 allows both forms, so no test outcome depends on the point, and the row 4a item records it.
- RV6-A-12 on LSUP-11: no. Every compiler agrees, no test carries an override for the element type that is not a literal type, and the row 4a item records the split. Q-0421 holds the other LSUP-11 split.
- RV6-B-2 on DEP-15 and DEP-P4: no. Every compiler agrees, so no override needs a question to cite, and the row 4a item records the reading.
