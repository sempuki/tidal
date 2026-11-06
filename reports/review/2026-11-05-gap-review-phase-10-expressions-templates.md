# Gap review: the text phase 10 wrote on expressions and templates

Date: 2026-11-05

Role: source reader

Reference: the C++23 working draft sources. The review covers the rules and principles whose prose changed between commit d962fc7 and now on the pages for comparison, lambdas, template arguments, class template argument deduction, concepts, constraints, deduction, dependent names, partial ordering, and packs. It also covers one principle use (CNS-P2 on Q-0263) and three questions (Q-0411, Q-0293, Q-0396). Later defect resolutions are named only where they may change an answer.

## Result

Most of the changed text agrees with the reference. Four items read against it, and one item adds a condition. Several items agree with a small doubt. The use of CNS-P2 on the programs of Q-0263 now gives the provisional answer, and the reference supports that answer with no doubt. The three questions are each supported with a doubt.

Gaps found in Part A:

- TARG-27 forbids an inherited injected class name as a template template argument when two bases supply it. The reference treats the name in that use as the template itself, with no ambiguity (Q-0409).
- CTAD-16 lets the default of a derived parameter rescue an inherited guide. The reference's matching test for the guide never gives that parameter a value, so the guide fails whether or not a default exists (Q-0408). The reference is murky here.
- DEP-P6 lists a template-id with independent arguments that fail the constraints, inside a template that is never instantiated, as a case that the implementation must reject. The reference marks that case as not valid with no diagnostic required, and the table's own row for independent constructs that are not valid says the same.
- ORD-P1 says that two specializations named by explicit template arguments, with no target type, are not ordered. The reference keeps partial ordering on the function types in that case (Q-0405).
- ORD-13 keeps the conditions of equivalent heads and equal parameter types by position in force when one template is a swapped rewritten candidate. In that case the reference replaces those conditions with the reordering test. As written, the list needs both, which is stricter than the reference.

## Rules

| Rule | Verdict | Note |
|---|---|---|
| CMP-8 | agrees | Members of one base class subobject have a fixed order. A base class subobject, or a member of it, against a member of the derived object has no required order, so the comparison is not a constant expression. |
| LAM-6 | agrees with a doubt | A lambda outside a block scope can have neither a capture default nor a simple capture, outside a default member initializer. The term "named capture" is not defined, and only LAM-7 makes clear that init captures stay allowed. |
| LAM-29 | agrees with a doubt | The reference never marks a closure type final, and does not let an implementation change that property. It also does not say so in plain terms. |
| LAM-15 | agrees | A member template name after a dot on an object of dependent type needs the keyword for a following less-than sign to open an argument list. A call operator name is covered like any other name. |
| TARG-27 | contradicts | The reference treats an injected class name used as a template template argument as a name of the template, and an ambiguity between bases that are specializations of one template then goes away. |
| CTAD-16 | contradicts | The inherited guide carries a test that all parameters of the derived template are deducible from the base type. A parameter that the base does not use is never deduced, so the guide fails. A default may not help. |
| CPT-19 | agrees with a doubt | In C++23 the fold from a type constraint on a pack is one atom, and two atoms are identical only when they come from one appearance. A fold that the language makes up has no clear place in the source. A C++26 change may reverse the answer. |
| CNS-P2 | agrees with a doubt | The reference makes a program not valid, with no diagnostic required, when an atom can never pass a check and no check reaches it. The principle states only the case of a non-bool type that does not depend on a parameter, which is narrower. |
| CNS-P4 | agrees | The new sentence follows from the definition of subsumption by shared atoms. The reference does not state it, and it adds no new condition. |
| CNS-14 | agrees with a doubt | Member bodies, default arguments, and default member initializers are instantiated only on use, so a dependent template-id there is checked then. The reference is silent on whether the template template parameter's own constraints apply after substitution. |
| DED-24 | agrees | A template-id that fails its constraints is not valid where it is written, and the failure arises during substitution into the function type, so it removes the candidate. |
| DED-29 | agrees with a doubt | A losing candidate is not named in the sense that forces instantiation, so its body is not instantiated. A note in the reference says any use of a specialization with a deduced result instantiates it, which leaves some doubt. |
| DEP-P6 | wrong classification | The new row requires rejection. The reference marks a failing independent template-id inside an uninstantiated template as no diagnostic required. |
| ORD-P1 | contradicts | With no target type, the reference still adds the specializations and removes the less specialized one by ordering on the function types. The row's answer of "ambiguous" does not follow. |
| ORD-4 | agrees | In the template arguments of a class type during ordering, an argument that came from a pack expansion and has no counterpart on the other side is skipped. |
| ORD-13 | extra condition | For one swapped rewritten candidate, the reordering test replaces the positional head and type checks. ORD-13 keeps all of them. |
| PACK-P2 | agrees | A pack can be expanded in any initializer list, which includes a parenthesized initializer, a member or base initializer, a new initializer, and a functional cast. |
| PACK-13 | agrees | Each copy of the pattern is one argument of the initialization in those places. |

## Principle uses

| Principle | Question | Test file | The model's answer | Matches the provisional answer? | Reference verdict |
|---|---|---|---|---|---|
| CNS-P2 | Q-0263 | `atomic-not-bool-skipped.cpp` | The right operand of the concept has an unsigned integer type for every argument, so it can pass no check. The program is not valid even though the short circuit skips it. No diagnostic is required, and an implementation may reject the concept where it is written. | yes (option 3) | supports with no doubt |
| CNS-P2 | Q-0263 | `atomic-not-bool-nondependent-concept.cpp` | The concept is never used, and its second atom is never bool. The program is not valid, with no diagnostic required. Rejection at the definition and acceptance both fit. | yes (option 3) | supports with no doubt |
| CNS-P2 | Q-0263 | `atomic-not-bool-nondependent-clause.cpp` | The same atom in the requires clause of an unused function template. Not valid, no diagnostic required, and rejection at the definition is allowed. | yes | supports with no doubt |
| CNS-P2 | Q-0263 | `atomic-not-bool.cpp` | The check reaches the atom, the atom has type int, and the atom must be bool, so the program is not valid and is diagnosed (CNS-9). | yes | supports with no doubt |
| CNS-P2 | Q-0263 | `or-short-circuit.cpp` | The type of the skipped atom depends on the argument, so the new sentence does not apply. The left operand is true, the atom is never checked, and the program is accepted. | yes | supports with no doubt |

## Questions

| Question | Verdict | Reason |
|---|---|---|
| Q-0411 | supports with a doubt | In C++23 each type constraint on a pack gives its own fold, which is one atom. Two atoms are identical only when they come from one appearance, so neither declaration subsumes the other and the call is ambiguous. The doubt is that the fold has no written appearance, and a C++26 change may give EDG's answer. |
| Q-0293 | supports with a doubt | After both deductions succeed, the reference compares the lengths of the template parameter lists and the function parameter lists, and an ellipsis is not a parameter. So the constraint decides. The doubt is that the reference does not define the function parameter list as a term. |
| Q-0396 | supports with a doubt | Outside a call, the function type is the one type that takes part, and only a top-level reference would be stripped. The function type is not a reference, so the references inside it stay. The doubt concerns the forwarding parameter case, where an adjustment exists for deduction when an address is taken and it is unclear whether ordering in that context shares it. |

## LAM-6

The reference lets a lambda whose innermost scope is not a block have neither a capture default nor a simple capture, except inside a default member initializer of the enclosing class. A simple capture covers a plain name, a name by reference, this, and a copy of the object. Init captures are allowed. LAM-6 says "named capture", which a reader may take to include an init capture, since an init capture also has a name. LAM-7 later allows init captures at namespace scope, so the two rules together give the right answer. A short definition of "named capture" in LAM-6 would remove the doubt. The test `named-capture-in-block-function-default-argument.cpp` agrees with the reference: a default argument of a function declared in a block is in the parameter scope, so the capture is not allowed.

## LAM-29

The reference never states that a closure type is not final. It describes the closure type as a class with given members, and it lists the properties that an implementation may change. Finality is not among them, so a closure type cannot be final. The conclusion is sound, but it rests on an inference. The overload-set use agrees: the derived class template is an aggregate, so aggregate deduction applies, even though a closure type itself is not an aggregate.

## TARG-27

The reference says an injected class name used as an argument for a template template parameter is a template name for the class template. A separate passage says that when a lookup finds injected class names in more than one base, and all of them name specializations of one class template, a use as a template name refers to the template and is not ambiguous. Together these make the name in `injected-name-two-bases-template-argument.cpp` the template, and the program valid. TARG-27 forbids that use. The rule follows the compilers, which all reject, and Q-0409 records the choice. The reference reads against the rule for that use. The ban on a use as a type stays correct.

## CTAD-16

For a class template that inherits constructors, the reference builds the guides of an alias of the base type over the derived parameter list. Each such guide gets a constraint that the alias's arguments are deducible from the guide's result, through a test that matches a made-up partial specialization over the derived parameter list. A derived parameter that the base type does not use cannot be deduced from the base type, so the match fails. A real partial specialization cannot have defaults, and the reference does not say that the made-up one keeps them. So a default may not rescue the guide, and Clang's rejection fits. The doubt is that the made-up template is described as having the parameter list of the alias, which carries the defaults. Q-0408 records the choice of option 1, which follows GCC and EDG.

## CPT-19

In C++23, a type constraint on a type parameter pack gives a fold over the concept check, and the fold is not a conjunction at the top level, so it is one atom. Two atoms are identical only when they are formed from one appearance of one expression. Each declaration's type constraint introduces its own fold, so the folds of two declarations are two atoms, as CPT-19 now says. The doubt is that the fold is made up by the language and has no place in the source text. The reference does not say which appearance it has. C++26 adds a separate constraint kind for such folds, and under it, the inner concept checks of both folds come from the same concept definition, so one declaration can subsume the other. That change may give EDG's answer in `type-constraint-pack-same-concept-two-declarations.cpp`.

## CNS-P2

The reference makes a program not valid, with no diagnostic required, when an atom in any constraint expression could never pass a check and no check of it happens. CNS-P2 now gives one sufficient case: an atom whose type is not bool and does not depend on a template parameter. Other atoms that can never pass, such as an atom that can never be a constant expression, fall under the same reference rule, and the principle does not mention them. When a check does reach such an atom, the program is not valid and a diagnostic is required (CNS-9). The principle gets this right by saying "also when no check reaches the atom".

## CNS-14

The reference instantiates member function definitions, default arguments, and default member initializers of a class template only when they are used, so a dependent template-id in them is checked only then. The tests `template-id-in-called-member-body.cpp` and `template-id-in-uncalled-member-body.cpp` fit. The reference makes the template template parameter's own constraints apply to a template-id with independent arguments (CNS-38), and says nothing about a dependent one after substitution. After substitution, the template-id names the argument template, so it is reasonable that only that template's constraints apply, as in `template-template-parameter-own-constraint-dependent.cpp`. The reference does not state it.

## DED-29

The reference deduces a placeholder result type when the definition is instantiated, and a note says any use of such a specialization instantiates it. A candidate that loses overload resolution is not the selected member of the overload set, so the expression does not name it, and nothing requires its definition. Clang and GCC agree. The doubt is the breadth of the note. The reference also instantiates both candidates when an address is taken with a target type, because the result types must match the target. DED-29 says "only when the call selects it", which is correct for a call and does not cover that address case.

## DEP-P6

The reference's example for constraints on template-ids marks a failing template-id with independent arguments, inside a class template that is never instantiated, as not valid with no diagnostic required. That fits the general rule that an independent construct that can never be valid needs no diagnostic. DEP-P6's new row says "rejects" and cites CNS-13, which states the same requirement. The row conflicts with the row above it for independent constructs that are not valid (DEP-30). The test `independent-template-id.cpp` expects rejection, and all three families reject, but the reference does not require a diagnostic.

## ORD-P1

The reference selects the non-template functions when there is no target. It adds the specialization that each named function template gives, here from explicit template arguments, and then removes any specialization whose template is less specialized than another. Ordering outside a call uses the function types. Nothing in that path depends on a target, so for two templates on a value and on a pointer, the template on the pointer survives. The row now says the address is ambiguous (Q-0405). The doubt is that C++23 does not say clearly how a placeholder variable deduces its type from such an overload set. A later core defect resolution addresses deduction from an overload set with no target and may change the answer.

## ORD-13

When exactly one of two templates comes from a swapped rewritten candidate, the reference drops the plain position-by-position checks of the heads and the parameter types. It asks instead for every template parameter to be deducible from the function parameters, and for exactly one reordering of one head that makes the heads equivalent and the corresponding parameter types the same. ORD-13 introduces its list with "all these conditions hold", so the new bullet is added to the positional bullets. For the reference's own case of two equality operators with three parameters each, the positional checks fail and only the reordering succeeds. ORD-13 then says that constraints do not decide, and the reference says they do. ORD-13 should make the swapped-candidate bullet replace the two positional bullets in that case.

## Q-0411

See CPT-19. The C++23 reference gives two atoms, so neither template is more constrained and the call in `type-constraint-pack-same-concept-two-declarations.cpp` is ambiguous, as Clang and GCC say. The doubt is the made-up fold's appearance, and the C++26 change may give EDG's answer.

## Q-0293

For two templates, the constraint step first compares the lengths of the template parameter lists and the function parameter lists. An ellipsis is not a parameter, so a template on one parameter and a template on one parameter and an ellipsis have lists of one length. Their heads match and their single parameters have the same type, so the more constrained template wins in both `constraints-ellipsis.cpp` and `constraints-ellipsis-on-second.cpp`. For two plain functions, the reference compares parameter-type-lists, which record the ellipsis, so Q-0146 can differ. The doubt is that the reference does not define the function parameter list as a term.

## Q-0396

Outside a call, the function type of each template is the one type compared. The stripping of references and top-level qualifiers applies to that type as a whole, and a function type is neither. Deduction then compares the parameter types inside it by structure, so a template on a plain parameter accepts the invented reference type and a template on a reference parameter does not accept the invented plain type. That gives option 1 for `address-reference-against-value.cpp` and `address-value-against-forwarding.cpp`. The doubt concerns a forwarding parameter. The reference turns a forwarding parameter into a plain one against an lvalue reference when function types come from deduction for an address. It does not say whether ordering in the same context shares that adjustment. Without it, the result of `address-reference-kinds.cpp` would be ambiguous.
