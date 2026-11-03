# Gap review: question cluster 3

Date: 2026-11-03
Role: source reader

Reference: the C++23 working draft in the local reference tree (the
final C++23 draft). The review compares each provisional answer with
that draft only. Later changes are mentioned only where they matter,
and they are not in the reference.

## Result

One of the 9 provisional answers conflicts with the reference.

- Q-0220 contradicts the reference, with some doubt. The draft gives no
  preference for a plain function over a template specialization when
  it matches a placement deallocation function. Two matches give no
  deallocation function, which is option 2.

The other 8 provisional answers agree with the reference. Q-0218 and
Q-0293 carry a small doubt.

- Q-0216: the draft supports option 1, so the review does not support
  the change to option 2 that the question's note asks it to weigh.
- Q-0293: the draft leans to option 1. The draft's wording for the
  template case and for the plain-function case differs in a way that
  fits the split that Clang 22 makes.

## Questions in the cluster

| Question | Provisional answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0211 | Partial ordering reverses the parameters of a swapped candidate (option 1). | agrees | option 1 | When exactly one of the two templates came from a rewritten candidate with reversed operands, the draft reverses the order of the function parameters in that template's transformed form before deduction. |
| Q-0214 | The object parameter is an rvalue reference when the other template's first parameter is an rvalue reference (option 1). | agrees | option 1 | For a member with no ref-qualifier, the draft makes the invented object parameter an rvalue reference when the parameter at the same position in the other transformed template is an rvalue reference, and an lvalue reference otherwise. A guard handles the case where the choice would depend on itself. |
| Q-0216 | A parameter with a default argument counts against the pack, and the call is ambiguous (option 1). | agrees | option 1 | See below. |
| Q-0217 | ORD-11 applies to any two references, and the pair gives no order (option 1). | agrees | option 1 | See below. |
| Q-0218 | The object parameter is added, and then the parameters are reversed (option 1). | agrees | option 1 | The draft first inserts the object parameter at the front of a member's parameter list, then reverses the parameter order of the transformed template. The reversal therefore covers the inserted parameter. The draft states the two steps one after the other and gives no example of a swapped member, which is the only doubt. |
| Q-0219 | Partial ordering chooses the more specialized placement deallocation template (option 1). | agrees | option 1 | The draft lists the match of a placement deallocation template specialization to a placement allocation function as a context of partial ordering. The deduction used for the match fails only if no template, or more than one, remains after partial ordering. |
| Q-0220 | The plain function wins over the template (option 1). | contradicts, with doubt | option 2 | See below. |
| Q-0285 | A fold expression is one atomic constraint, and neither template is more constrained (option 1). | agrees | option 1 | See below. |
| Q-0293 | For templates, the ellipsis takes no part, and the constraint decides (option 1). | agrees, with doubt | option 1 | See below. |

### Q-0216 in more detail

- The draft's tie-breaker for a trailing pack asks whether one template
  has a trailing pack for which the other template has no corresponding
  parameter. The test looks at the declared parameters. It does not
  limit itself to the parameters that meet call arguments.
- The template with the defaulted second parameter has a parameter at
  the position of the pack, so the tie-breaker does not apply. The two
  templates are each at least as specialized as the other, and the call
  is ambiguous.
- The draft has a note with an example of this same pair of templates
  and a call with one argument, and the note marks the call ambiguous.
  That example settles the point for the C++23 text.
- ORD-5 limits the types that are compared in a call to the parameters
  with call arguments. The draft has the same limit, and it keeps the
  pack tie-breaker as a separate step that looks at the whole
  declaration. So the draft gives no support for option 2 through
  ORD-5.
- I know of no later defect resolution that changes this example.

### Q-0217 in more detail

- The draft records, before references are stripped, which of the two
  referenced types carries more cv-qualifiers. The draft does this
  whenever both types are references, of either kind.
- After a successful deduction, two checks can remove one direction.
  The first check applies when the argument side is an lvalue reference
  and the parameter side is not. The second check, on cv-qualifiers,
  applies when the first one does not.
- With the lvalue reference template as the argument, the first check
  makes the rvalue reference template not at least as specialized.
  With the rvalue reference to const as the argument, the first check
  does not apply, and the second check makes the lvalue reference
  template not at least as specialized. Neither template wins, as Clang
  finds.
- The draft links the two checks only by the condition that the second
  runs when the first does not. Both checks remove the same direction,
  so that condition never changes a result. This fits ORD-P2 and the
  provisional answer.

### Q-0220 in more detail

- The draft matches a placement deallocation function to the placement
  allocation function by comparing declarations. For templates, it
  deduces against the type that would match, and partial ordering
  narrows the templates to one.
- The draft then calls a deallocation function only when lookup finds
  exactly one match. Otherwise it calls none.
- The draft does not use overload resolution here, and it states no
  preference for a function that is not a template over a matching
  specialization. A plain function and one specialization are two
  matches, so no deallocation function is called, as Clang does.
- The doubt is whether the draft's narrowing among templates was meant
  to extend to plain functions as well. The text gives no sign of it.
  OVL-27 belongs to overload resolution, and the draft does not apply
  it to this match. The answer of Q-0219 does not carry over, because
  the draft's ordering step covers templates only.

### Q-0285 in more detail

- A type constraint on a template parameter pack gives a fold over the
  conjunction of the concept checks for the pack elements.
- In the C++23 draft, normalization splits only parentheses,
  conjunctions, disjunctions, and concept references. Any other
  expression, a fold included, is one atomic constraint.
- The two folds are different expressions, so neither atomic
  constraint is identical to the other, and neither template subsumes
  the other. The call is ambiguous, as Clang and GCC find.
- The next language version adds a way to normalize such folds through
  their operands, which would give the result EDG reports. That change
  is a new feature, and it is outside the C++23 scope (Q-0007).

### Q-0293 in more detail

- For two templates, the draft's constraint step first checks that the
  template parameter lists and the function parameter lists have the
  same length, and that parameters at the same positions have the same
  type. An ellipsis is not a parameter in the draft's terms, so both
  templates have one function parameter, and the checks pass. The
  constraint then decides. This is option 1.
- For two plain functions, the draft's constraint tie-breaker needs the
  same list of parameter types, and the draft defines that list to
  record whether an ellipsis is present. The ellipsis then counts, which
  fits Q-0146.
- The draft thus uses two different conditions for the two cases, in
  line with the split that Clang 22 makes. The doubt is that the draft
  does not define a function parameter list as a term, and a reader
  could count the ellipsis in it. The draft gives no example with an
  ellipsis in this step.
