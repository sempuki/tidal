<!--
Behaviour inventory, 2026-10-31: concepts and constraints (roadmap item
P).
Written by: a source reader under AGENTS.md that read the C++ working
draft sources and Tidal's own pages. It did not write or change any file
in this repository; the integrator copied this summary here.
Checked before use: the source reader ran tools/similarity.py on its
summary outside the repository; the integrator's own check gives 0
flagged. No code, no quotes, no section names. Each item is a behaviour
to probe, not a rule.
-->

Role: source reader

# Inventory for item P: concepts and constraints

## 1. Categories for a probe grid

- **Constrained entity.** Alias template, class template, class template
  partial specialization, constructor or other special member, destructor,
  friend function, function template, lambda call operator, member function
  of a class template that is not itself a template, member template,
  template template parameter, variable template.
- **Constraint shape.** A single concept with arguments; conjunction;
  disjunction; logical negation; a fold over a pack; a nested concept that
  refers to another concept; a requires expression; a plain constant
  expression such as a comparison or a call.
- **Context that checks the constraint.** Address of an overload set;
  call through overload resolution; constant evaluation of a concept name
  with arguments; deduction of a placeholder type; destructor selection;
  explicit instantiation; explicit specialization; naming a class, variable,
  or alias specialization; partial specialization matching; special member
  eligibility; template template argument matching.
- **Outcome.** Satisfied; not satisfied through a false value; not satisfied
  through substitution failure; hard error (wrong type or not constant);
  ill-formed with no diagnostic required.
- **Ordering relation between two declarations.** Same constraints; one
  subsumes the other; neither subsumes; textually equal but separate
  expressions; same body in two different concepts.
- **Spelling of the constraint.** Concept before a type parameter; concept
  before a placeholder in a function parameter; concept before a placeholder
  in a variable, return type, or constant template parameter; clause after
  the template parameter list; clause after the function declarator; return
  type check inside a compound requirement; nested requirement.

## 2. Behaviours

### A. Concept definitions

- **A1 Attributes allowed.** A concept definition may carry attributes, and
  they belong to the concept.
- **A2 Constant boolean result.** A concept named with a full argument list
  is a constant prvalue of the boolean type. Its value is true exactly when
  the arguments satisfy the concept's defining expression after
  normalization. It works in a static assertion, a constexpr if, and a
  template argument.
- **A3 Defining expression is unevaluated.** Nothing in the defining
  expression is odr-used or evaluated beyond what satisfaction needs.
- **A4 Full expression grammar in the definition.** The defining expression
  can be any logical-or expression without extra parentheses. The restricted
  grammar applies only to clauses.
- **A5 Namespace scope only.** A concept defined inside a class or a block
  is an error.
- **A6 No constraints on a concept.** A concept's own template head cannot
  carry a clause or a constrained parameter.
- **A7 No self-reference.** The concept's name is not yet visible inside its
  own definition, so a concept cannot refer to itself.
- **A8 No specialization or instantiation.** A concept cannot be explicitly
  instantiated, explicitly specialized, or partially specialized. A concept
  name with arguments names no entity and no specialization.
- **A9 Parameter kinds.** A concept can take type, constant, and template
  parameters and packs. A concept whose first parameter is a type parameter
  is a type concept.

### B. Clauses

- **B1 Attribute ambiguity on lambdas.** When a lambda's clause could end in
  attributes, the attributes go to the lambda.
- **B2 Clause is unevaluated.** The expression of a clause is never
  evaluated except to decide satisfaction.
- **B3 Lambda clauses.** A lambda with an explicit template parameter list
  can carry a clause after that list and a trailing clause after its
  declarator. Both constrain the call operator.
- **B4 Leading clause position.** A clause can follow the template parameter
  list of any template declaration.
- **B5 Negation needs parentheses.** A logical negation in a clause must be
  written inside parentheses.
- **B6 Nested keyword use.** A clause whose operand is a requires expression
  repeats the keyword, once for the clause and once for the expression.
- **B7 No trailing clause on non-functions.** A variable, a function pointer
  declarator, a parameter, and the type in a new expression cannot carry a
  trailing clause.
- **B8 No trailing clause on non-templated functions.** A function that is
  neither a template nor a member of a templated entity cannot carry a
  trailing clause.
- **B9 Non-template friend with a clause must be a definition.**
- **B10 Restricted operand grammar.** In a clause, each operand of the top
  level and-operator or or-operator must be a primary expression. A
  comparison, a call, a cast, or arithmetic needs parentheses.
- **B11 Trailing clause after the trailing return type.** A trailing clause
  placed before a trailing return type is an error.
- **B12 Virtual functions cannot be constrained.** A virtual function with a
  trailing clause is an error.

### C. Type constraints and placeholders

- **C1 Abbreviated function template.** A function parameter whose type
  contains a placeholder preceded by a concept makes the function a template.
  The invented type parameter carries that concept.
- **C2 Abbreviated pack.** A constrained placeholder in a function parameter
  pack invents a constrained type parameter pack.
- **C3 Compound requirement sees the declared type of the parenthesized
  expression.** The return type check in a compound requirement passes the
  type of the expression with its value category folded in: an lvalue gives
  an lvalue reference type, an xvalue gives an rvalue reference type, a
  prvalue gives the plain type.
- **C4 Concept with extra arguments.** A concept written before a parameter
  with its own argument list receives the parameter as its first argument
  and the listed arguments after it.
- **C5 Constant template parameter with a constrained placeholder.** The
  deduced type of the argument must satisfy the concept.
- **C6 Constrained decltype placeholder.** A concept can precede the
  decltype form of the placeholder. The concept sees the type deduced by
  the decltype rules, references included.
- **C7 Constrained pack parameter.** A concept before a type parameter pack
  must hold for each element separately. An empty pack satisfies it.
- **C8 Constrained return type.** A function whose return type is a
  constrained placeholder is an error when the deduced return type fails
  the concept.
- **C9 Constrained variable.** A variable declared with a constrained
  placeholder is an error when the deduced type fails the concept.
- **C10 Placeholder check sees the deduced part only.** When the placeholder
  sits inside a larger declared type (a reference or a pointer, with
  qualifiers), the concept checks the type deduced for the placeholder
  itself, not the whole declared type.
- **C11 Qualified concept names.** The concept before a parameter or
  placeholder may be named with a namespace qualifier.
- **C12 Type concepts only.** A concept whose first parameter is not a type
  parameter cannot be written before a type parameter or a placeholder.

### D. Requires expressions

- **D1 Always failing requirement.** A requirement that fails for every
  possible argument makes the program ill-formed with no diagnostic
  required.
- **D2 Evaluation order and stop.** Requirements are checked in the order
  written, and checking stops at the first one that fails.
- **D3 Failure gives false inside a template.** An invalid type or
  expression in a requirement, after substitution, gives the value false
  and is not an error.
- **D4 Invalid outside a template is an error.** A requires expression in a
  non-templated context with an invalid requirement makes the program
  ill-formed.
- **D5 Local parameters.** The optional parameter list introduces names
  with no storage and no lifetime. The parameters cannot have default
  arguments, and the list cannot end in an ellipsis.
- **D6 Nested requirement.** A nested requirement holds when its constraint
  expression is satisfied. It can use the local parameters, and its
  operands are checked by the satisfaction rules (with short circuit).
- **D7 Nested requirement keyword wins.** A requirement that starts with the
  keyword is always a nested requirement, never a simple one.
- **D8 Noexcept check.** A compound requirement marked noexcept fails when
  its expression can throw.
- **D9 Prvalue of boolean type.** The requires expression is a constant
  boolean prvalue and can appear anywhere a constant expression can.
- **D10 Return type check.** A compound requirement with a concept after the
  arrow checks the concept on the expression's type (C3), after the
  expression itself is valid.
- **D11 Simple requirement.** A simple requirement holds when its expression
  is valid. The expression is not evaluated.
- **D12 Type requirement.** A type requirement holds when the named type
  can be formed. A nested member type, a valid class template
  specialization, and a valid alias template substitution all count.
- **D13 Type requirement needs no complete type.** A class template
  specialization named in a type requirement is not instantiated, so it
  may stay incomplete.
- **D14 Unevaluated requirements.** Expressions inside the body are
  unevaluated operands.

### E. Normalization and atomic constraints

- **E1 Concept names expand.** A concept named with arguments inside a
  constraint is replaced by its own normalized expression, with its
  parameters mapped to the arguments.
- **E2 Identity of atomic constraints.** Two atomic constraints are the same
  only when they come from one source appearance of one expression and their
  parameter mappings give the same arguments. Two copies of the same text in
  two declarations are different atomic constraints.
- **E3 Invalid mapping at normalization.** If expanding a concept forms an
  invalid type or expression in a parameter mapping, the program is
  ill-formed with no diagnostic required.
- **E4 Lazy validity.** An atomic expression that is invalid for some
  arguments does not make normalization fail. Only the mapping must be
  valid.
- **E5 Logical operators split.** At the top level of a constraint and
  through concept expansion, the and-operator makes a conjunction and the
  or-operator makes a disjunction. Parentheses alone do not stop the split.
- **E6 Negation is atomic.** A logical negation is one atomic constraint,
  including a negated concept name. The negation operator is not a logical
  operation on constraints.
- **E7 Order of the combined constraints.** A declaration's constraints are
  one conjunction in this order: the concepts before template parameters,
  then the leading clause, then the concepts in placeholder function
  parameters, then the trailing clause. The order fixes the order of
  satisfaction checks.
- **E8 Other expressions are atomic.** Any other expression is one atomic
  constraint. That includes comparisons, calls, casts, requires
  expressions, and a fold over a pack. An and-operator hidden inside such an
  expression does not split.
- **E9 Same constraint from three spellings.** A concept before the type
  parameter, the same concept in a leading clause, and the same concept in
  a trailing clause give the same associated constraint.

### F. Satisfaction

- **F1 Atomic must be constant.** An atomic expression that is not a
  constant expression is a hard error.
- **F2 Atomic needs exact boolean type.** After substitution, an atomic
  expression must be a constant expression of exactly the boolean type. An
  integer, or a class with a conversion to the boolean type, makes the
  program ill-formed. The error stands even if another candidate would win.
- **F3 Conjunction short circuit.** If the left operand of a conjunction is
  not satisfied, the right operand is never substituted, so errors in it do
  not arise.
- **F4 Disjunction short circuit.** If the left operand of a disjunction is
  satisfied, the right operand is never substituted.
- **F5 Lvalue to rvalue.** An atomic expression that names a boolean
  variable or constant is read as a value.
- **F6 Negation does not rescue a substitution failure.** When substitution
  into a negated atomic expression fails, the constraint is not satisfied.
  The negation does not turn the failure into true.
- **F7 Satisfaction must be stable.** If identical atomic constraints with
  identical arguments give different results at two points in the program
  (for example, before and after a class is completed), the program is
  ill-formed with no diagnostic required.
- **F8 Substitution failure means not satisfied.** If substituting into an
  atomic expression forms an invalid type or expression, that atomic
  constraint is false. It is not an error.
- **F9 Wrapping in a concept localizes failure.** A substitution failure
  inside a concept makes that concept false, so negating the concept gives
  true. The same text written inline under a negation gives not satisfied
  (F6).

### G. Where constraints are checked

- **G1 Address of an overload set.** Functions with unsatisfied constraints
  drop out when an overload set's address is taken.
- **G2 Alias, class, and variable specializations.** Naming a specialization
  of a constrained alias, class, or variable template with non-dependent
  arguments requires satisfaction. Failure is an error.
- **G3 Alias deduction guide constraint.** A deduction guide made from an
  alias template's underlying guides carries the original guide's
  constraints joined with a check that the alias arguments are deducible.
- **G4 Calls.** A candidate with unsatisfied constraints is not viable.
- **G5 Checked after deduction, before the function type.** For a function
  template, constraints are checked once all template arguments are known
  and before substitution into the function type and the conversion check of
  non-dependent parameters. A failed constraint ends deduction, so errors in
  the return type are never reached.
- **G6 Class instantiation keeps constrained members.** Instantiating a
  class template declares its constrained members without checking their
  constraints. Members with unsatisfied constraints stay present but cannot
  be selected.
- **G7 Constrained constructors suppress the implicit default.** Declared
  constructors with unsatisfied constraints still count as user
  constructors, so no implicit default constructor appears.
- **G8 Dependent arguments postpone the check.** Inside a template, naming a
  constrained class, alias, or variable template with dependent arguments
  does not check its constraints until instantiation.
- **G9 Destructor selection.** A class may declare several prospective
  destructors with different constraints. Overload resolution among them
  with no arguments picks the destructor at the end of the class. Failure
  is an error.
- **G10 Explicit instantiation must satisfy.** Explicitly instantiating a
  constrained template with arguments that fail its constraints is an
  error.
- **G11 Explicit instantiation of a class skips unsatisfied members.**
  Explicitly instantiating a class template specialization also instantiates
  only those members whose constraints hold.
- **G12 Explicit specialization must satisfy.** Explicitly specializing a
  constrained template for arguments that fail its constraints is an error.
- **G13 Lambda conversion to function pointer.** A captureless non-generic
  lambda converts to a function pointer only when its constraints hold.
- **G14 Local class members.** Constraints on members of a local class are
  not instantiated with the enclosing function.
- **G15 Naming an unsatisfied function anywhere is an error.** A function
  whose trailing clause fails cannot be called, cannot have its address
  taken, and cannot be named in a decltype operand.
- **G16 Never satisfiable member template.** A member template whose
  constraints cannot hold for any argument, or cannot be validly
  substituted for any argument, makes the program ill-formed with no
  diagnostic required.
- **G17 Non-dependent use of a constrained template template parameter.**
  Inside a template, using a constrained template template parameter with
  non-dependent arguments that fail is ill-formed with no diagnostic
  required. The same goes for naming a constrained alias with failing
  non-dependent arguments inside a template.
- **G18 Partial specialization matching.** A partial specialization whose
  constraints fail for the arguments does not match.
- **G19 Special member eligibility.** A special member is eligible only when
  it is not deleted, its constraints hold, and no other special member of
  the same kind is more constrained. Eligibility feeds triviality and the
  trivially copyable property.

### H. Subsumption and ordering

- **H1 At least as constrained.** A constrained declaration is at least as
  constrained as an unconstrained one. Between two constrained declarations,
  the one whose constraints subsume the other's is at least as constrained.
- **H2 Atomic subsumption is identity only.** One atomic constraint subsumes
  another only when they are identical (E2). No arithmetic or logical
  reasoning applies, so a size test of more than four does not subsume a
  size test of more than two.
- **H3 Basic cases.** A conjunction subsumes each of its operands. An atomic
  constraint subsumes a disjunction that contains it. Each constraint
  subsumes itself. Operand order does not matter.
- **H4 Distinct concepts with equal bodies.** Two concepts with the same
  defining text do not subsume each other, because their atomic constraints
  come from different appearances.
- **H5 Duplicated inline text is ambiguous.** Two overloads whose
  constraints repeat the same non-concept text (a negation, a comparison)
  do not order, and the call is ambiguous. Moving the text into a named
  concept makes them order.
- **H6 Functionally equivalent mappings.** When ordering depends on two
  atomic constraints whose mappings are functionally equivalent but not
  written equivalently, the program is ill-formed with no diagnostic
  required.
- **H7 More constrained.** One declaration is more constrained than another
  when it is at least as constrained and the reverse fails.
- **H8 Non-template functions with the same parameters.** Between two
  viable non-template functions that tie on conversions and have the same
  parameter list, the more constrained wins.
- **H9 Partial specialization ordering.** Partial specializations with
  equivalent argument lists order by their constraints.
- **H10 Reordered template parameters for rewritten candidates.** When one
  candidate is a reversed rewritten candidate, the template parameters are
  matched by a unique reordering before constraints compare. Several or no
  reorderings give no order.
- **H11 Subsumption through normal forms.** One constraint subsumes another
  when each disjunctive clause of the first, taken against each conjunctive
  clause of the second, shares an identical atomic constraint.
- **H12 Template template argument matching.** A template template
  parameter accepts an argument only when the parameter is at least as
  specialized as the argument, and the comparison includes both heads'
  clauses.
- **H13 Templates order by constraints last.** Two function templates order
  by constraints only after partial ordering by types ties, and only when
  their heads and function parameter lists correspond. Invented parameters
  from constrained placeholders count as part of the head.

### I. Declarations and redeclarations

- **I1 Constrained friend is distinct per scope.** A friend with a clause, or
  a friend template whose constraint depends on an enclosing template's
  parameter, must be a definition. It never declares the same function as a
  declaration in another scope.
- **I2 Constraint is not part of the function type.** A trailing clause does
  not change a function's type.
- **I3 Equivalent spelling of concept arguments.** Two concept names with
  arguments compare equal when they name the same specialization, even when
  the argument tokens differ (forty-two and forty plus two).
- **I4 Explicit specialization picks a member by instantiated constraints.**
  When an explicit specialization of a member template of a class
  specialization must find its target, each candidate's in-class constraint
  is instantiated. An invalid result means no match.
- **I5 Implicit equality copies the clause.** The equality operator declared
  from a defaulted three-way comparison takes the same trailing clause.
- **I6 Language linkage ignored.** A C linkage block does not give C linkage
  to a friend with a trailing clause.
- **I7 Out of class member definition repeats the clause.** The definition
  of a constrained member outside its class template must repeat an
  equivalent trailing clause. Leaving the clause out finds no matching
  member.
- **I8 Overloading by clause alone.** Two members of a class template may
  differ only in their trailing clauses, and they are different functions.
  Two clauses with different text that are always true still differ.
- **I9 Override ignores the clause.** Override matching ignores trailing
  clauses.
- **I10 Redeclaration needs equivalent constraints.** Redeclarations of a
  function template must agree on the concepts before parameters, the
  leading clause, and the trailing clause. A concept before a parameter and
  the same concept in a clause make different heads.
- **I11 Separate definitions for lookup.** Each concept before a parameter
  and each clause of a function template is its own definition for name
  lookup and instantiation, apart from the function body.

## 3. Gaps and doubts

- **Access in requires expressions.** Whether a requirement that names a
  private member gives false or an error. The access context is unclear.
- **Atomic type check strictness.** Compilers may differ on how strictly a
  non-boolean atomic result is diagnosed, and on whether it counts as an
  error or as not satisfied (F2).
- **Caching of satisfaction.** Compilers cache results. A probe that
  completes a type between two checks (F7) may show different answers per
  compiler.
- **Constrained destructors and eligibility.** Support for several
  prospective destructors and for eligibility of constrained special members
  arrived late in some compilers. The effect on triviality may differ
  (G9, G19).
- **Constrained friends.** Whether two constrained friend definitions from
  two instantiations clash, and when compilers check that (I1).
- **Explicit instantiation of a class with failing members.** Whether
  compilers skip such members silently (G11).
- **Fold over a pack in a constraint.** This draft treats a fold as one
  atomic constraint. Newer compilers may split it into its operands for
  subsumption. Probes should test both views (E8).
- **Functionally equivalent constraints.** The no-diagnostic cases (E3, H6,
  D1, G16, G17) may be diagnosed by some compilers and accepted by others.
- **Invalid requires expression outside a template.** Some compilers may
  give false instead of an error (D4).
- **Non-template members of nested classes.** Whether a trailing clause is
  accepted on a member of a non-template class nested inside a class
  template, which is templated (B8).
- **Out of class definitions.** Matching an out of class definition with a
  trailing clause against several in-class overloads that differ only in
  their clauses has had bugs (I7, I8).
- **Recursive satisfaction.** A constraint whose check needs the same
  constraint again (through overload resolution on the same function) gets
  a compiler-specific error. The draft text gives no rule.
- **Reversed candidate reordering.** Few compilers implement the unique
  reordering rule (H10).
- **Satisfaction of a placeholder inside a declared type.** Whether a
  compiler checks the deduced placeholder type or the whole declared type
  (C10).
- **Template template parameters.** Checking that a parameter is at least as
  specialized as an argument, with clauses in play, is new and uneven
  (H12).

Tidal already records compiler differences for OVL-19, OVL-20, OVL-30,
OVL-41, and ORD-13 (questions Q-0142, Q-0160, Q-0162, Q-0146, Q-0210).

## 4. Already covered by Tidal

- **AUTO-13** lets a concept precede a placeholder (C6, part of C9).
- **AUTO-16** requires the deduced type of a constrained placeholder to
  satisfy the concept (C9). The rule does not say that the check sees only
  the deduced part (C10).
- **DED-34** makes a placeholder function parameter invent a type parameter
  (part of C1). Constrained placeholders and packs are open.
- **ORD-13** orders function templates by constraints after the type steps,
  with the head and parameter conditions (H13, in part H7).
- **ORD-14** gives partial specializations their constraints in ordering
  (H9).
- **OVL-19** removes unsatisfied functions and less constrained plain
  functions when an address is taken with a target (G1).
- **OVL-20** applies the same removals without a target type (G1).
- **OVL-24** makes a candidate with unsatisfied constraints not viable (G4).
- **OVL-30** lets the more constrained of two plain functions win (H8).
- **OVL-41** lets the more constrained of two template specializations win
  (H13).
- **TPL-20** compares template heads but leaves constraints out (I10 is
  open).

Every other item in section 2 is open. The pages that mark constraints as
missing are the template index, declarations, deduction, dependent names,
instantiation, and ordering pages, the overload resolution page, the
placeholder page, and the primary expressions page.
