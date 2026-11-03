# Gap review: question cluster 5

Date: 2026-11-03
Role: source reader

Reference: the C++23 working draft in the local reference tree (the
final C++23 draft). The review compares each answer with that draft
only. The draft already contains the defect resolution that changes how
a string literal meets an array member in the aggregate candidate of
class template argument deduction, which matters for Q-0277. Later
changes are mentioned only where they matter, and they are not in the
reference.

## Result

Six of the 34 answers conflict with the reference. In each of the six,
the draft sides with EDG on at least one point, so Clang and GCC share
the defect there.

- Q-0195 contradicts the reference. The draft states the rule for a
  parameter after a pack with no exemption from diagnosis, and its own
  example marks two forms of the test as errors. The draft supports
  option 2.
- Q-0197 contradicts the reference. The rule on a friend template with
  a default argument carries no exemption from diagnosis. The draft
  supports option 2.
- Q-0235 contradicts the reference, with doubt. The draft has no rule
  on where an explicit instantiation of a class template appears or on
  whether its name is qualified. The draft supports option 2.
- Q-0277 contradicts the reference in part. For the form where EDG
  differs, the draft agrees with Clang and GCC. For the first form,
  which every compiler deduces as an array of pointers, the draft
  gives an array of char. The draft supports option 2.
- Q-0280 contradicts the reference. The draft forbids two guides for
  one class template with equivalent parameter lists, guide templates
  included. The draft supports option 2.
- Q-0282 contradicts the reference, with doubt. The constraint of a
  constrained constant parameter takes part in the check of the
  explicit argument, and that check comes before the associated
  constraints. The draft supports option 2.

Two answers get no clear support.

- Q-0266 agrees for the copy constructor. For the default constructor
  the draft is unclear, and its literal text leans to option 2.
- Q-0272: the draft has no rule on a pack expansion as the argument
  for a concept parameter that is not a pack.

The other 26 answers agree with the reference. Q-0201, Q-0203, and
Q-0278 carry a doubt.

## Questions in the cluster

| Question | Answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0146 | Constraints decide only when parameter lists, heads, and conversion results are the same (option 1). | agrees | option 1 | For plain functions the draft compares constraints only when the parameter type lists are the same, and a default argument or an ellipsis makes the lists differ. For templates, a difference in the length of either list, a pair of parameters that are not of the same type, or different conversion results stop the comparison. Every row of the table falls under one of these. |
| Q-0170 | The argument is a type, and the program is rejected (option 1). | agrees | option 1 | The draft settles the doubt between a type and an expression in a template argument in favor of the type, whatever kind of parameter it meets. |
| Q-0172 | The declaration is rejected without a call (option 1). | agrees | option 1 | The draft allows a partial specialization only of a class or variable template. The rule is not exempt from diagnosis, and it applies to the declaration itself. |
| Q-0195 | Invalid, no diagnostic required (option 1). | contradicts | option 2 | See below. |
| Q-0197 | Invalid, no diagnostic required (option 1). | contradicts | option 2 | See below. |
| Q-0198 | The friend declaration is valid (option 1). | agrees | option 1 | See below. |
| Q-0201 | The argument template matches (option 1). | agrees, with doubt | option 1 | See below. |
| Q-0203 | The explicit specialization can be thread-local alone (option 1). | agrees, with doubt | option 1 | See below. |
| Q-0208 | The noexcept part gives a value to the bool parameter (option 1). | agrees | option 1 | The draft lists the noexcept part of a function type among the forms that deduction takes apart, gives the parameter there the type bool, and has an example that deduces true and false from two functions. |
| Q-0215 | Defaults are substituted before the check of parameters with no template parameter (option 1). | agrees | option 1 | The draft orders the steps. The defaults and the head are substituted first, then the constraints are checked, then each parameter that was not dependent is checked against its argument, and the function type comes last. A failure inside the instantiated class body is not a deduction failure. |
| Q-0224 | The name is a value, and the instantiation fails (option 1). | agrees | option 1 | A dependent qualified name outside the draft's list of type-only contexts is taken as a value. The list has no entry for the operand of sizeof, a functional cast, or a function declared in a block, and an example of the draft marks the block case as an error. |
| Q-0229 | The alias names the current instantiation (option 1). | agrees | option 1 | The draft's definition of the current instantiation names an equivalent alias template specialization in so many words. |
| Q-0231 | The initializer waits for a use (option 1). | agrees | option 1 | Instantiating a class instantiates the declarations of its static data members but not their definitions. The initializer of an inline static member belongs to the definition. |
| Q-0235 | The name must be qualified (option 1). | contradicts, with doubt | option 2 | See below. |
| Q-0237 | An explicit specialization after an explicit instantiation declaration is not valid (option 1). | agrees | option 1 | The draft forbids an explicit instantiation of either form together with an explicit specialization for the same arguments, unless the specialization is reachable from the instantiation. A diagnostic may be skipped only when neither is reachable from the other, so a single unit needs one. |
| Q-0240 | Deduction works through an alias template in each form (option 1). | agrees | option 1 | The draft builds guides for the alias from the guides of the class template. A fixed argument becomes a plain parameter type in the new guide, so an argument that converts to it is accepted. |
| Q-0245 | The keyword template is not valid before a destructor name (option 1). | agrees | option 1 | The draft forbids the keyword right before the tilde of a destructor name. |
| Q-0256 | The explicit instantiation writes the deduced type (option 1). | agrees | option 1 | A variable may use a placeholder only in a declaration with an initializer, and an explicit instantiation has none, so the placeholder form is not valid. The draft lets a variable be redeclared with its deduced type. A third type differs from the type of the specialization and is not valid. |
| Q-0260 | The trailing clause is valid on the lambda and the local class member (option 1). | agrees | option 1 | The draft allows a trailing clause on a templated function. A local class in a templated entity, its members, and a closure type made in one are templated, so their functions are templated functions. A closure type in the declaration of a class template is templated too. |
| Q-0265 | The more constrained argument is not valid (option 1). | agrees | option 1 | The draft's matching rule compares the constraints of the parameter and the argument when the parameter is constrained, and an example rejects an argument whose constraint is stronger. |
| Q-0266 | Only eligible special members count (option 1). | agrees for copy, unclear for default construction | option 1 for copy, unclear for default construction | See below. |
| Q-0267 | A definition is required (option 1). | agrees | option 1 | The draft requires a friend function template whose constraint depends on a parameter of the enclosing template to be a definition. |
| Q-0268 | Two functions (option 1). | agrees | option 1 | The draft gives such a constrained friend no link to a declaration in any other scope, so each specialization of the class has its own friend. |
| Q-0272 | The expansion is not valid for a concept (option 1). | silent or unclear | none | See below. |
| Q-0277 | Braces elided for a fixed bound, also for a string literal (option 1). | contradicts, in part | option 2 | See below. |
| Q-0278 | The parameter keeps its kind, and the lvalue does not bind (option 1). | agrees, with doubt | option 1 | See below. |
| Q-0279 | The alias gives the same result as the class template (option 1). | agrees | option 1 | The draft skips the initializer-list phase when the single element is a specialization of the class template that the placeholder names directly or indirectly, which covers an alias. |
| Q-0280 | Non-template guides with the same parameters are invalid, guide templates are valid (option 1). | contradicts | option 2 | See below. |
| Q-0281 | The constrained template is more constrained (option 1). | agrees | option 1 | A constant parameter whose type is a constrained placeholder adds the constraint to the template, and the draft counts such constraints among the associated constraints. |
| Q-0282 | The head's constraints are checked in parameter order (option 1). | contradicts, with doubt | option 2 | See below. |
| Q-0283 | Invalid, no diagnostic required (option 1). | agrees | option 1 | The draft requires the constraints of a constrained template template parameter to hold when its arguments are not dependent. An example inside a class template with such a parameter marks this exact use as invalid with no diagnostic required. |
| Q-0284 | The mapping holds only the parameters that the expression uses (option 1). | agrees | option 1 | The draft builds the mapping from the parameters that appear in the expression. A normalization example accepts a concept whose argument would be invalid in an atom that never uses it. |
| Q-0286 | A constrained placeholder is not valid on a structured binding (option 1). | agrees | option 1 | The draft limits the specifiers of a structured binding declaration to a short list, and an example rejects the constrained placeholder. |
| Q-0292 | The partial specialization is not valid (option 1). | agrees | option 1 | The draft requires a partial specialization to be more specialized than the primary template. With the same arguments, only stronger constraints make it so, and neither constraint subsumes the other. |

### Q-0195 in more detail

- The draft forbids a pack of a function template to be followed by a
  parameter that is neither deducible from the function parameters nor
  given a default. The rule has no exemption from diagnosis.
- The draft's example after the rule shows a function template with two
  type packs and one with a pack followed by a plain type parameter,
  and it marks both as errors. These are two of the three forms in the
  question.
- The constant pack whose type is the earlier type pack falls under the
  same rule, since no function parameter gives it a value.
- A general rule makes a template with no valid specialization invalid
  with no diagnostic required. The specific rule still asks for a
  diagnostic, so EDG conforms and Clang and GCC do not. The draft
  supports option 2.

### Q-0197 in more detail

- The draft requires a friend function template declaration that gives
  a default template argument to be a definition, and to be the only
  declaration of that template in either direction of reachability.
- The rule has no exemption from diagnosis. Both test forms put the
  other declaration in the same unit, where each is reachable from the
  other, so an implementation can see the violation.
- The draft supports option 2, EDG's result.

### Q-0198 in more detail

- The draft forbids binding a template parameter's name in a later
  declaration inside the parameter's scope. A friend declaration binds
  no name, and the draft's example accepts a friend function named
  after a template parameter.
- The draft's rule that a friend binds no name does not depend on
  whether the friend is a definition. The friend definition, which
  Tidal leaves open, therefore fits Clang's acceptance. GCC and EDG
  reject it, which the draft does not support.

### Q-0201 in more detail

- The draft matches a template template argument by rewriting the
  parameter and the argument as two function templates and asking
  whether the one for the parameter is at least as specialized.
- In that rewrite the invented class template takes the argument's
  head, so its constant parameter has the wider type. Deduction then
  meets an argument of the narrower type converted to the wider one.
  The draft's rule that fails deduction on a type mismatch compares the
  type of the deduced parameter with the invented template's parameter,
  and those types are equal here. So the match succeeds, which is
  option 1.
- The doubt is that the draft does not say plainly how deduction treats
  a dependent argument that was converted to the parameter's type.
  I know of no later resolution that settles the case.

### Q-0203 in more detail

- The draft lets an explicit specialization carry thread-local storage,
  and no other storage class.
- The draft requires thread-local storage on every declaration of a
  variable once one declaration has it. Whether the declaration of the
  variable template counts as a declaration of the explicit
  specialization is not stated. The draft treats a specialization as an
  entity of its own in other places, which supports option 1.
- The doubt is that the draft has no example of the mixed case.

### Q-0235 in more detail

- The draft has no rule that places an explicit instantiation in the
  namespace of its template, and no rule that its name be qualified.
  Earlier standards had both rules. The draft dropped them in its
  rewrite of scope and lookup.
- For a class template, ordinary lookup finds the name in the explicit
  instantiation, and a using-directive makes the template visible. The
  program is then valid, as EDG finds.
- The draft keeps a lookup limit for a function template
  specialization, which only considers templates that belong to the
  scope where the declaration appears. That limit does not apply to the
  class form of the test.
- The doubt is that Clang and GCC still apply the old rule, and the
  draft may have dropped it without intent. The draft supports option
  2.

### Q-0266 in more detail

- The draft calls a special member eligible when it is not deleted, its
  constraints hold, and no rival of the same kind is more constrained.
  A class is trivial when all of its eligible default constructors are
  trivial.
- For a union's copy constructor, the draft looks at the constructor
  of each variant member that overload resolution selects. Overload
  resolution prefers the more constrained candidate, so the copy test
  gets a trivial copy and the union's copy is not deleted. This is
  option 1.
- For a union's default constructor, the draft's text asks only
  whether some union member's class has a default constructor that is
  not trivial. It does not mention selection or eligibility. Read as written, a member class
  that declares any non-trivial default constructor, eligible or not,
  makes the union's default constructor deleted. That reading gives
  EDG's result for the three default-constructor tests.
- The intent of eligibility suggests option 1, and the copy rule shows
  the draft using selection elsewhere. The default rule is the doubt.
- The next language version reworks the rules for the default
  constructor of a union. That change is not in the reference, and I
  have not checked how it treats this case.

### Q-0272 in more detail

- The draft calls a template argument followed by an ellipsis a pack
  expansion, and it states no rule that limits such an argument to a
  parameter pack of a concept.
- Normalization of a concept-id substitutes each argument for the
  matching parameter of the concept. With an unexpanded pack in the
  argument list, the draft does not say which argument goes to which
  parameter, so normalization of the requires clause is not well
  defined. That points toward option 1 for the clause, but the draft
  does not say it.
- A long-standing core issue on the same question for alias templates
  was open at the time of this draft.

### Q-0277 in more detail

- The draft's aggregate candidate does not consider brace elision for
  an array member whose element type is dependent when the element of
  the list is a string literal. The string literal then corresponds to
  the whole array, and its parameter becomes a reference to a const
  array of the member's type.
- In the first form, the element type depends on the parameter and the
  initializer is a string literal. The draft then deduces the parameter
  as char, as the gap review reads. Every compiler gives an array of
  pointers, so all three share the defect.
- In the second form, the array does not depend on the parameter, so
  brace elision is considered. A string literal can initialize the whole
  char array, so it does, and the parameter for that element is a
  reference to a const array. The other element gives int. This is
  Clang's and GCC's result, and EDG's rejection is a defect.
- A draft example deduces char for a class with a dependent element
  type and a dependent bound, which points the same way as the first
  form.
- So the draft supports option 2 for the first form and the provisional
  result for the second.

### Q-0278 in more detail

- In the draft, an rvalue reference to a template parameter is not a
  forwarding reference when, while deducing class template arguments,
  that parameter stands for one of the class's own parameters.
- In a guide built through an alias, the alias parameter takes the
  place of the class parameter in the guide's function type. Read that
  way, it still stands for a class parameter, the reference is not a
  forwarding reference, and the lvalue does not bind. This is option 1.
- The doubt is that the draft does not say what a parameter of the
  alias stands for. The guide's template parameters are the alias's
  own, and a reader could treat them as fresh parameters, which gives
  EDG's result.

### Q-0280 in more detail

- The draft forbids two deduction guides for the same class template
  whose parameter lists are equivalent, when either is reachable from
  the other. The result type plays no part, and the rule is not exempt
  from diagnosis.
- The rule makes no exception for guide templates. Two guide templates
  with the same head and parameter list have equivalent parameter
  lists, so the second pair is not valid either.
- This is the gap review's reading. Clang and GCC reject only the pair
  of guides that are not templates, through the rules for overloading,
  and EDG accepts both. The draft supports option 2.

### Q-0282 in more detail

- When a call gives explicit template arguments, the draft first asks
  whether the resulting template-id is valid, and deduction fails if it
  is not. Validity needs each argument to match its parameter.
- For a constant parameter whose type has a placeholder, the draft takes
  the type from an invented variable declaration initialized with the
  argument. A constrained placeholder in such a declaration requires the
  constraint to hold for the deduced type. In the test the constraint
  fails, so the argument does not match the parameter.
- The associated constraints, in order of the parameters, are checked
  only after that step. So the type constraint of the first parameter
  is never checked, the class with the failing assertion is never
  instantiated, and the second template is chosen. This is EDG's and
  GCC 14's result.
- The doubt is that the draft does not say plainly that a failed
  placeholder constraint makes the argument not match, as opposed to
  making the program invalid. The constraint also appears among the
  associated constraints, which is where Clang and GCC 16 check it. The
  draft supports option 2.
