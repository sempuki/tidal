# Read of the RV10 repair diff (88d14ea2)

Date: 2026-11-11

Role: source reader

Reference: the C++23 working draft sources.

Commit read: 88d14ea2338d573a36ae4144740a387aee70932c.

The read covers every hunk of commit 88d14ea2 under `specification/`, with the review that the commit repairs (`reports/review/2026-11-11-gap-review-phase-17.md`). The reader also read the 17 new and 6 changed tests that the hunks cite, and the question records of commits 18efd5eb, 80a70210, and 88d14ea2, with `pending.md`, where they state the review's reading. The read is item RV10, last part, of decision 0065. New points carry IDs of the form RV11-R-n.

## Result

Most hunks agree with the reference. The repairs close RV10-1, RV10-3, RV10-4, RV10-5, RV10-8, RV10-12, and RV10-17. They close RV10-7 for a member declaration and RV10-14 for the three forms with no specialization.

The text keeps the compilers' answer against the review for RV10-6 (TARG-16) and RV10-10 (`typename_specifier`). The TARG Note, the row 4 item "TARG-16", the Q-0226 line, and the Q-0226 records state the review's reading correctly. RV10-2 gains only a record, and the record is correct.

Each new Syntax form matches a form that the reference derives:

- `decltype_expr` is the reference's `decltype` form with an expression. The four places that now take it (`base_type`, the `~` arm of `unqualified_id`, `dtor_call`, and the `decltype` arm of `nested_name`) and `cast_type` take only that form in the reference too.
- The `template` arm of `base_type` and of `cast_type` matches the reference's arm of a qualifier, the keyword, and a template-id, in a base name and in the type of a functional cast.
- The leading attributes of `using_directive` match the reference, and there they apply to the directive.
- The leading attributes of `sb_decl` match the reference's optional attributes before a structured binding declaration.
- The `template_id` arm of `using_enum_decl` matches the reference's template-id arm.
- `new_ptr` matches the reference's pointer and member pointer parts in the type of a `new` expression, apart from the order of the attribute and the cv-qualifiers. That order is the known split of Q-0123 (RV11-R-4).

The reference's template-id arms in a base name and in a `using enum` declaration take only a plain template name with arguments. Tidal's `template_id` also has the operator forms. No such name can denote a class or an enumeration, so lookup rejects it, as RV10-1 said.

One hunk adds a condition. TPL-4's new sentence forbids a template head right after another in a class. The reference has a friend declaration in a class with two template heads, which names a member template of a class template specialization as a friend (RV11-R-1).

The other new points are doubts or gaps:

- The casts paragraph names a class template with no argument list and leaves out an alias template that the reference also deduces (RV11-R-2).
- ATTR-19's table has no row that fits an attribute before a using-directive (RV11-R-3).
- OBJ-2's new sentence omits one condition on the class member and gives the plain reading for a scalar inside a living class member (RV11-R-5).
- JMP-14 still rejects a missing label in a discarded statement of a used template. The reference does not decide that form (RV11-R-6).
- The `braced_init` arm of `template_arg` also derives a braced list for a scalar parameter, and no Tidal rule names that case (RV11-R-7).
- AUTO-11's new item names the place before `::`, and no test holds it (RV11-R-8).

## Hunks

| File | Hunk | Rule, principle, or nonterminal | RV10 point | Verdict |
|---|---|---|---|---|
| constructors.md | `dtor_call` takes `decltype_expr` | `dtor_call` | RV10-1 | agrees. The reference allows only the expression form of `decltype` after the tilde. |
| derived.md | `base_type` gains the `template` arm and takes `decltype_expr` | `base_type`, `mem_init_id` | RV10-1 | agrees. `mem_init_id` takes `base_type`, so a member initializer gains the same forms, as in the reference. |
| derived.md | paragraph on the keyword before the last template-id | `base_type`, DEP-25 | RV10-1 | agrees. The reference allows the keyword after any qualifier, also one that is not dependent. |
| unions.md | UNI-5 and UNI-13 tests lines | UNI-5, UNI-13 | RV10-12 | agrees. See the OBJ-2 rows. |
| unions.md | Evidence count | none | none | no reference content |
| attributes.md | ATTR-2 tests line and the item on `maybe_unused` before a using-directive | ATTR-2 | RV10-3 | agrees. The attribute applies to the directive, a directive is not on the attribute's list of entities, and a standard attribute on an entity outside its list makes the program not valid. |
| attributes.md | ATTR-19 tests line | ATTR-19 | RV10-3, RV10-5, RV10-8 | agrees with a doubt (RV11-R-3) |
| attributes.md | Evidence count | none | none | no reference content |
| attributes.md | Q-0021 divergence line and the split row | ATTR-2 | RV10-3 | agrees. EDG's rejection is the reference's result. |
| attributes.md | Note on the directive, the `new` pointer, and the structured binding | ATTR-2, ATTR-19 | RV10-3, RV10-5, RV10-8 | agrees |
| auto.md | AUTO-11 tests line and the new item | AUTO-11 | RV10-1, RV10-4 | agrees. The reference allows a placeholder only in the places it lists, and these four places are not on that list. The place before `::` has no test (RV11-R-8). |
| auto.md | Evidence count | none | none | no reference content |
| enumerations.md | `using_enum_decl` takes `template_id` | `using_enum_decl` | RV10-3 | agrees |
| enumerations.md | paragraph on the template-id | `using_enum_decl` | RV10-3 | agrees. Only an alias template can give an enumeration type through a template-id. |
| enumerations.md | ENUM-9 tests line | ENUM-9 | RV10-3 | agrees |
| enumerations.md | ENUM-20, the name must not be dependent | ENUM-20 | RV10-3 | agrees. The reference asks for a type that is not dependent and whose enumeration definition is reachable. |
| enumerations.md | Evidence count, Q-0020 line, split row | ENUM-9 | RV10-3 | no reference content |
| enumerations.md | Note on the alias template and the dependent name | ENUM-9, ENUM-20 | RV10-3 | agrees |
| exception-specifications.md | Q-0077 line names RV10-2 | EXS-5 | RV10-2 | agrees as a record. The line states RV10-2 correctly. |
| namespaces.md | `using_directive` takes leading attributes | `using_directive` | RV10-3 | agrees |
| namespaces.md | Note on the directive | `using_directive`, ATTR-2 | RV10-3 | agrees |
| structured-bindings.md | `sb_decl` takes leading attributes | `sb_decl`, `init_stmt` | RV10-8 | agrees. The reference places those attributes on the hidden variable that the declaration introduces. |
| structured-bindings.md | Note on the attribute | `sb_decl` | RV10-8 | agrees |
| casts.md | `cast_type` gains the `template` arm and takes `decltype_expr` | `cast_type` | RV10-4 | agrees |
| casts.md | paragraph on a class template with no argument list | `cast_type`, CTAD-1 | RV10-4 | agrees with a doubt (RV11-R-2) |
| casts.md | Note on the keyword and `decltype(auto)` | `cast_type`, AUTO-11 | RV10-4 | agrees |
| decltype.md | `decltype_spec` splits into `decltype_expr` and the placeholder | `decltype_spec`, `decltype_expr` | RV10-1 | agrees |
| decltype.md | sentence on where only `decltype_expr` stands | `decltype_expr`, AUTO-11 | RV10-1, RV10-4 | agrees |
| decltype.md | Note on DTY-3 and AUTO-11 | DTY-3, AUTO-11 | RV10-1, RV10-4 | agrees as a record. The Note states the review's reading correctly. |
| new-delete.md | `new_type` takes `new_ptr` | `new_type`, `new_ptr` | RV10-5 | agrees for the member pointer and the attribute after `*`. The order with cv-qualifiers reads against (RV11-R-4). The reference's grammar also derives reference parts there, and a semantic rule rejects them, since the allocated type must be an object type. So `new_ptr` loses nothing. |
| new-delete.md | paragraph on `new_ptr` | `new_ptr` | RV10-5 | reads against for the order, the known split of Q-0123 (RV11-R-4). Agrees otherwise. |
| new-delete.md | NEW-1 tests line | NEW-1 | RV10-5 | agrees |
| new-delete.md | Evidence count | none | none | no reference content |
| new-delete.md | Note on `new_ptr` | `new_ptr`, ATTR-18 | RV10-5 | agrees as a record. Clang's acceptance of the attribute before the cv-qualifier is the reference's order. |
| primary.md | `~` arm of `unqualified_id` and the `decltype` arm of `nested_name` take `decltype_expr` | `unqualified_id`, `nested_name` | RV10-1 | agrees |
| constant-evaluation.md | CE-3 tests line and the sentence on parts of a `mutable` member | CE-3 | RV10-17 | agrees. The reference keeps a part usable only when no member on its path is `mutable`. |
| constant-evaluation.md | Evidence count | none | none | no reference content |
| constant-evaluation.md | Note on CE-3 | CE-3 | RV10-17 | agrees. EDG's acceptance of a read of the `mutable` member itself reads against, as Q-0449 records. |
| objects.md | OBJ-2 tests line and the new sentence on the chain | OBJ-2, UNI-13 | RV10-12 | agrees with a doubt (RV11-R-5) |
| objects.md | Evidence count | none | none | no reference content |
| objects.md | Note on OBJ-2 | OBJ-2 | RV10-12 | agrees as a record. The Note states the RV10-12 doubt correctly. |
| jumps.md | JMP-7 leaves a template with no specialization to DEP-30 | JMP-7, DEP-30 | RV10-14 | agrees. A misplaced `break` or `continue` does not depend on the parameters, and the reference needs no diagnostic in a template that is never instantiated. |
| jumps.md | JMP-14 rejects in a discarded statement and leaves the unused template to DEP-30 | JMP-14, DEP-30 | RV10-14 | agrees for the unused template. For the discarded statement of a used template, reference silent (RV11-R-6). |
| jumps.md | Q-0451 divergence line | JMP-14, DEP-30 | RV10-14 | agrees as a record, with the doubt of RV11-R-6 |
| jumps.md | split rows cite DEP-30 | JMP-14, DEP-30 | RV10-14 | agrees |
| jumps.md | X24-3 Note gains DEP-30 | JMP-7, JMP-14, DEP-30 | RV10-14 | agrees |
| arguments.md | TARG Note on the braced argument | TARG-16, `template_arg` | RV10-6 | agrees as a record. The Note states RV10-6 correctly. TARG-16 and the arm still read against. The scalar case has no rule (RV11-R-7). |
| declarations.md | TPL-4 tests line and the sentence on two template heads in a class | TPL-4 | RV10-7 | adds a condition (RV11-R-1) |
| declarations.md | Evidence count | none | none | no reference content |
| declarations.md | Note on TPL-4 | TPL-4 | RV10-7 | agrees for the member function form that the test holds |
| dependent-names.md | DEP-25 tests line | DEP-25 | RV10-1, RV10-4 | agrees |
| dependent-names.md | DEP-30 tests line and the new list item on jumps | DEP-30 | RV10-14 | agrees. The reference's rule on a construct that does not depend on the parameters covers each jump condition of JMP-7 and JMP-14. |
| dependent-names.md | Evidence count | none | none | no reference content |
| dependent-names.md | Q-0226 line names RV10-10 | `typename_specifier`, DEP-26 | RV10-10 | agrees as a record |
| dependent-names.md | new Q-0451 line | JMP-14, DEP-30 | RV10-14 | agrees as a record, with the doubt of RV11-R-6 |
| dependent-names.md | three split rows | JMP-14, DEP-30 | RV10-14 | agrees |
| dependent-names.md | DEP-30 Note | DEP-30 | RV10-14 | agrees as a record. The Note states RV10-14 correctly. |

## Tests

| Test | Verdict |
|---|---|
| `decltype-auto-as-base.cpp`, `decltype-auto-member-initializer.cpp`, `decltype-auto-destructor-call.cpp`, `decltype-auto-functional-cast.cpp` | agree. The placeholder is not valid in each place. |
| `template-keyword-base-last-name.cpp` | agrees. The keyword is valid after a dependent and after a plain qualifier, in the base list and in the member initializer, with parentheses and with braces. |
| `template-keyword-cast-last-name.cpp` | agrees. Both casts are constant, and the empty parentheses give the member the value zero. |
| `using-directive-attribute.cpp` | agrees. An empty attribute specifier is valid. |
| `maybe-unused-on-using-directive.cpp` | agrees |
| `using-enum-alias-template.cpp` | agrees |
| `using-enum-dependent.cpp` | agrees. The rule on a type that is not dependent needs a diagnostic. |
| `new-type-pointer-forms.cpp` | agrees. In each `new` expression the type ends before the parenthesized initializer. |
| `attribute-before-binding.cpp` | agrees. `maybe_unused` may apply to a structured binding declaration. |
| `two-template-heads-in-class.cpp` | agrees. The class is not a template, and the declaration is a member declaration. |
| `write-after-destructor-union-class-member-constant.cpp` | agrees. The union member of class type is on the chain, has a trivial default constructor, and has ended. So the assignment creates a new object of that class, and the read of the written scalar is valid. |
| `write-after-pseudo-destructor-nested-union-member-constant.cpp` | agrees. The scalar union member is on the chain and has ended. The struct member above it is not a union member and adds nothing. |
| `mutable-member-part-read-outside.cpp`, `mutable-member-part-read-inside.cpp` | agree |
| the five jump tests that now cite DEP-30 | agree. Rejection is allowed. GCC's acceptance in the three `goto` tests also conforms. |
| `goto-missing-label-discarded-branch.cpp` | reference silent (RV11-R-6) |

## RV10 points

| Point | Closed | Note |
|---|---|---|
| RV10-1 | yes | `decltype_expr` and the `template` arm close both details. |
| RV10-2 | no change | The Q-0077 line records the review. |
| RV10-3 | yes | The directive's attributes, the template-id arm, and ENUM-20's dependent name. RV11-R-3 is a small remainder. |
| RV10-4 | yes | The paragraph now names deduction. RV11-R-2 is a small remainder. |
| RV10-5 | yes | `new_ptr` gives the member pointer and the attribute after `*`. It inherits the order split of Q-0123 (RV11-R-4). |
| RV10-6 | no | TARG-16 and the arm keep the compilers' answer. The TARG Note and row 4 record the review. RV11-R-7 names the scalar case. |
| RV10-7 | yes for a member declaration | TPL-4's sentence also catches a friend declaration that the reference allows (RV11-R-1). |
| RV10-8 | yes | |
| RV10-9 | no change | Q-0123 records it. |
| RV10-10 | no | The production keeps the deduced form, and Q-0226 records the review. The doubt on the deprecated keyword in a template template default stays. |
| RV10-11 | no change | RV10-11 agreed with the text. |
| RV10-12 | yes | OBJ-2 follows the chain and covers a class member. The doubt on the array element stays, as the OBJ Note and Q-0447 say. RV11-R-5 is a small remainder. |
| RV10-13 | no change | RV10-13 agreed with the text. |
| RV10-14 | yes for the three forms | JMP-7, JMP-14, and DEP-30 now give the reference's result. The discarded statement stays open (RV11-R-6). |
| RV10-15 | no change | |
| RV10-16 | no change | Q-0406 records it. |
| RV10-17 | yes | |
| RV10-18 | no change | ATTR-2 also gains the directive form, which agrees. |
| RV10-19 | no change | |

## Question records

The records of commits 18efd5eb, 80a70210, and 88d14ea2 state the review's reading correctly:

- 18efd5eb. The Opinions entries and dated notes of Q-0021, Q-0077, Q-0123, Q-0406, Q-0449, Q-0450, Q-0451, and Q-0452 state RV10-18, RV10-2, RV10-9, RV10-16, RV10-17, RV10-1, RV10-14, and RV10-13 correctly. The row 4 item "JMP-14, Q-0451" stated RV10-14 correctly before 88d14ea2 removed it.
- 80a70210. Q-0450's settling paragraph states RV10-1 correctly. The reference allows parentheses or braces after every name form of a member initializer, the `decltype` form included.
- 88d14ea2, Q-0021. The new row and note agree. EDG's result is the reference's result.
- 88d14ea2, Q-0123. The note states correctly that `new_ptr` inherits the split. Clang's result there matches the reference's order.
- 88d14ea2, Q-0225. The dated note and the Opinions entry state RV10-1's remark correctly. A base list is a place where only a type can stand, so a `<` after the last name there starts an argument list without the keyword. For the base form, that reading supports option 2.
- 88d14ea2, Q-0226. The dated note and the Opinions entry state RV10-10 correctly, with its doubt.
- 88d14ea2, Q-0447. The note states the RV10-12 doubt correctly.
- 88d14ea2, Q-0449. The note states RV10-17 correctly.
- 88d14ea2, Q-0451. The new answer and the dated note state RV10-14 correctly for the three forms. The note says correctly that the review gives no clear answer for the discarded statement. See below.
- 88d14ea2, `pending.md`. The row 4 item "TARG-16" states RV10-6 correctly for a class parameter (RV11-R-7 for the scalar case). The Q-0226 item gains the review correctly.

### Q-0451, the changed provisional answer

The new answer has two parts:

- For a function template that no specialization uses, a member function of a class template that is never used, and a generic lambda that is never called, no diagnostic is required, and Clang's, EDG's, and GCC's results each conform (option 2). The reference supports this part. A missing label does not depend on the parameters, and the reference makes a template that would fail for such a construct not valid with no diagnostic required. It also lets an implementation check a template before any instantiation, so a rejection conforms too. Option 2's own text says that the check waits for a specialization. The reference does not ask any implementation to wait. The answer's words, that each result conforms, give the reference's outcome, and the option's text describes only GCC's model. The difference is in wording only.
- For a discarded statement of a used template, the program is rejected (option 1). The reference does not decide this form (RV11-R-6). So the "Waits for" line, a review (doubt), fits.

## New questions

None.

## RV11-R-1

TPL-4's new sentence and `two-template-heads-in-class.cpp`.

The sentence forbids a template head right after another template head in a class. For a member declaration, the reference agrees. In a class that is not a template, a member template takes one template head. The extra heads that the reference allows outside a class belong to the enclosing class templates of a member defined out of line.

The reference also allows a friend declaration with two template heads inside a class. The first head belongs to a class template, and the friend is a member template of that class template's specializations. The qualifier of the friend's name ends in a template-id of that class template, and its parameters must be deducible from that template-id. The reference's own example on friends shows the form. So TPL-4's sentence holds for a member declaration and fails for this friend declaration. No test holds the friend form. RV10-7 said that a second template head inside a class is not valid. That holds only for a member declaration.

## RV11-R-2

The casts paragraph on a class template with no argument list.

In the reference, a type specifier that names a deducible template with no argument list stands for a class type whose arguments deduction supplies, and a functional cast is one of the places where that placeholder may stand. A deducible template is a class template, or an alias template whose target is a template-id of a deducible template, under a qualifier that is not dependent. So a cast can also deduce through an alias template. The paragraph names only a class template, although CTAD-1, which it cites, names the alias template too (CTAD-14). The reference also asks for a qualifier that is not dependent. The paragraph's next sentence, on `typename` for a dependent qualified name, does not say that a dependent qualifier before a template name with no arguments is not valid. The doubt is small.

## RV11-R-3

ATTR-19's tests line, `using-directive-attribute.cpp`, and ATTR-2's new item.

The reference places the attributes before a using-directive on the directive itself. ATTR-2's new item says the same. ATTR-19 now cites the directive test, but its table has no row for the directive. The nearest row, before a declaration, gives each entity that the declaration declares. A using-directive declares no entity, so that row's last column does not fit. The structured binding fits that row, since the reference gives those attributes to the hidden variable. The doubt is small.

## RV11-R-4

`new_ptr` and the paragraph after the production.

In the reference, the attribute of a pointer part in the type of a `new` expression comes right after `*` or after the member pointer's `*`, before the cv-qualifiers. `new_ptr` places it after the cv-qualifiers, as `ptr_part` does. This is the known split of Q-0123, and Q-0123 now names `new_ptr`. With no cv-qualifier, both orders give the same form, so the new test agrees. The point is a record of the split in a new production.

## RV11-R-5

OBJ-2's new sentence, the OBJ Note, and the two new object tests.

The reference collects each union member on the chain whose type is a scalar, a class whose default constructor is both trivial and usable (so not deleted), or an array whose elements have one of those types. OBJ-2's sentence and UNI-13 name a trivial default constructor and leave out the condition that it is not deleted. A default constructor that is deleted at its first declaration can still be trivial, so the sentence covers one class that the reference leaves out. No test holds the form.

The sentence also names a union member that the call ended. When the call ends only a scalar inside a union member of class type, and the class member stays alive, the chain still collects the class member. The reference does not say whether a write to the ended scalar counts as a change to the class member outside its lifetime. This is the doubt of RV10-12, now for a class member in place of an array member. OBJ-2's words give the plain reading, where no object is created. The doubt is small.

## RV11-R-6

JMP-14's sentence on a discarded statement, the Q-0451 lines on the jumps and dependent-names pages, `goto-missing-label-discarded-branch.cpp`, and Q-0451's answer for that form.

The reference makes a template not valid, with no diagnostic required, when no valid specialization exists for a statement that `if constexpr` discards and the template is never instantiated. In the test, the template is instantiated, so that clause does not apply. In the specialization, the discarded statement is not instantiated, so the specialization contains no `goto`. The reference states the label rule for a function, and it does not say whether the rule binds the template's written body or only its specializations. Under the first reading, the program needs a diagnostic, as JMP-14 says. Under the second, the program is valid, or at most not valid with no diagnostic required, and GCC's acceptance conforms. Outside a template, the discarded statement is part of the function, and the rule binds it. So JMP-14 picks one of two readings that the reference leaves open. Q-0451 records the doubt correctly.

## RV11-R-7

The `braced_init` arm of `template_arg`, the TARG Note, and the row 4 item "TARG-16".

The TARG Note says that every compiler also accepts a braced list for an `int` parameter. The arm derives that form. TARG-16 names only a parameter of class type, and no other Tidal rule I found says what a braced list means for a scalar parameter. The reference has no braced form for any parameter (RV10-6). The row 4 item names only the class case. So the scalar case has a production and a compiler result, and no rule text or row 4 item. The point is a gap in Tidal's text, with no new reference content.

## RV11-R-8

AUTO-11's new item.

The item names four places: a base, after `~`, before `::`, and the type of a functional cast. The four new tests hold the base, the member initializer, the destructor call, and the cast. No test holds the place before `::`. The reference agrees with the item in that place too. The point has no reference content and is small.
