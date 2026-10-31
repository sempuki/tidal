<!--
Behaviour inventory, 2026-10-31: overload resolution (roadmap item M,
decision 0044).
Written by: a source reader under AGENTS.md that read the C++ working
draft sources and Tidal's own pages. It did not write or change any file
in this repository; the integrator copied this summary here.
Checked before use: the source reader ran tools/similarity.py on its
summary outside the repository and reworded two flagged passages before
returning it. The integrator's own check gives 0 flagged. No code, no
quotes, no section names. Each item is a behaviour to probe, not a rule.
-->

Role: source reader

# Behaviour inventory for item M (overload resolution)

Role: source reader. All text below is my own description of language
behaviour. Items marked "covered" name the Tidal rule that already
states the behaviour, fully or in part.

## 1. Categories for a probe grid

Call context (where the set of candidates comes from):

- call by a plain name to free functions,
- call by a plain name to class members from inside a member function,
- call by a plain name to class members where no current object exists
  (a static member function, for example),
- call through a dot or arrow on an object,
- call on a class object (call operators and conversions to a callable
  pointer or reference),
- call whose callee is the address of an overload set,
- operator in an expression: unary, binary, assignment, subscript,
  arrow, postfix increment, equality, relational, three-way, the
  conditional operator with class operands,
- construction: direct, default, copy from the same or a derived class,
- copy initialization of a class from some other type,
- initialization of a non-class object from a class value,
- direct binding of a reference to the result of a conversion function,
- list initialization of a class,
- taking the address of an overload set against a target type.

Kind of candidate:

- plain function, function template specialization,
- member with an implicit object parameter: no qualifier, a const or
  volatile qualifier, an lvalue qualifier, an rvalue qualifier,
- member with an explicit object parameter, static member,
- constructor: converting, explicit, explicit with a condition that
  depends on a template parameter, inherited from a base,
- conversion function: plain, explicit, template,
- surrogate made from a conversion to a callable pointer or reference,
- built-in operator candidate,
- rewritten comparison in the written order, rewritten comparison with
  the operands swapped,
- deleted function, constrained function.

Viability factor:

- argument count equal to, less than, or greater than the parameter
  count,
- trailing defaults, an ellipsis, a parameter pack,
- constraints satisfied or not,
- an argument with no conversion sequence, a reference that cannot bind.

Kind of tie and of tie-breaker:

- one function better on every argument, crossing arguments,
- equal sequences settled by the result conversion of a conversion
  function, by the reference kind of a function reference, by plain
  over template, by the more specialized template, by the more
  constrained function, by a derived constructor over an inherited one,
  by written over rewritten, by written order over swapped order,
- equal in every way (ambiguous).

Outcome:

- one best function, ambiguous, no viable function, best function
  deleted, best function not accessible, best function with a faulty
  sequence, default argument found in two scopes.

## 2. Behaviours

### Address of an overload set (AO)

- AO-1 Callee address: when the callee of a call is the address of an
  overload set, resolution runs on that set with no implied object
  argument. If the winner is a member with an implicit object
  parameter, the call is an error. A member with an explicit object
  parameter takes the object as an ordinary first argument.
- AO-2 Class of the member: for a target of pointer-to-member-function
  type, only the function type has to match. The class named in the
  target is not compared with the class that declares the member.
- AO-3 Constraints: functions whose constraints are not satisfied drop
  out first. Then a plain function that another plain function is more
  constrained than drops out.
- AO-4 Exact type: a plain function matches only when its type equals
  the function type of the target. The only adjustment allowed is
  dropping the no-throw property. No base conversion of a parameter or
  of the result is allowed, and no ellipsis matching.
- AO-5 Member kinds: free functions, static members, and members with an
  explicit object parameter match pointer-to-function and
  reference-to-function targets. Members with an implicit object
  parameter match only pointer-to-member-function targets.
- AO-6 No target: when the context gives no target type, only plain
  functions are taken, and exactly one must remain.
- AO-7 Overload set as argument: if the called name and an argument are
  both overload sets, every pairing is considered, and one pairing must
  win.
- AO-8 Parenthesized member name: taking the address of a qualified
  member name inside parentheses gives an ordinary pointer, so the form
  can reach a static member but never forms a member pointer.
- AO-9 Plain beats template: if any plain function remains after the
  constraint step, every template specialization drops out.
- AO-10 Target kinds: a target type can come from an initialized
  variable or reference, the left side of an assignment, a parameter of
  a function or of an operator function, a return value, an explicit
  cast, or a non-type template parameter.
- AO-11 Templates: each named template contributes at most one
  specialization, deduced from the target. Among specializations, one
  from a more specialized template removes the others.
- AO-12 Uniqueness: after all removals exactly one function must remain,
  or the program is not valid.

### Ambiguity and failure (AF)

- AF-1 Access after choice: access is checked only on the winner. An
  accessible loser never rescues the call (covered: ACC-3).
- AF-2 Ambiguity: if no single viable function beats every other viable
  function, the call is not valid, even when one function beats some.
- AF-3 Crossing arguments: a function better for one argument and worse
  for another is not better overall. Two such functions tie.
- AF-4 Default from two scopes: if the winner was found through
  declarations in different scopes and more than one of them supplies a
  default argument that the call relies on, the call is not valid. A
  call that passes that argument itself is fine.
- AF-5 Empty sets: no candidate at all, or no viable candidate, makes
  the call not valid (covered for operators: OPR-15).
- AF-6 Faulty winner: a winner whose sequence carries a fault (narrowing
  in braces, ambiguous base, and similar) makes the call not valid
  (covered: ICS-8).
- AF-7 Identical surrogates: two surrogate candidates with the same
  parameter list cannot be told apart. A call that needs one of them is
  ambiguous unless a third candidate beats both.
- AF-8 Usable result: some language rules ask whether resolution yields
  a usable function. That means resolution succeeds and the winner is a
  built-in candidate, or a function that is neither deleted nor
  inaccessible from the context.

### Built-in operator candidates (BI)

- BI-1 Arithmetic pairs: for every pair of floating or promoted integer
  types there are candidates for multiply, divide, add, subtract, and
  the six comparisons. The arithmetic result type is the common type of
  the usual arithmetic conversions.
- BI-2 Assignment family: for every arithmetic left type (plain or
  volatile, taken by reference) and every floating or promoted integer
  right type, there are candidates for plain assignment and for the
  multiply, divide, add, and subtract compound forms.
- BI-3 Conditional operator: for every pair of floating or promoted
  integer types, and for every pointer, member pointer, or scoped
  enumeration type used for both branches, there is a candidate with a
  boolean first operand (covered in part: COND-11).
- BI-4 Dereference and unary plus on pointers: every object type and
  every unqualified function type has a dereference candidate from a
  pointer. Every pointer type has a unary plus candidate.
- BI-5 Duplicate removal: a built-in candidate drops out when a
  non-template free operator, or a rewritten free operator, has the
  same parameter list.
- BI-6 Enumerations and pointers compared: for each enumeration type and
  each pointer type there are candidates for the six comparisons and
  three-way comparison with both operands of that type.
- BI-7 Increment and decrement: for every arithmetic type except the
  boolean type, and every pointer to an object type, there are prefix
  and postfix candidates on a reference to the plain or volatile type.
- BI-8 Integer-only operators: remainder, the three bitwise operators,
  and the two shifts have candidates for every pair of promoted integer
  types. A shift's result has the left type. The compound forms take an
  integer left reference and a promoted integer right operand.
- BI-9 Left operand of assignment: for built-in assignment candidates
  the left operand gets no temporary and no user-defined conversion
  (covered: ICS-10).
- BI-10 Logical operators: negation, logical and, and logical or have
  candidates on boolean operands.
- BI-11 Member pointer access: for a pointer to a class and a member
  pointer of that class or a base of it, there is a candidate for the
  arrow-star operator.
- BI-12 Member pointers and the null pointer type: these have equality
  and inequality candidates only.
- BI-13 No built-ins for three operators: the comma, unary address-of,
  and arrow have no built-in candidates. If those operators find no
  viable function, they take their built-in meaning.
- BI-14 Pointer arithmetic: every object type has candidates for
  pointer plus or minus the pointer difference type, the integer plus
  the pointer, and subscript in both operand orders. Two pointers of the
  same object type have a subtraction candidate.
- BI-15 Pointer assignment: every pointer type has an assignment
  candidate. Pointers to object types also have add and subtract
  compound candidates. Enumeration and member pointer types have
  assignment candidates.
- BI-16 Selection then built-in check: when a built-in candidate wins,
  a class operand goes through its conversion function but not the
  standard steps after it. The operator is then checked by the ordinary
  built-in rules, which can still reject it.
- BI-17 Three-way on numbers: each integer type has a strong ordering
  candidate with both operands of that type. Each pair of floating
  types has a partial ordering candidate.
- BI-18 Unary arithmetic: unary plus and minus exist for floating and
  promoted integer types. Bitwise complement exists for promoted integer
  types. An unscoped enumeration reaches these through promotion.

### Candidates in calls (CC)

- CC-1 Arrow call: a member call through an arrow behaves as a
  dereference followed by a dot call.
- CC-2 Call operators: a call on a class object takes the class's call
  operators as candidates, with the object as the implied argument.
- CC-3 Contrived object: an unqualified call to members where no current
  object exists still resolves. If the winner is a non-static member,
  the call is not valid. A static winner is fine.
- CC-4 Current object: inside a non-static member function, an
  unqualified call that finds members adds the current object as the
  implied object argument.
- CC-5 Explicit object member called by plain name: the current object
  fills the explicit object parameter. Passing the object again as an
  extra argument finds no viable function.
- CC-6 Lookup decides the kind: an unqualified call finds either only
  free functions or only members of one class, never a mix.
  Argument-dependent lookup adds free functions (covered: NAM-5,
  NAM-27).
- CC-7 Member call: a dot call takes the functions that member lookup
  finds, plus the object as the implied argument. The const and volatile
  qualifiers of the object count, also for a class prvalue.
- CC-8 Surrogate candidates: each non-explicit conversion function of
  the object's class to a pointer to function, a reference to such a
  pointer, or a reference to function adds a surrogate candidate. Its
  first parameter is the conversion's target type. The rest are the
  function type's parameters. The conversion function must be at least
  as qualified as the object (covered in part: CVF-2).
- CC-9 Surrogates from bases: conversion functions in base classes add
  surrogates too, unless a declaration in the class hides them.
- CC-10 Surrogate object match: for a surrogate, the object converts to
  the first parameter through the conversion function. For a call
  operator, the object meets the object parameter.
- CC-11 Template candidates: each function template found adds at most
  one specialization by deduction. A failed deduction removes the
  template quietly. Specializations join the plain functions.

### Candidates in initialization (CI)

- CI-1 Class from another class: copy initialization of a class from a
  different type takes the converting constructors of the target class,
  plus the non-explicit conversion functions of the source that yield
  the target class or a class derived from it.
- CI-2 Construction contexts: direct and default initialization take
  every constructor. Copy initialization from the same or a derived
  class takes only the converting constructors (covered: CTOR-9).
- CI-3 Dependent explicit: for a constructor or conversion template
  whose explicitness depends on a template parameter, deduction runs
  first. The specialization then drops out if it turns out explicit in a
  context that allows only non-explicit candidates.
- CI-4 Explicit conversion into a copy constructor parameter: in direct
  initialization of a class from one argument, the temporary for the
  reference parameter of a copy or move constructor can also come from
  an explicit conversion function of the source.
- CI-5 Hidden or templated conversion functions: a conversion function
  that a derived class hides is not a candidate. A conversion template
  is found by searching for a conversion to the target type (covered in
  part: CVF-5, ICS-5).
- CI-6 List initialization: two phases, initializer-list constructors
  first (covered: ILC-2, ILC-3). An explicit winner in copy list
  initialization is an error (covered: ILC-8).
- CI-7 Non-class target: conversion functions whose result reaches the
  target by a standard sequence are candidates. In direct
  initialization, explicit ones are too when only a qualification
  adjustment separates result and target (covered: ICS-4, CVF-3).
- CI-8 One argument: in each initialization context the argument list
  is the single initializer. A constructor matches it against its
  leading parameter, and a conversion function against its object.
- CI-9 Qualified results: when an object is initialized, a conversion
  that yields a cv-qualified version of an allowed type, or a reference
  to one, counts as yielding that type.
- CI-10 Reference target: for direct binding, conversion functions that
  return an lvalue reference serve an lvalue reference. Conversions
  returning a value or an rvalue reference serve an rvalue reference.
  A reference to function accepts both. The referred type must be
  reference-compatible with the target (covered in part: ICS-4).

### Candidates for operators (CO)

- CO-1 Arrow chain: the member name after an arrow plays no part in
  choosing the arrow operator, and the arrow repeats on a class result
  (covered: OPR-5).
- CO-2 Built-in only for plain operands: if no operand has class or
  enumeration type, no overload resolution happens.
- CO-3 Enumeration-only operands: when no operand is a class, a free
  operator is a candidate only if a parameter in the matching position
  is that enumeration type or a reference to it.
- CO-4 Expression lookup differs from call lookup: a member operator of
  the enclosing class hides a free operator for an explicit call by the
  operator's name, but not for the operator expression.
- CO-5 Member set: member candidates come from the left operand's class
  if the class is complete or being defined (covered: OPR-6).
- CO-6 No free candidates for three operators: assignment, subscript,
  and arrow take only members (covered: OPR-2).
- CO-7 Free set: free operator candidates come from unqualified and
  argument-dependent lookup with members ignored (covered: OPR-6).
- CO-8 Postfix form: postfix increment and decrement are called with an
  extra zero argument (covered: OPR-4).
- CO-9 Result of a rewritten three-way: a relational expression that
  picks a rewritten three-way candidate compares its result against
  zero. With swapped operands, zero is the left side. No further
  rewriting happens in that inner expression.
- CO-10 Result of a rewritten equality: an inequality that picks an
  equality candidate negates its result. A swapped equality calls the
  candidate with the operands swapped. The chosen equality must return
  a possibly qualified boolean type (covered: DCMP-12).
- CO-11 Rewrite target check: an equality operator is not used for
  rewriting if a search finds a matching inequality operator. The
  search is in the operand's class for a member, and in the operator's
  own namespace for a free function. A template's specializations follow
  the template (covered in part: DCMP-6).
- CO-12 Rewritten sets: inequality also tries equality candidates.
  Equality tries equality with swapped operands. Relational operators
  try three-way candidates in both orders. Three-way tries itself with
  swapped operands (covered in part: DCMP-6).
- CO-13 Swapped member: a swapped candidate made from a member has the
  object as its second parameter. The first operand can be converted,
  the second cannot.
- CO-14 Union of sets: the candidates are members, free operators,
  built-ins, and rewritten candidates together. A user operator beats
  built-in use through a conversion when its sequences are as good.

### Deleted and excluded functions (DE)

- DE-1 Deleted participates: a deleted function competes normally. If it
  wins, the call is not valid (covered: DEL-1).
- DE-2 Defaulted move removed: a defaulted move constructor or move
  assignment that is defined as deleted is not a candidate anywhere. An
  rvalue falls back to copying (covered: CPY-7, CPY-23).
- DE-3 Inherited constructor removed: when a derived object is built
  from exactly one argument, an inherited constructor drops out if its
  first parameter is a reference to a type related to both the base
  and the derived class. Inherited copy and move constructors, and
  forwarding templates of the same shape, never take over a derived
  copy.

### Object parameter (OP)

- OP-1 Explicit object parameter: the parameter is ordinary, so a
  user-defined conversion can reach it (covered: ICS-13).
- OP-2 Implicit object parameter type: lvalue or rvalue reference to the
  class, with the member's qualifiers (covered: ICS-13).
- OP-3 Inherited by a using-declaration: the object parameter refers to
  the derived class (covered: ICS-13).
- OP-4 Member of the object's class for conversions: a conversion
  function counts as a member of the object's class (covered: ICS-13).
- OP-5 No user-defined conversion to the implicit object parameter
  (covered: ICS-10).
- OP-6 Rvalue object without a qualifier: an rvalue binds to a member
  without a reference qualifier, and the rvalue gives no ranking edge
  (covered: ICS-13, ICS-28).
- OP-7 Static member slot: a static member in a member call has a slot
  that any object fills, with no sequence to rank (covered: ICS-14).

### Ranking of viable functions (RK)

- RK-1 Base test: one function beats another only if none of its
  argument sequences is worse. Then the first of the following steps
  that applies decides.
- RK-2 Better argument: some argument has a better sequence (covered in
  detail: ICS-21 to ICS-36).
- RK-3 Conversion result: in an initialization by a user-defined
  conversion, the function whose result converts to the target by a
  better standard sequence wins (covered in part: ICS-37).
- RK-4 Function reference kind: when a reference to a function binds
  directly to a conversion result, the conversion returning the same
  reference kind (lvalue or rvalue) as the target wins.
- RK-5 Plain over template: a plain function beats a template
  specialization with equal sequences.
- RK-6 More specialized: of two template specializations, the one from
  the more specialized template wins.
- RK-7 More constrained: of two plain functions with the same parameter
  lists, the more constrained one wins.
- RK-8 Derived constructor over inherited: a constructor of the derived
  class beats an inherited base constructor when every used parameter
  has the same type.
- RK-9 Written over rewritten: a candidate that is not rewritten beats a
  rewritten one (covered in part: DCMP-6).
- RK-10 Written order over swapped order: of two rewritten candidates,
  the one in the written operand order beats the swapped one.
- RK-11 Deduction-guide steps: class template argument deduction has its
  own later steps (guides over constructors, the copy guide, plain over
  template constructors, inherited guides). These belong outside item M
  or in a separate item.
- RK-12 Argument sequences first: a template with an exact match beats
  a plain function that needs a conversion. The template-related steps
  apply only when sequences give no winner.
- RK-13 Defaults do not rank: parameters left to default arguments take
  no part in the comparison. Two functions equal on the given arguments
  tie, whatever their defaults.
- RK-14 Ellipsis worst: an argument matched by an ellipsis has the worst
  kind of sequence (covered: ICS-6, ICS-21).
- RK-15 Order independence: the winner does not depend on declaration
  order. A function must beat each other viable function one on one.
- RK-16 Rewritten swapped tie: a written equality that needs a
  qualification step on one argument can tie with its own swapped
  version, so a comparison of two equal class values becomes ambiguous
  (covered as a warning: DCMP-7).
- RK-17 Static slot neutral: the slot of a static member neither helps
  nor hurts in the comparison (covered: ICS-14).

### Viability (VI)

- VI-1 Constraints: a candidate with unsatisfied constraints is not
  viable.
- VI-2 Count equal: a candidate with exactly as many parameters as
  arguments passes the count test.
- VI-3 Count lower with ellipsis: a candidate with fewer parameters is
  viable only with an ellipsis. The extra arguments meet the ellipsis
  (covered: PFX-5).
- VI-4 Count higher with defaults: a candidate with more parameters is
  viable only if all the extra ones have defaults. The comparison then
  ignores them (covered: PFX-5).
- VI-5 Each argument converts: every argument needs some implicit
  conversion sequence to its parameter.
- VI-6 Faults ignored here: access, deletion, and the sequence faults of
  ICS-8 do not affect viability (covered: ICS-8, DEL-1, ACC-3).
- VI-7 Object argument counts: the implied object argument is checked
  like any other argument against the object parameter.
- VI-8 Pack absorbs: a template with a function parameter pack adapts
  its parameter count to the arguments by deduction.
- VI-9 Reference failures: a modifiable lvalue reference given an
  rvalue, or an rvalue reference given an lvalue,
  has no sequence, so the candidate is not viable (covered: ICS-12).

## 3. Gaps and doubts

- G-1 Static and non-static members together: a call with no object
  context that finds a static member with a default argument and a
  non-static member with no parameters is ambiguous by the draft. The
  compilers may differ, mostly for members with explicit object
  parameters.
- G-2 Address of explicit object members: forming a plain function
  pointer from a member with an explicit object parameter, and calling
  through the address of a mixed overload set, are recent features.
  Support may be partial.
- G-3 Rewrite target search: compilers may differ on where they search
  for a matching inequality operator, especially with inline namespaces,
  templates, and friends defined in a class.
- G-4 Swapped tie: some compilers accept a comparison where the written
  and swapped candidates tie, with a warning, and others reject it
  (see DCMP-7).
- G-5 More constrained with different object parameters: members that
  differ in reference qualifiers or in the explicit object form may not
  count as having the same parameter lists. Compilers may disagree.
- G-6 Function reference kind: the step that prefers a conversion
  returning the matching reference kind for a function reference is
  rarely tested. Compilers may ignore it.
- G-7 Inherited constructor removal: older compilers may still consider
  inherited copy or move shapes and report ambiguity.
- G-8 Default argument from two scopes: some compilers may not diagnose
  the case where two scopes give the default that the call uses.
- G-9 Built-in removal by a same-shaped free operator: probe an
  enumeration operand with a free operator that matches a built-in
  shape exactly, and one that differs only by a reference.
- G-10 Surrogate qualifiers: compilers may differ on whether a const
  object can use a non-const conversion to a function pointer, and on
  surrogates from base classes.
- G-11 No target: an overload set with one plain function and one
  template, used where no target type exists, keeps only the plain
  function. Compilers may differ on deduction contexts.
- G-12 Dependent explicit: the order of deduction and the explicit check
  may show in error messages or in which candidate is removed.
- G-13 Unary address-of, comma, and arrow fallback: the built-in meaning
  applies when no function is viable. A class with only a deleted or
  inaccessible operator still selects that operator.
- G-14 Usable result: whether a defaulted special member becomes
  deleted depends on the usable test. Compilers may treat an
  inaccessible or deleted base function differently.
- G-15 Constraints and templates together: for two template
  specializations, the more specialized step already folds in
  constraints. Probe constrained templates with identical heads.

## 4. Already covered

ACC-3, COND-11, CPY-7, CPY-23, CTOR-9, CVF-2, CVF-3, CVF-4, CVF-5,
CVF-12, DCMP-6, DCMP-7, DCMP-12, DEL-1, ICS-1 to ICS-37 (the conversion
sequences, the object parameter, and the per-argument comparison),
ILC-2, ILC-3, ILC-8, NAM-5, NAM-25, NAM-27, OPR-2, OPR-4, OPR-5, OPR-6,
OPR-15, PFX-5.

Not covered: the AO group, AF-2 to AF-4, AF-7, AF-8, the BI group
except BI-3 and BI-9, CC-1 to CC-5, CC-9 to CC-11, CI-1, CI-3, CI-4,
CI-8 to CI-10 in full, CO-2 to CO-4, CO-9, CO-13, CO-14, DE-3, RK-1,
RK-4 to RK-8, RK-10 to RK-13, RK-15, VI-1, VI-2, VI-5, VI-7, VI-8.
