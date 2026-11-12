# Gap review: phase 17 text that no source reader has read (RV10)

Date: 2026-11-11

Role: source reader

Reference: the C++23 working draft sources.

Commit read: e3cb5a3c3c4327f284ff8cd63afd5f2cd6b95d9d.

The review is item RV10 of decision 0065. It covers three groups of text:

- the grammar text of item GR (commit 1c223631): each new or changed production in the Syntax segments, the paragraphs beside them, and the rule text that GR changed (ATTR-19's lambda row, IF-7, DTY-3),
- the text of item F7 (commit b511838d): OBJ-1, OBJ-2, OBJ-3, JMP-6, JMP-7, JMP-9, JMP-14, LOOP-3, RFOR-4, FN-7, FN-12, INST-1, INST-30, CE-3, CE-22, ATTR-2, the principles JMP-P1, JMP-P2, and ATTR-P1, and the glossary entry for label,
- first readings of Q-0450, Q-0451, and Q-0452.

The read of the RV9 repair diff used points of the form RV10-R-n. This review does not repeat those points. New points carry IDs of the form RV10-n.

## Result

Most of the text agrees with the reference.

Three items read against the reference:

- `exception_spec` and the `function_suffix` that uses it. The reference has only the two `noexcept` forms. The empty `throw()` form is gone from it. This is the known split of Q-0077, and EXS-5 records it (RV10-2).
- The braced list in `template_arg`. The reference's template argument is a type, a constant expression, or a name. It has no braced list. Every compiler accepts the form, and TARG-16 and two tests rest on it (RV10-6).
- The `template` keyword before a plain identifier in `typename_specifier`. The reference's typename form takes the keyword only before a template-id (RV10-10).

JMP-14's new sentence on when the implementation checks the labels also reads against the reference, and so does Q-0451's answer. The reference makes a template that no specialization uses, and that holds a fault which does not depend on the parameters, not valid with no diagnostic required. So GCC's acceptance conforms there (RV10-14).

Several productions agree with a doubt. The reference derives a few forms that the Tidal grammar does not: the `template` keyword before the last template-id of a qualified base or cast type (RV10-1, RV10-4), attributes before a using directive (RV10-3), a template-id in a `using enum` declaration (RV10-3), an attribute after `*` and a member pointer in a `new` type (RV10-5), and attributes before a structured binding in an initializer (RV10-8). The Tidal grammar derives `decltype(auto)` in four places where the reference has only the expression form of `decltype` (RV10-1).

The F7 text agrees, apart from JMP-14. OBJ-2's new clause agrees with a doubt on the array element (RV10-12). CE-3 agrees with a doubt on the parts of a `mutable` member (RV10-17). INST-30 agrees with the doubts of RV9-2 and Q-0406 (RV10-16). FN-7's warning has no counterpart in the reference.

Q-0450 and Q-0452 are supported. Q-0451 reads against.

## Grammar text of item GR (commit 1c223631)

The column "Reference" says whether the reference derives the same forms as the production, more forms, or fewer.

| Item | Reference | Verdict | Note |
|---|---|---|---|
| `base_type` | about the same | agrees with a doubt | RV10-1. |
| `base_spec` | same | agrees | RV10-1. The three orders of access and `virtual` and the leading attributes match. |
| `mem_init_id` | about the same | agrees with a doubt | RV10-1. |
| `dtor_call` and `unqualified_id` after `~` | about the same | agrees with a doubt | RV10-1. |
| `exception_spec` and `function_suffix` | fewer | reads against | RV10-2. Q-0077. |
| `using_directive` | more | agrees with a doubt | RV10-3. |
| `namespace_alias` | same | agrees | RV10-3. |
| `namespace_def` and the paragraph after it | fewer, with NS-11 | agrees | RV10-3. |
| `using_enum_decl` and its paragraph | more | agrees with a doubt | RV10-3. |
| `cast_type` and the casts paragraph | about the same | agrees with a doubt | RV10-4. |
| `new_type` and `new_bounds` | more | agrees with a doubt | RV10-5. |
| `decl_specifiers` | same | agrees | RV10-5. |
| `parameter` | same, with CLS-14 | agrees | RV10-5. |
| `template_id` and its paragraph | same | agrees | RV10-6. |
| `template_arg` | fewer | reads against | RV10-6. |
| `member_decl`, the `using_enum_decl` and `enum_opaque` arms | same | agrees | RV10-7. |
| `member_decl`, the `template_head` arm, and the paragraphs in templates/declarations.md and class-template-deduction.md | same, with TPL-4 | agrees | RV10-7. |
| `init_stmt` | more | agrees with a doubt | RV10-8. |
| `static_assert_decl` | same | agrees | RV10-9. |
| `conversion_type` | same apart from the order in `ptr_part` | agrees with a doubt | RV10-9. Q-0123. |
| `typename_specifier` and its paragraph | fewer | reads against | RV10-10. |
| `template_template_param` default | same | agrees with a doubt | RV10-10. |
| `lambda_expr` and `lambda_template` | same | agrees | RV10-11. |
| ATTR-19, the lambda row | | agrees | RV10-11. |
| IF-7, the structured binding item | | agrees | RV10-8. |
| DTY-3, the member initializer item | | agrees | RV10-1. |

## Text of item F7 (commit b511838d)

| Item | Verdict | Note |
|---|---|---|
| OBJ-1, the array row | agrees | RV10-12. The condition of RV9-11 is now met. |
| OBJ-2, the union member clause | agrees with a doubt | RV10-12. |
| OBJ-3, the statement scope | agrees | RV10-13. |
| JMP-6 | agrees | RV10-13. |
| JMP-7 | agrees with a doubt | RV10-14. The reference asks for no diagnostic in a template that no specialization uses. |
| JMP-9 | agrees | RV10-13. |
| JMP-14, the local class | agrees | RV10-14. |
| JMP-14, the check at the definition | reads against | RV10-14. |
| LOOP-3 | agrees | RV10-13. |
| RFOR-4 | agrees | RV10-13. |
| FN-7 | reference silent | RV10-15. The clause on a `break` in a nested statement agrees. |
| FN-12 | agrees | RV10-16. |
| INST-1 | agrees | RV10-16. RV9-2 stands. |
| INST-30, the `noexcept` item | agrees with a doubt | RV10-16. |
| CE-3, the `mutable` sentence | agrees with a doubt | RV10-17. |
| CE-22, the sentence on a member that is not active | agrees | RV10-17. |
| ATTR-2, `maybe_unused` on a type | agrees | RV10-18. |
| ATTR-P1, the label row | agrees | RV10-19. |
| JMP-P1 | agrees | RV10-19. |
| JMP-P2 | agrees | RV10-19. |
| glossary, label | agrees | RV10-19. |

## Questions

| Question | Verdict | Reason |
|---|---|---|
| Q-0450 | supports | RV10-1. A base named by `decltype` can start a member initializer, and any member initializer takes either parentheses or a braced list. |
| Q-0451 | reads against | RV10-14. The program is not valid in each form, but for the three forms with no specialization, the reference requires no diagnostic. GCC conforms there. The discarded branch of a used template is less clear. |
| Q-0452 | supports | RV10-13. Constant evaluation follows the same order of destruction as a run. |

## RV10-1

`base_type`, `base_spec`, `mem_init_id`, `dtor_call`, the `~` arm of `unqualified_id`, DTY-3's new item, and Q-0450.

In the reference, a base in a base list and the name of a base in a member initializer use one shared form. That form is a type name with an optional qualifier, a qualifier followed by the `template` keyword and a template-id, or a `decltype` of an expression. A type name there covers a class name, a template-id, an alias, and an enumeration name, and semantic rules then require a class. The leading attributes, `virtual`, and the access specifier in a base come in the same three orders that `base_spec` gives. So `base_type` and `base_spec` derive the same forms as the reference, apart from two details:

- The reference lets the `template` keyword come right before the last template-id of a qualified base name. `nested_name` takes the keyword only before a template-id that a `::` follows. So `base_type` cannot place the keyword before the final template-id. A base that names a member template of a dependent type with the keyword is a reference form that Tidal does not derive. The reference does not need the keyword in that place, since the name there can only be a type, so the gap is small.
- `decltype_spec` includes `decltype(auto)`. In the reference, the `decltype` form that may name a base, follow a `~`, or start a cast type is only the form with an expression. The placeholder form is separate, and the reference makes a placeholder not valid in any place that it does not list. So `base_type`, `mem_init_id`, `dtor_call`, the `~` arm of `unqualified_id`, and `cast_type` derive one form that the reference does not. No Tidal rule that I found rejects `decltype(auto)` in these places. AUTO-11 limits its use in a declaration only.

`template_id` now also covers the operator forms, so `base_type` and the destructor arms can derive an operator template-id. No such name denotes a class, so lookup rejects it, and this widening is harmless.

For a destructor name, the reference allows a type name or a `decltype` of an expression after the tilde, with an optional qualifier before. The new arms match, apart from the `decltype(auto)` point.

A member initializer in the reference takes either parentheses or a braced list after any of its name forms, the `decltype` form included. So DTY-3's new item agrees, `member-initializer-base-braces.cpp` agrees, and GCC's rejection of the braced form reads against the reference. Q-0450 is supported.

## RV10-2

`exception_spec`, the `function_suffix` that now uses it, and the paragraph that cites EXS-5.

The reference's exception specification has only the two `noexcept` forms. The empty `throw()` form was removed in an earlier revision, and the reference lists that removal among its changes. So the production derives one form more than the reference, and a program with `throw()` is not valid. A compiler that accepts it needs to give a diagnostic. This is the known split of Q-0077, and the review of 2026-11-03 gave the same reading. The production follows EXS-5 and the paragraph says so, so the text is consistent with Tidal's own choice.

## RV10-3

`using_directive`, `namespace_alias`, `namespace_def`, the paragraph on attributes after `namespace`, and `using_enum_decl` with its paragraph.

- `using_directive`. The reference allows an optional qualifier and then a namespace name, which is an identifier. The new production matches. The reference also lets an attribute specifier come before `using` in a using directive, and the attribute then belongs to the directive. `using_directive` has no such place, and no other production of `block_decl` supplies one. No standard attribute applies to a directive, so the gap affects only an attribute that the implementation ignores or one it must reject.
- `namespace_alias`. The reference's target is an optional qualifier and a namespace name. The new production matches.
- `namespace_def`. The reference has three definition forms. A named and an unnamed definition each take an optional `inline` before `namespace` and optional attributes after it. A nested definition takes no attributes. Its first name cannot carry `inline`, and each later name can. The first arm of `namespace_def` matches the named and unnamed forms. The second arm takes `ns_path`, which allows `inline` before the first name, so the grammar derives a form that the reference does not. NS-11 rejects that form, so the grammar and the rule together agree. The paragraph on attributes agrees.
- `using_enum_decl`. The reference allows the declaration at namespace scope, in a block, and in a class, as the paragraph says. Its name is an optional qualifier and then an identifier or a template-id. The template-id arm lets an alias template name an enumeration. `using_enum_decl` takes only the identifier, so it derives fewer forms. The reference also requires the named type to be a reachable enumeration definition, and the name must not be dependent. ENUM-9 does not say the second part, but that is outside GR.

## RV10-4

`cast_type` and the new paragraph on the casts page.

In the reference, the functional form of a cast takes a simple type specifier or a typename specifier, with parentheses or a braced list. A simple type specifier there covers a type name with an optional qualifier, a qualifier with the `template` keyword and a template-id, a `decltype` of an expression, `auto`, a class template name for deduction, and the single keywords for the built-in types. The reference allows only `auto` as a placeholder in a cast, with no constraint before it. So `cast_type` matches, with two details:

- the `template` keyword before the last template-id after a qualifier, as in RV10-1, which `cast_type` cannot place,
- `decltype(auto)`, which `decltype_spec` derives and the reference does not allow here (RV10-1).

The paragraph says that the tokens are a cast if lookup finds a type, and a call otherwise. A class template name with no argument list also makes a cast in the reference, by deduction of the arguments, and that name denotes a template. The paragraph does not cover that case in words. The rest of the paragraph and the DEP-19 citation agree.

## RV10-5

`new_type`, `new_bounds`, ATTR-19's `new` row, `decl_specifiers`, and `parameter`.

- `new_type` and `new_bounds`. In the reference, attributes may follow the last type specifier of the type in a `new` expression and apply to that type. Attributes may follow each bound and apply to the array type of that bound. The new arms match. The reference's `new` type also takes the full pointer forms of a declarator: an attribute after each `*` and a member pointer with a qualifier. `new_type` takes only `*` with cv-qualifiers. So it derives fewer forms. The gap was in the production before GR.
- `decl_specifiers`. The reference allows attributes only after the last specifier of the sequence, and they apply to the type. The new arm matches.
- `parameter`. The reference's parameter takes optional attributes, then optional `this`, then the specifiers and the declarator. The reference has no default argument after an explicit object parameter. The production allows both together, and CLS-14 rejects the pair. The new attribute arm agrees.

## RV10-6

`template_id`, `template_arg`, and the paragraph on the arguments page.

The reference's template-id has the same three arms as the new `template_id`: a template name, an operator function name, or a literal operator name, each with an argument list. So the production and the paragraph's example agree.

The reference's template argument is a constant expression, a type, or an id-expression. It has no braced list. So the `braced_init` arm of `template_arg` derives a form that the reference does not, and the paragraph's sentence that cites TARG-16 reads against the reference. A class value as an argument needs a written type in the reference, as in a functional cast with braces, and that form is a constant expression. TARG-16's words "also from a braced list", `class-arguments.cpp`, and `same-specialization-class.cpp` rest on the same arm. Every compiler accepts those two tests, so the split is between the reference and every compiler, and no question records it.

## RV10-7

`member_decl`: the new arms for `using_enum_decl` and `enum_opaque`, and the `template_head` arm in place of `template_decl`, with the paragraphs on the template declarations page and the deduction guide page.

The reference's member declaration includes a `using enum` declaration and an opaque enumeration declaration, so the first two arms agree.

The reference's member declaration includes a template declaration, and its template declaration takes any declaration after the template head. A semantic rule then limits what that declaration may declare. In a class, the effect is a template head before a member declaration, as GR now writes it. The new arm also derives a template head before an access label, a `static_assert`, or a second template head. TPL-4 rejects the first two, since each declares no named entity. A second template head inside a class is not valid in the reference either, and I found no Tidal rule that names it. The paragraph's sentence on a member with no specifiers, for a constructor template or a conversion function template, agrees. A deduction guide in a class with a template head is a member template, as the new sentence says.

## RV10-8

`init_stmt`, IF-7's new item, and the three new tests.

The reference's initializer of `if`, `switch`, and `for` is an expression statement, a simple declaration, or an alias declaration. The empty statement is an expression statement with no expression. The reference's simple declaration includes the structured binding form. So `init_stmt` with `sb_decl` agrees, and IF-7's item and the tests for `if`, `switch`, and `for` agree.

The reference lets attribute specifiers come before a structured binding declaration. `sb_decl` has no such place, so it derives fewer forms in an initializer and in a block. The gap was in `sb_decl` before GR.

## RV10-9

`static_assert_decl` and `conversion_type`.

- `static_assert_decl`. The reference's message is one string literal. Adjacent string literals are joined into one literal before the grammar applies, so a list of adjacent literals is one message. The new arm derives the same programs.
- `conversion_type`. The reference's target type of a conversion function is a type specifier sequence and then any number of pointer, reference, and member pointer parts, each with its own attributes. `ptr_part` gives the same parts, so the change agrees. One detail stays. The reference places the attribute after `*` and before the cv-qualifiers. `ptr_part` places it after the cv-qualifiers. This is the known split of Q-0123, and `conversion_type` now inherits it.

## RV10-10

`typename_specifier`, the paragraph after it, and the default of `template_template_param`.

The reference's typename form is `typename`, a qualifier, and then either an identifier or an optional `template` keyword and a template-id. It has no `template` keyword before a plain identifier. The reference does allow the keyword before a qualified name of a class template or alias template with no argument list, and marks that use deprecated. That use applies where a production lets the keyword come before a plain name, such as a qualified name used as a template template argument. The typename form has no such place. So the new arm derives a form that the reference does not, and the paragraph's sentence that ties the form to DEP-26 reads against the grammar. The doubt is that the reference's rule on the keyword names class templates in general and does not exclude the typename form in words. The paragraph's first sentence, which lets a typename specifier be the type of a functional cast, agrees (RV10-4).

The reference's default of a template template parameter is an id-expression, which may be a qualified name with the `template` keyword before its last part. So the new optional keyword agrees. The doubt is that the reference marks the keyword before a template name with no argument list deprecated. The production does not say so.

## RV10-11

`lambda_expr`, `lambda_template`, and ATTR-19's new row.

The reference lets attribute specifiers come right after the captures, or after the template head and its optional requires clause. Those attributes belong to the function call operator or the operator template. The rest of the lambda then follows, and the parameter list may be absent. So the move of the attributes out of `lambda_template` and into `lambda_expr` derives the same forms, also with no template head and no parameter list, as the new test holds. The row's position and its last column agree. The reference resolves a clash between attributes at the end of a requires clause and the lambda's attributes in favor of the lambda. Tidal's `requires_clause` does not end in attributes, so the clash does not arise.

## RV10-12

OBJ-1's array row and OBJ-2's new clause on a union member, with the two new tests.

OBJ-1. In the reference, an array's lifetime begins when it has storage and its initialization is complete. An array with no initializer whose element type is not a class, or is a class with a trivial default constructor, is initialized by default and that initialization does nothing. So such an array begins with its storage. The row now says that the array begins after its last element when it has an initializer, or when the element type is a class whose default constructor is not trivial. That matches. The end column names destruction. The condition of RV9-11 is now met.

OBJ-2. In the reference, an assignment whose left side reaches a union member through member access can start that member's lifetime. The reference follows the chain of member access and of built-in subscripts of arrays on the left side, and collects each union member on the chain whose type is a scalar, an array of scalars, or a class (or an array of such a class) whose default constructor is trivial and usable. For each such member, if a change to it would be outside its lifetime, the assignment first creates a new object of the member's type there, with no initialization. That new object makes the member active.

- For a scalar member that a pseudo-destructor call ended, the member is on the chain and its lifetime has ended. So the assignment creates a new scalar and is valid, as the new clause says. `write-after-pseudo-destructor-union-member-constant.cpp` agrees.
- For an element of an array member, the subscript passes the chain to the array member itself. The array member is still alive when only one element has ended. So a plain reading creates no object, the write reaches an ended element, and the clause's last sentence agrees.

The doubt is the array element. The reference does not say whether a change to an array counts as outside its lifetime when the change reaches an ended element. If it does, the assignment would create a new array of the member's type, with no element values, and then the write would be valid. GCC and EDG accept the test, which fits that reading. Clang follows the plain reading, as the test expects.

The clause names only a scalar member. The reference also covers a member of class type with a trivial default constructor and a member reached through an outer member, which do not arise in these tests.

## RV10-13

OBJ-3, JMP-6, JMP-9, LOOP-3, RFOR-4, the new run tests, and Q-0452.

The reference models each variable with automatic storage as active from its declaration to the end of its scope. On any transfer of control inside a function, it destroys the variables that are active at the source and not at the target, in the reverse of their construction. Then it initializes the variables that are active at the target and not at the source. A return goes to a point after the whole body. The variables of an `if`, a `switch`, or a loop initializer, and of a condition, have a scope that holds the body. The hidden range variable of a range-based `for` lives in a scope that holds the loop. So:

- OBJ-3's new sentences agree. The body's variables end first, then the condition variable, then the initializer variable. An exception follows the same order through unwinding.
- JMP-9's new sentence agrees.
- LOOP-3 agrees. The reference rewrites the `for` loop so that a condition variable lives through the iteration expression, and the initializer variable lives until the statement ends by any route.
- RFOR-4 agrees. A temporary in the range expression lives as long as the hidden reference that binds the range, and that reference ends after the body's objects and the iterator variables.
- JMP-6 agrees. The reference's rule on initialization applies only to a variable that is active at the target and not at the source. It treats the move from a `switch` condition to a `case` label as a jump. A `break`, a `return`, and a `continue` never reach a point where a new variable is active. The reference places the target of `continue` after the end of an inner block that holds the body, so the body's later declarations are not active there. So the new sentences in JMP-6 and the JMP-P2 row agree, and `continue-before-later-declaration.cpp` agrees.

The reference gives constant evaluation the same semantics as a run, so the order of destruction holds there too. A skipped destructor or a changed order gives a different log, and the evaluation then reaches a different result. Q-0452 is supported, and GCC 16 and EDG read against.

## RV10-14

JMP-7, JMP-14, the DEP-30 Note, the new reject tests, and Q-0451.

- The reference requires `break` and `continue` to sit inside a loop or a `switch` (a loop for `continue`) through a chain of statements. That chain does not pass into a lambda body or into the body of a member function of a local class, since those bodies sit inside an expression or a declaration. Each `goto` must name a label of the current function, and no two labels in a function may share a name. A member function of a local class is its own function. So the new local class wording in JMP-7, JMP-14, and JMP-P1 agrees, and so do the four new tests on the local class.
- The reference allows an implementation to check a template before any instantiation. It also makes a program not valid, with no diagnostic required, when a construct that does not depend on the parameters would make an instantiation right after the definition fail, or when no valid specialization exists for a template that is never instantiated. A `goto` to a missing label, a label name used twice, and a `break` or `continue` outside a loop do not depend on the parameters. So for a function template that no specialization uses, a member function of a class template that is never used, and a generic lambda that is never called, each program is not valid, and the reference does not require a diagnostic.

So JMP-14's sentence that the implementation checks its conditions at the definition, also in a template that no specialization uses, asks for more than the reference. The three tests for those forms expect a rejection, which the reference allows. GCC's acceptance also conforms, so the GCC overrides do not mark a defect. DEP-30 is Tidal's form of the same reference rule. The DEP-30 Note says that a jump condition is not on DEP-30's list, but the reference treats it like any other construct that does not depend on the parameters.

The discarded branch of a used template is less clear. The template is instantiated, so the clause on a template that is never instantiated does not apply. The branch itself is never instantiated. The reference does not say in words whether the label rule binds a statement that is discarded in every specialization. Outside a template, the discarded branch is checked, and every compiler rejects.

JMP-7's new sentence makes the same claim for `break` and `continue`. Every compiler rejects those forms, so the tests agree. The doubt is that the reference does not require the diagnostic.

Q-0451 reads against for the three forms with no specialization. Option 1's outcome is allowed, but the reference does not make GCC's acceptance a fault. The reference gives no clear answer for the discarded branch of a used template.

## RV10-15

FN-7, the new clause on `break` and `goto`, and the two new warning tests.

The reference makes control that flows off the end of a function with a return value undefined, apart from `main` and coroutines. It asks for no warning, so FN-7's recommendation has no counterpart there. The reference ends the smallest loop or `switch` that holds a `break`, so a `break` in a nested statement leaves only that statement, as the new clause says. A `goto` to a label outside the loop leaves it. The clause's account of which paths reach the end agrees.

## RV10-16

FN-12, INST-1's list item, INST-30's new item, and the new tests.

In the reference, a parameter type or the return type of a function definition must not be an incomplete class within the function body, unless the definition is deleted. A function body inside a class's member list is a place where the class counts as complete. A nested class defined inside its enclosing class shares those places with the enclosing class. A nested class defined later, outside the enclosing class, does not. Its own definition is the outermost one that holds its member function bodies. So FN-12's new words, the outermost class definition that contains the function definition, agree, and so do the two tests on a nested class defined later. INST-1's item follows FN-12 and agrees. The point of RV9-2 stands. The reference's point of instantiation for a class specialization has no exception for these places.

INST-30's new item. The operand of a `noexcept` specifier in the class's member list is one of the places where the class counts as complete in the reference. So the item agrees with FN-12's logic and with every compiler. The doubts are the two known ones. The reference's rule for the point of instantiation does not reconcile with these places (RV9-2). The reference also counts a default template argument inside the member list among these places, and the Note's reading that INST-30 does not name it follows CLS-2 and every compiler, against the reference (Q-0406).

## RV10-17

CE-3's new sentence, CE-22's new sentence, the three new tests, and Q-0449's new rows.

In the reference, an object usable in constant expressions includes a usable variable and each subobject of it that is not `mutable`. A `mutable` member is not usable, so a read of it in an evaluation that did not create the variable is not constant. A copy of a struct with a `mutable` scalar member reads that member, so the copy is not constant either. CE-3's sentence and the two reject tests agree.

The doubt is small. A member nested inside a `mutable` member is also not usable, even if it is not `mutable` itself, because the reference keeps only the parts that are not `mutable` along the whole path. CE-3's sentence names only the `mutable` member.

The reference's limit on the implicit copy or move of a union applies only when the active member is `mutable`. A `mutable` member that is not active sets no limit. So CE-22's new sentence agrees, `union-copy-inactive-mutable-member-outside.cpp` agrees, and Clang and GCC read against.

## RV10-18

ATTR-2, `maybe_unused` on a type, and `maybe-unused-on-type.cpp`.

The reference lists the entities that `maybe_unused` may apply to: a class, an alias, a variable, a structured binding, a data member, a function, an enumeration, and an enumerator. An attribute after the type specifiers of a declaration applies to the type. The reference makes a standard attribute that applies to an entity outside its list not valid. So the new item and the test agree, and GCC's acceptance with a warning conforms only because it gives a diagnostic. The item fits Q-0021.

## RV10-19

JMP-P1, JMP-P2, ATTR-P1's label row, the opening of the jumps page, and the glossary entry for label.

In the reference, a label is an identifier, a `case` with a constant expression, or `default`, each followed by a colon and each with optional attributes before it. A label may come before any statement and also at the end of a block. An identifier label serves only as the target of a `goto` in its own function. A `case` or `default` label must sit inside a `switch` and belongs to the innermost one. The glossary entry agrees. `likely` and `unlikely` may apply to a statement or to a label of any kind, so ATTR-P1's row agrees. The new JMP-P1 sentences and the region row for a local class agree with RV10-14. The JMP-P2 rows agree with RV10-13.
