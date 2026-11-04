# Gap review: open questions on templates

Date: 2026-11-03
Role: source reader

Reference: the C++23 working draft in the local reference tree (the
final C++23 draft). The review compares each answer with that draft
only. The cluster is the 43 open questions whose first Affects rule is
on a page under the templates group and that no gap review has read.
The draft already contains the rewrite of scope and lookup, which
matters for Q-0248. Later changes are mentioned only where they
matter, and they are not in the reference.

## Result

Eight of the 43 answers conflict with the reference in some form.

- Q-0168 contradicts the reference, with doubt. The draft treats an
  alias template that passes its parameters through as a template of
  its own, so the two types differ. The draft supports option 1
  (Clang). An old open core issue backs GCC's and EDG's reading, and
  no resolution of it is known to have been adopted.
- Q-0205 agrees for the tested cases but contradicts TSP-30 in part.
  The draft does not allow an explicit specialization after a use for
  an unscoped member enumeration that the class template only
  declares. The draft also settles the open case. A scoped
  enumeration that the class template defines can be specialized
  after a use.
- Q-0246 contradicts the reference, with doubt. The repeated lookup of
  a qualified name in the current instantiation makes the program
  ill-formed with a diagnostic (option 3). The general rule for a
  construct whose meaning changes points to option 1.
- Q-0247 contradicts the reference. A variadic template whose only
  valid specializations need an empty pack is ill-formed with no
  diagnostic required. The draft supports option 2.
- Q-0248 contradicts the reference. A conversion function name with a
  dependent target type is a dependent name, and lookup at
  instantiation finds it in a dependent base. The draft supports
  option 2, which is Clang's result.
- Q-0263 contradicts the reference in part. The draft agrees that the
  program is not valid, but no diagnostic is required, so GCC's
  acceptance conforms. CNS-9 and CNS-39 are of kind reject.
- Q-0269 contradicts the reference, with doubt. The draft treats the placeholder
  form and the written-out template as two templates (option 2), and
  the program needs no diagnostic. The doubt is that the draft states
  this only in an example.
- Q-0273 contradicts the reference for the partial specialization,
  with doubt. The C++23 draft has no rule that each parameter of a
  partial specialization must be deducible, so the declaration is
  valid and never matches (option 2). The function type form agrees
  with option 1.

One answer meets a reference that is silent or unclear.

- Q-0200. The draft does not say whether two uses of one alias
  specialization with the same arguments give one closure type or
  two.

Two answers agree with a doubt.

- Q-0287. The draft's normative steps support option 1, but one of
  its examples points to option 3 for an extra argument that the
  concept never uses.
- Q-0295. The draft is not clear on which type a constant is compared
  with when the primary template's parameter has a placeholder type.

The other 32 answers agree with the reference.

## Questions in the cluster

| Question | Answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0168 | The result is unspecified (option 3). | contradicts, with doubt | option 1 | See below. |
| Q-0171 | The class is a valid place for the explicit specialization (option 1). | agrees | option 1 | The draft lets an explicit specialization appear in any scope where its primary template could be defined. A member template can be defined in its class, so the class body is a valid place. The draft lists a member function template of a class among the things that can be explicitly specialized. GCC's rejection is a defect. |
| Q-0173 | The program is not valid, and no diagnostic is required (option 1). | agrees | option 1 | The draft requires a partial specialization to be reachable from every use that would pick it during instantiation. A breach of that rule needs no diagnostic. So GCC and EDG may reject, and Clang may accept and keep the primary template. |
| Q-0174 | The partial specialization must be more specialized, and the implementation diagnoses it (option 1). | agrees | option 1 | The draft requires every partial specialization to be more specialized than its primary template. With the two parameters swapped, deduction succeeds in both directions, so neither is more specialized and the declaration is not valid. The draft does not exempt the rule from diagnosis, so GCC's and EDG's acceptance is a defect. |
| Q-0196 | A friend declaration of a class template must not give a default (option 1). | agrees | option 1 | The draft forbids a default template argument in a friend declaration of a class template outright. For a friend function template the draft allows a default only on a definition that is the only declaration, which matches the rule the question cites. The rule is not exempt from diagnosis, so GCC's acceptance is a defect. |
| Q-0199 | The definition after the body is valid (option 1). | agrees | option 1 | The draft lets a template declaration define a member enumeration of a class template, and it states that a member enumeration of a class template can be defined outside the class. Its example uses an enumeration with a fixed base that depends on the parameter, much like the scoped case in the question. GCC's pedantic error has no support in the draft. |
| Q-0200 | Two uses of one specialization give one closure type (option 1). | silent or unclear | none clearly | See below. |
| Q-0204 | The declaration of an explicit specialization is not access-checked, for function templates as for class templates (option 1). | agrees | option 1 | The draft turns off the usual access checks for names used when an explicit specialization or explicit instantiation is declared. The exceptions are a function body, a default argument, a base list, a member list, an enumerator list, and an initializer of a static data member or variable template. Parameter types of a function are not among the exceptions, and a note in the draft names them as a place where a private type can appear. GCC's rejection is a defect. |
| Q-0205 | A member enumeration that the class template only declares can have an explicit specialization, also after a use; an unscoped one that the class template defines cannot (option 1). | agrees for the tested cases; contradicts TSP-30 in part | option 1 for the table | See below. |
| Q-0207 | The explicit specialization replaces the pointer partial specialization for that one class specialization, and the others still apply (option 1). | agrees | option 1 | When a program explicitly specializes one partial specialization of a member template for a single class specialization, the draft keeps the primary member template and the remaining partial specializations in play for that class specialization. The explicit one takes the place of the partial one it specializes. GCC's choice of the class template's partial specialization is a defect. |
| Q-0209 | The bound has the type of a sizeof expression (option 1). | agrees | option 1 | The draft states that the bound of an array type has the standard size type, and that a constant parameter declared with a placeholder or a dependent type takes its type from the type of the deduced value. An example in the draft deduces the size type from an array argument this way. GCC's result is a defect. |
| Q-0223 | No diagnostic is required while the template is unused (option 1). | agrees | option 1 | The draft makes a program ill-formed with no diagnostic required when a construct that depends on no template parameter would make an instantiation placed right after the definition invalid. The draft also allows an implementation to check a template before any instantiation. A conversion from an integer to a pointer and a call that needs one both fall in this case, and an example in the draft treats such a conversion as one that may be diagnosed early. Both GCC's and the other compilers' results are allowed. |
| Q-0228 | No diagnostic is required while the template is unused (option 2). | agrees | option 2 | A qualified name whose scope is the current instantiation is dependent only when the class has a dependent base and lookup finds nothing. With no dependent base, the name is not dependent, so the failed lookup is an error in a construct that depends on no template parameter. The rule from Q-0223 then applies, and an example in the draft treats a call to an undeclared member named through the current instantiation as one that may be diagnosed early. GCC 16 rejects by default, which the draft allows and does not require. Once the class is instantiated, the error needs a diagnostic, which fits DEP-17. |
| Q-0230 | The qualified name is looked up again and is ambiguous (option 1). | agrees | option 1 | The draft repeats the lookup of a qualified name whose scope is the current instantiation and that finds a member when a specialization is made. When the second result differs from the first, the draft calls the lookup ambiguous, so the program is not valid. An example in the draft shows the same base layout with member access through the object pointer and an unqualified name. The unqualified form is valid, since it is not rewritten into member access at the definition. GCC's acceptance of the qualified form is a defect. |
| Q-0232 | The scoped enumeration's definition waits for a use (option 1). | agrees | option 1 | When a class template specialization is instantiated, the draft instantiates only the declaration of a scoped member enumeration, and it instantiates the definition of an unscoped one. So the enumerators of the scoped enumeration are not instantiated with the class, and the program is valid. GCC's rejection is a defect. |
| Q-0239 | The class gets the guides of the base through its inherited constructors (option 1). | agrees | option 1 | The draft adds guides for a class template that inherits constructors from a direct base. The draft builds them from the guides of an alias template whose target is the base, and it maps their results back to the derived class. A worked example in the draft deduces the derived specialization from one argument, and it fails a class that has the base but no inheriting declaration. A user guide of the base takes part through the alias. The tie-breaker that prefers the class's own guides over inherited ones applies only when the parameter types are the same for every explicit argument. So a pair whose types differ stays ambiguous, which is GCC's and EDG's result. Clang 18 and 22 do not implement this part of C++23. |
| Q-0241 | The guide is not valid and must be rejected (option 2). | agrees | option 2 | The draft requires each template parameter of a guide that has no default to be deducible from the guide's parameter types. The rule carries no exemption from diagnosis. GCC's and EDG's acceptance is a defect. |
| Q-0242 | The result must name the specialization with no cv-qualifier (option 1). | agrees | option 1 | The grammar of a deduction guide allows only a template name with an argument list after the arrow. A cv-qualifier has no place there, so the form is a syntax error and needs a diagnostic. GCC's acceptance is a defect. |
| Q-0244 | The candidate is formed for a parenthesized list in a new expression too (option 1). | agrees | option 1 | The draft forms the aggregate candidate when the initializer is a non-empty braced list or a parenthesized expression list. The draft makes no exception for the initializer of a new expression, and a placeholder for a deduced class type may appear as the type in a new expression. GCC's rejection is a defect. |
| Q-0246 | Not valid, no diagnostic required (option 1). | contradicts, with doubt | option 3, or option 1 under a second reading | See below. |
| Q-0247 | The implementation rejects the template at the definition (option 1). | contradicts | option 2 | See below. |
| Q-0248 | The name is not found (option 1). | contradicts | option 2 | See below. |
| Q-0249 | No diagnostic required; both results are allowed (option 1). | agrees | option 1 | The draft makes a program ill-formed with no diagnostic required when its validity or meaning would change by taking into account a default argument that appears in a declaration reachable from the point of instantiation but that lookup for the specialization does not find. A qualified call is not a dependent call, so the name is bound at the definition, and the later declaration is not part of that lookup. A note in the draft lists a later default argument among the ways that the meaning at the definition and at an instantiation can differ. |
| Q-0250 | No diagnostic required; both results are allowed (option 1). | agrees | option 1 | A note in the draft names the case directly. Constant evaluation inside an instantiation that relies on the definition of a constexpr function not yet defined when the template was defined is one of the ways a construct's meaning can differ between the definition and an instantiation. That difference makes the program ill-formed with no diagnostic required. The same holds for the array bound form. |
| Q-0251 | The instantiation is not valid (option 1). | agrees | option 1 | When the draft checks whether a declaration made by instantiation redeclares another validly, it counts an instantiated declaration as a definition when the template's declaration is a definition. Two member partial specializations that both have a body and become the same partial specialization for one argument are then two definitions of one entity. An example in the draft rejects a class instantiation with this shape. GCC's and EDG's acceptance is a defect. |
| Q-0252 | A braced initializer instantiates each constexpr function that it names, also on a branch that is not taken. Other unevaluated operands do not (option 1). | agrees | option 1 | The draft instantiates a constexpr function when an expression that may be constant evaluated names it. That set covers potentially evaluated expressions, expressions that must be constant evaluated, and each direct element of a braced list (for the narrowing check), with their subexpressions. A worked example in the draft instantiates a function on the branch of a conditional that is not taken when the conditional is an element of a braced list in an unevaluated operand. A plain operand of the size operator is unevaluated and is none of these, so GCC's instantiation there is a defect. The parenthesized cast is not a braced list, so no instantiation happens. EDG's result on the braced conditional row is a defect. |
| Q-0253 | The default argument is instantiated with the function (option 1). | agrees | option 1 | The draft says that a local class and its members, including their default arguments, exception specifications, and member initializers, are never instantiated on their own. Their checks happen when the enclosing entity is instantiated. The statement is a note, but it describes the effect of the normative rules. GCC's acceptance is a defect. |
| Q-0258 | Not valid (option 1). | agrees | option 1 | The draft forbids a trailing requires clause on any virtual function. A function that overrides a virtual function of a base is itself virtual, whether or not it repeats the keyword, so the second test falls under the same ban. The ban is not exempt from diagnosis, so GCC's acceptance is a defect. |
| Q-0259 | Not valid (option 1). | agrees | option 1 | The draft bans a default argument on a parameter of a requires expression, and bans a parameter list there that ends with an ellipsis. Neither ban is exempt from diagnosis, so GCC's acceptance of both forms is a defect. |
| Q-0262 | The leading clause comes before the placeholder constraints (option 1). | agrees | option 1 | The draft builds the associated constraints as one conjunction in a fixed order. The type constraints in the template parameter list come first, then the clause after the parameter list, then the type constraints of placeholder parameters, then the trailing clause. The draft also says this order is the order of checking, and a conjunction stops at the first false operand. In the test, the false leading clause ends the check before the placeholder's constraint is substituted, which is GCC's and EDG's result. Clang's order is a defect. |
| Q-0263 | Not valid, and diagnosed at the definition (option 1). | contradicts, in part | option 1 for validity, with no diagnostic required | See below. |
| Q-0264 | Not valid, no diagnostic required (option 1). | agrees | option 1 | The draft makes a program not valid when two checks of the same atomic constraint with the same arguments give different results at different points, and it requires no diagnostic. Both GCC 16's error and the other compilers' silence conform. |
| Q-0269 | One template (option 1). | contradicts, with doubt | option 2, and the program needs no diagnostic | See below. |
| Q-0273 | The partial specialization is rejected; for a function type the pack is empty unless explicit arguments fill it (option 1). | contradicts for the partial specialization, with doubt; agrees for the function type | option 2 for the partial specialization, option 1 for the function type | See below. |
| Q-0274 | Not valid, no diagnostic required (option 1). | agrees | option 1 | The draft lists, among programs that are not valid and need no diagnostic, a template whose meaning in a hypothetical instantiation right after its definition differs from its meaning in an actual instantiation. A note in the draft names this exact case. Lookup at the definition finds the expanded using-declarator, and the instantiation with an empty pack finds nothing there. So Clang's rejection and GCC's and EDG's call to the namespace function are both allowed. |
| Q-0276 | The last element deduces its own type, which conflicts, so deduction fails (option 1). | agrees | option 1 | The draft builds the aggregate candidate with the trailing pack of bases matched to all remaining elements. Ordinary deduction then runs on every pair of parameter and argument. An empty braced list gives a pair that deduces nothing. An element of a class type deduces that type for its position in the pack. When that type differs from the one that the earlier element fixed, the deductions are inconsistent and deduction fails. The draft has no step that skips deduction for a pack whose length is already known. |
| Q-0287 | The steps run in order, a failed step stops later ones, and unused extra arguments are substituted (option 1). | agrees, with doubt | option 1 | See below. |
| Q-0290 | A parameter list of only void means no parameters (option 1). | agrees | option 1 | The parameter list of a requires expression uses the same grammar as a function's parameter list, and the draft refers to the function rules for it. A lone unnamed parameter of type void, where that type does not depend on a template parameter, means an empty list. In the test, void does not depend on anything, so the rule applies inside the template as it does outside. Clang's false result is a defect. |
| Q-0295 | The argument's type must fit the declared type with no conversion, as in a function template (option 1). | agrees, with doubt | option 1 | See below. |
| Q-0296 | The defaults of all declarations count, and the program is rejected (option 1). | agrees | option 1 | The draft merges the defaults of all reachable earlier declarations, in the way default function arguments merge. The draft requires each parameter after one with a default to have a default supplied. Read with the merge, the second parameter has no default in any declaration, so the program is not valid. The rule carries no exemption from diagnosis. The draft's own example of merging adds defaults from right to left, which every compiler accepts. |
| Q-0310 | Either point is allowed, since the two points give different meanings (option 1). | agrees | option 1 | The draft gives a function template specialization a point right after the use and another point at the end of the unit. When two points give a specialization different meanings, the program is not valid and needs no diagnostic. The draft has no special point for a constexpr function used outside constant evaluation. Each of the three results is therefore allowed. I know of no later change that fixes the point for this case. |
| Q-0352 | A list in parentheses forms the aggregate candidate for a class with a pack of bases (option 1). | agrees | option 1 | The draft forms the aggregate candidate from a parenthesized expression list under the same conditions as from a braced list. The treatment of a trailing pack of bases does not depend on the kind of list. GCC's rejection is a defect. |
| Q-0354 | The program is valid, and EDG's rejection is a defect (option 1). | agrees | option 1 | A parameter declared with a placeholder in a generic lambda gets the type of an invented template parameter. A name declared with a dependent type is type-dependent, and the type given by the declared-type operator applied to a type-dependent expression is a dependent type. So the cast and the call depend on a template parameter and are checked only when the call operator is instantiated. The draft gives no different treatment to an invented parameter and one written in an explicit list. |

### Q-0168 in more detail

- The draft says two specializations are the same when, among other
  things, their template template arguments refer to the same
  template.
- An alias template is a template of its own. A specialization of the
  alias is the same as the type it stands for, but the draft has no
  rule that makes the alias template itself refer to the class
  template, even when it passes its parameters through unchanged.
- So the two uses name different templates as arguments, and the two
  types differ. This is Clang's result and option 1. The draft gives a
  definite answer, so it does not support an unspecified result.
- The doubt is an old open core issue about whether such a
  pass-through alias should count as the class template. GCC and EDG
  follow the idea in that issue. As far as I know, no resolution of it
  has been adopted after the draft, so the compilers in the table do
  not all follow a later change.

### Q-0200 in more detail

- The draft says that a lambda in an alias template gives different
  closure types for different instantiations of the alias, even when
  the lambda does not depend on the parameters. This covers the
  second column of the table, and every compiler agrees there.
- The draft does not say whether two uses of one alias specialization
  with the same arguments are one instantiation or two. A
  specialization of an alias is not a class, function, or variable, so
  the draft's rule that equal specializations denote the same entity
  does not reach it directly.
- The wording about instantiations points toward option 1, since the
  same arguments would give the same instantiation. The draft does not
  settle the point, and Clang's reading of each use as a fresh
  substitution is also possible. I have not checked whether a later
  core issue settles this case.

### Q-0205 in more detail

- The draft lets a member enumeration of a class template have an
  explicit specialization, and it states that instantiating the class
  specialization instantiates only the declaration of a scoped member
  enumeration but the full definition of an unscoped one.
- An example in the draft shows the consequence. After the class
  specialization is instantiated, a scoped member enumeration can
  still be explicitly specialized, and an unscoped one cannot, even
  when the class template only declares it with a fixed base.
- So the three rows of the table come out as Clang and EDG find. The
  declared-only row uses a scoped enumeration, the unscoped defined
  row is not valid, and the scoped defined row is valid after a use.
  GCC's pedantic error is a defect.
- TSP-30 allows a specialization after a use for every member
  enumeration that the class template only declares. The draft does
  not allow that for an unscoped one, so TSP-30 says too much in that
  case. The draft also answers the case that the provisional answer
  leaves open. A scoped enumeration that the class template defines
  can have an explicit specialization after a use of the class
  specialization, as long as nothing has needed the enumeration's own
  definition.

### Q-0246 in more detail

- The draft repeats the lookup of a qualified name that refers to a
  member when its scope is the current instantiation and a specialization is made.
  If the second result differs from the first in any way, the draft
  says the lookup is ambiguous. The draft does not limit the rule to a
  second result that holds more than one member. A single different
  member found the second time counts.
- An ambiguous lookup makes the program ill-formed with a diagnostic
  required. Read this way, the draft supports option 3, and every
  compiler fails to diagnose the program.
- The doubt comes from the general rule on templates. The qualified
  name and the access through the object pointer are not dependent
  here, since lookup at the definition finds a member. The draft makes
  a program ill-formed with no diagnostic required when a construct
  that depends on no template parameter means something different in
  an actual instantiation than at the definition. That rule also
  covers this program. If that rule takes priority, the program is
  ill-formed with no diagnostic required, which is option 1.
- Option 2 has no support in the draft under either reading.
- Clang and EDG reject the form in Q-0230, where the second lookup
  finds two members, and accept this form. So these compilers treat
  only a lookup with two results as ambiguous.

### Q-0247 in more detail

- The draft makes a program ill-formed with no diagnostic required
  when every valid specialization of a variadic template needs an
  empty template parameter pack.
- A base list that expands the same pack twice is valid only for an
  empty pack, since any element would then be a direct base twice.
  So the unused template falls under that rule.
- An implementation may report the error at the definition, as Clang
  and EDG do, and need not, as GCC does. A specialization with a
  non-empty pack, as in the size probe, is invalid and needs a
  diagnostic, and every compiler rejects it.
- The draft supports option 2. The earlier gap review's reading, that
  the error needs a diagnostic at the definition, does not match this
  rule.

### Q-0248 in more detail

- The draft treats an unqualified conversion function name as a
  dependent name when the target type is dependent. Such a name is
  looked up only at the point of instantiation, in the definition
  context and the instantiation context, so lookup can find a member
  of a dependent base.
- An example in the draft shows a class template that derives from a
  base given as a template argument and calls the conversion function
  to a dependent type without the object pointer. The example accepts
  the call and finds the base's function.
- So the draft supports option 2, which is Clang's result. GCC and
  EDG apply the older rule that never looks into a dependent base for
  an unqualified name. The draft's rule came from the rewrite of scope
  and lookup that is already in the draft, and GCC and EDG have not
  followed that change for this form.
- A name with a target type that is not dependent stays unfound in a
  dependent base, as every compiler finds.

### Q-0263 in more detail

- The draft checks the type of an atomic constraint's expression only
  when satisfaction reaches that constraint. A constraint that a short
  circuit skips is never checked by that rule.
- A separate rule in the draft covers this case. An atomic constraint
  that can never be checked successfully, and that no check ever
  reaches, makes the program not valid, with no diagnostic required.
  The right operand in the test has an integer type for every
  argument, so no check could succeed, and the use with int never
  reaches it.
- So the draft agrees that the program is not valid. GCC's acceptance
  conforms, though, since the rule needs no diagnostic. Clang's and
  EDG's rejection at the definition also conforms.
- CNS-9 and CNS-39 are of kind reject and say the implementation
  checks the type at the definition. The draft allows that check but
  does not require it. The same holds for the requires clause of a
  function template, where GCC rejects. That case also falls under the
  rule above and needs no diagnostic.

### Q-0269 in more detail

- The draft turns each placeholder parameter into an invented type
  parameter of a function template. An example in the draft treats
  the declaration with placeholders and the template written out by
  hand as two declarations that accept the same arguments, yet are
  distinct.
- The draft has two rules for templates in that relation. A program
  whose outcome hinges on two constructs being the same, when they
  only accept the same arguments, needs no diagnostic and is not
  valid. Two function templates that do not correspond, yet share a
  name and signature and accept the same argument lists, likewise make
  the program not valid with no diagnostic required.
- So the draft treats the two declarations as two templates, which is
  option 2, and the program in the test needs no diagnostic. GCC's and
  EDG's ambiguity error conforms, and so does Clang's acceptance.
  Option 1, one template, is not the draft's reading.
- The doubt is that the statement that the two forms differ sits in
  an example. The normative text does not say whether the invented
  head counts as the same head for redeclaration. The example
  is the only direct statement. The unconstrained form follows the same
  reasoning.

### Q-0273 in more detail

- The C++23 draft has no rule that each parameter of a partial
  specialization must appear where deduction can find it. The limits
  on a partial specialization's argument list are that it is more
  specialized than the primary template, that no parameter has a
  default, that every pack is expanded, and that an expansion is the
  last argument.
- The partial specialization in the test is more specialized than the
  primary template. Deduction of the primary from the partial
  specialization's argument succeeds. Deduction the other way fails,
  because the inner argument list is a non-deduced position and its
  parameters get no values. So the declaration meets the draft's
  limits.
- When the class template is used, matching deduces nothing from the
  inner list, the last parameter has no value, and the match fails. So
  the specialization is valid and never matches, which is GCC's and
  EDG's result and option 2.
- The doubt is the draft's general rule that a template for which no
  valid specialization can be made is not valid, with no diagnostic
  required. A partial specialization that can never match may fall
  under that rule. Read that way, Clang's rejection is allowed, though
  not required. TSP-11's bullet about where each parameter appears has
  the same status in the draft, including the case of an expression of
  a parameter.
- For the function type, the draft makes a function parameter pack
  that is not at the end of the parameter list a non-deduced position.
  A trailing template parameter pack that deduction does not fill is
  empty. With no explicit arguments, the pack is empty, the parameter
  type becomes a pointer to a function of one int, and only the
  function with that type matches. With an explicit char, the pack
  holds char and the first function matches. This is Clang's result in
  all three rows. EDG's fallback for the second row and GCC's failure
  for the third row are defects. This agrees with option 1.

### Q-0287 in more detail

- The draft lists the steps of a compound requirement in this order.
  First the expression is substituted. Then, with the no-throw marker,
  the expression must not be potentially throwing. Then the
  return-type requirement is substituted, and last the type
  constraint's immediately-declared constraint must be satisfied. A
  failed step makes the requirement false, and the draft's general
  rule says checking stops at the first condition that decides the
  result.
- So the order matches option 1, and Clang's failed static assertion
  in the order case is a defect.
- For the unused argument, the step list says the whole return-type
  requirement is substituted, which includes every extra argument.
  That reading gives false, as EDG and Clang find.
- The doubt comes from an example in the draft that gives a compound
  requirement as equal to a simple requirement plus a nested
  requirement on the concept with the extra arguments. In a nested
  requirement, the concept is normalized, and an argument that the
  concept's constraint never uses has no place in the parameter
  mapping, so it would not be substituted. That reading gives GCC's
  true. The normative steps favor option 1, and the example points to
  option 3.

### Q-0295 in more detail

- The draft decides whether a partial specialization matches by deducing its arguments from the actual argument list. Deduction fails when a constant in an argument list has a type other than the type of the matching parameter of the named template. A worked example rejects a constant of type int deduced into a parameter declared short.
- In this question the primary template's parameter has a placeholder type. The draft does not say clearly what type the rule compares with in that case. The natural reading takes the type of the actual argument, which is long or char in the second row. That type differs from int, so the match fails, which is option 1.
- For a parameter whose type contains a placeholder, as in the third row, the draft deduces the placeholder from the type of the value. A pointer to int against a pattern of pointer to const placeholder does not match exactly. The draft is not clear on whether the added const counts as a mismatch. GCC, EDG, and Clang 18 reject the match, and Clang 22 accepts it.
- The draft's text does not settle these placeholder cases, so I mark the answer as agreeing with doubt.
