<!--
Behaviour inventory, 2026-10-31: dependent names, two-phase checking,
and instantiation (roadmap item N2).
Written by: a source reader under AGENTS.md that read the C++ working
draft sources and Tidal's own pages. It did not write or change any file
in this repository; the integrator copied this summary here.
Checked before use: the source reader ran tools/similarity.py on its
summary outside the repository; the integrator's own check gives 0
flagged. No code, no quotes, no section names. The compiler notes in
section 3 are expectations to probe, not matrix results.
-->

Role: source reader

# Behaviour inventory for item N2

## 1. Categories (probe-grid axes)

- Where the name sits: in a class template body, in a member defined
  outside its class, in a function template body, in a nested class,
  in a partial specialization, in a generic lambda.
- How the name is written: unqualified, qualified through a parameter,
  qualified through the class itself, after a member access with a
  dot or arrow, as a call with dependent arguments.
- What the name resolves to: a type, a template, a value, a function,
  nothing.
- Kind of dependence: none, on the type only, on the value only, on
  both.
- Base classes of the enclosing class: none, independent, dependent,
  the enclosing class itself.
- Disambiguating keyword: absent, typename present, template present,
  both, one present where it has no use.
- Context of the name: a place where only a type can appear, or a place
  where an expression can also appear.
- Time of the error: the template is never used; the template is used
  once with arguments that work; the template is used with arguments
  that fail.
- Kind of construct with the error: independent of every parameter;
  dependent; impossible for every argument.
- Kind of specialization: class, member function, static data member,
  member class, member enumeration, function template, variable
  template, default argument, exception specifier, virtual member.
- What triggers instantiation: an object, a pointer only, a member
  access through a pointer, a conversion between pointers, deletion,
  a call, overload resolution only, a constant evaluation need.
- Order of declarations: helper declared before the template, between
  the template and its use, after the use, after the end of the file.
- Explicit instantiation form: definition, declaration (with the
  extern keyword), both in one file, declaration with no definition
  anywhere.

## 2. Behaviours

### A. Binding of names

- A1 Argument lookup at both points: for a call whose arguments depend
  on a parameter, argument-dependent lookup looks in two places: where the
  template is written and where the specialization gets made.
- A2 Assignment operator name dependent: the name of the assignment
  operator inside a templated class is treated as dependent, since each
  specialization declares its own set.
- A3 Default argument seen later: a default for a function parameter or for a
  template parameter that only becomes reachable after the definition,
  and that would change the meaning of a specialization, makes the
  program broken with no diagnostic required.
- A4 Dependent base hidden from plain names: an unqualified name in a
  class template is not searched for in a base class whose type depends
  on a parameter, so a member of such a base needs an explicit object
  path or qualification.
- A5 Dependent call names: an unqualified function name in a call is
  dependent when an argument is type-dependent, when an argument is a
  pack expansion, or when the name carries template arguments that
  depend on a parameter.
- A6 Dependent conversion names: the name of a conversion function to a
  dependent type is dependent, and it is resolved per specialization,
  which can find a conversion in a dependent base.
- A7 Dependent names resolved per specialization: a dependent name is
  resolved again for each set of arguments, after the arguments replace
  the parameters.
- A8 Fundamental argument adds nothing: a call with only arguments of
  fundamental type finds functions declared after the template only if
  ordinary lookup at definition saw them, because argument lookup adds
  no namespaces.
- A9 Independent names bound early: a name that does not depend on any
  parameter is bound to the declarations visible where the template is
  defined; later declarations do not change the binding.
- A10 Independent names not found: an independent name with no visible
  declaration at the template definition is an error even if a later
  declaration exists before the use.
- A11 Lookup through all files: if a dependent call would find a better
  or a different viable function by looking at every external function
  in the associated namespaces of every file, the program is broken and
  no diagnostic is required.
- A12 Members of the argument ignored: members of a type argument do not
  change what a name in the template means when the name already binds
  to a member of the template or to a namespace name.
- A13 Ordinary lookup only at definition: for a dependent call, plain
  unqualified lookup sees only declarations that precede the template
  definition; functions declared after the definition are found only by
  argument-dependent lookup.
- A14 Qualified member found twice: when a qualified name that names a
  member of the current instantiation finds one declaration at the
  definition and a different set at instantiation, the result is
  ambiguous and the program is rejected.
- A15 Unqualified member through implicit this: an unqualified member
  name that finds one member at definition is not checked again for a
  conflict with a dependent base at instantiation, while the same name
  written with an explicit this path is.

### B. Checking a template before instantiation

- B1 Early check allowed: an implementation may check a template body
  before any instantiation, and it may diagnose errors in independent
  constructs at that time.
- B2 Errors in dependent constructs deferred: an error that depends on
  the arguments is reported only when a specialization with those
  arguments is made.
- B3 Hypothetical instantiation at definition: if making a
  specialization right after the definition would fail because of an
  independent construct, the program is broken with no diagnostic
  required.
- B4 Impossible constraint: an atomic constraint that could never be
  checked without error, and that is never checked, makes the program
  broken with no diagnostic required.
- B5 Local class part of its owner: a local class or enumeration in a
  template, with its members, default arguments, and member
  initializers, is checked and instantiated together with the enclosing
  entity, never on its own.
- B6 Meaning drift: if an independent construct means one thing right
  after the definition and something else at a real instantiation, the
  program is broken with no diagnostic required. Causes include a type
  that is incomplete at definition and complete later, a constant
  defined later, a partial or explicit specialization declared later,
  and a default argument added later.
- B7 No valid specialization: a template, or a discarded branch of a
  compile-time if inside a template, for which no argument list can
  give a valid result, and which is never instantiated, makes the
  program broken with no diagnostic required. Failing static assertions
  are left out of this test.
- B8 Pack always empty: a variadic template whose every valid
  specialization needs an empty pack is broken with no diagnostic
  required.
- B9 Syntax errors: a token sequence that cannot parse is diagnosable
  in a template that is never used.
- B10 Valid template, no diagnostic: when some valid specialization
  exists and none of the cases above applies, the implementation must
  not reject the template body before instantiation.

### C. Current instantiation

- C1 Alias counts as same: an alias template specialization equivalent
  to the class template with its own parameters names the current
  instantiation.
- C2 Base equals enclosing class: a nested class whose base is the
  enclosing class template with its own parameters has a base that is
  the current instantiation, so lookup into that base works at
  definition.
- C3 Dependent member of the current instantiation: a member of the
  current instantiation that lookup finds in a declaration of that
  class (a member, or a using-declaration of a dependent name) is a
  dependent member of it.
- C4 Equivalent constant argument: a constant argument that is a
  variable of the same type initialized with just the parameter name,
  directly or through a chain of such variables, counts as the
  parameter; a parenthesized initializer, an added expression, or a
  different type breaks the equivalence.
- C5 Injected name: inside a class template, a nested class, or a
  member, the injected name of the class names the current
  instantiation.
- C6 Lookup into current instantiation at definition: a qualified name
  whose qualifier is the current instantiation is looked up at
  definition time and needs no typename keyword when it names a type.
- C7 Member not found with dependent base: a qualified name with the
  current instantiation as its qualifier that finds nothing, in a class
  with a dependent base, is dependent and is resolved at instantiation.
- C8 Member not found without dependent base: a qualified name with
  the current instantiation as its qualifier that finds nothing, in a
  class with no dependent base, is an error at definition.
- C9 Nested class by member name: inside a nested class, the nested
  class named as a member of the current instantiation is the current
  instantiation.
- C10 Own template-id: the class template name with its own parameters
  in order names the current instantiation, also when written with a
  global qualifier.
- C11 Partial specialization: in a partial specialization, the template
  name with the argument list of that partial specialization names the
  current instantiation; the primary form does not.
- C12 Reordered or changed arguments: the template name with any other
  argument list, such as a pointer to the parameter or the parameters
  swapped, is a different, dependent specialization.
- C13 Typedef of an independent type: a member alias of the current
  instantiation is dependent only when the type it stands for is
  dependent.

### D. Dependent expressions

- D1 Address of a dependent member: the address of a qualified name that
  finds a dependent member of the class being defined has a
  value that depends on the arguments;
  taking the address of a templated static object or member function
  that a constant evaluation reaches is also value-dependent.
- D2 Braced list: a braced list has a dependent type as soon as one of
  its items has, or one of its items expands a pack.
- D3 Captured entity with dependent self: an entity captured by copy in
  a lambda whose explicit object parameter has a dependent type is
  type-dependent.
- D4 Cast and construction types: a cast, a functional-style
  conversion, or a new expression is type-dependent only when the named
  type is dependent, whatever its operand.
- D5 Concept check value: a concept used with dependent arguments is
  value-dependent and not type-dependent.
- D6 Deduced-type variable: a variable whose declared type uses a
  placeholder and whose initializer is type-dependent is
  type-dependent; the same holds for a structured binding.
- D7 Fold and pack size: a fold expression is always type-dependent;
  both a fold and the pack-size operator are always value-dependent.
- D8 Function name string: the predefined function-name variable is
  type-dependent inside any enclosing function template, member of a
  class template, or generic lambda.
- D9 Member access: a member access is type-dependent when the member
  name is dependent, or when it names a member of the current
  instantiation whose type is dependent; when the object's class is
  dependent and differs from the class being defined, the access is
  always type-dependent.
- D10 Member function with deduced return: a name that finds member
  functions of the current instantiation with deduced return types is
  type-dependent.
- D11 Never type-dependent forms: literals, size queries, alignment
  queries, type identification, delete, throw, and the noexcept
  operator always have a known type.
- D12 Placeholder constant parameter: a constant template parameter
  declared with a placeholder type is type-dependent.
- D13 Static array of unknown bound: a static data member of the
  current instantiation whose type is an array with no bound is
  type-dependent.
- D14 Subexpression rule: apart from the listed exceptions, an
  expression is type-dependent when a part of it is, and value-dependent
  in a constant context when a part of it is.
- D15 The object pointer: the this pointer is type-dependent when the
  enclosing class is dependent.
- D16 Value of casts: a cast is value-dependent when its type is
  dependent or its operand is value-dependent.
- D17 Value of size queries: size, alignment, type identification, and
  the noexcept operator are value-dependent when their operand is
  type-dependent or their type is dependent.
- D18 Value-dependent names: a constant template parameter, a constant
  variable initialized with a value-dependent expression, a static data
  member of the current instantiation initialized outside the class,
  and a static member function of the current instantiation are
  value-dependent.

### E. Dependent types and arguments

- E1 Array bound: an array type is dependent when its element type is
  dependent or its bound is value-dependent.
- E2 Built from dependent parts: a pointer, reference, function, or
  member pointer type built from a dependent type is dependent, and so
  is a cv-qualified dependent type.
- E3 Constant argument pointing into dependent class: a constant
  template argument for a pointer or reference parameter is dependent when it designates a member of a dependent
  class or of the class being defined.
- E4 Exception specifier dependence: a function type whose exception
  specifier is value-dependent is dependent; so is a function type with
  a parameter pack.
- E5 Nested member of current instantiation: a member class or member
  enumeration declared directly in the current instantiation is a
  dependent type.
- E6 Specialization with dependent arguments: a template-id whose
  template is a template parameter, or whose arguments are dependent,
  type-dependent, value-dependent, or pack expansions, names a
  dependent type.
- E7 Template parameter: a type template parameter is a dependent type.
- E8 Template template argument: a template argument that names a
  template parameter, or a template whose name is dependent, is
  dependent.
- E9 Type of a dependent expression: the declared type of a
  type-dependent expression, taken with decltype, is dependent.

### F. Disambiguation keywords

- F1 Base class lists need no keyword: names in a base list and before
  a scope operator are read as types without typename.
- F2 Context where only a type fits: in a type-only position, a
  qualified dependent name is read as a type without typename. These
  positions include alias targets, trailing return types, the type of a
  new expression, the type in the four named casts, a default for a type
  parameter, the type of a constant template parameter, the return and
  parameter types of a member declared in a class, the return type of a
  namespace-scope declaration, the parameters of a function whose name
  is qualified, and the parameters of a lambda or a requires parameter
  list (default arguments excluded).
- F3 Dependent qualified name read as a value: outside type-only
  positions, a qualified dependent name without typename is read as an
  expression, even if a specialization later makes it a type, and the
  specialization then fails.
- F4 Expression context at block scope: in a block, a declaration whose
  parameter list holds a qualified dependent name without typename is
  read as a variable with an initializer, not a function declaration.
- F5 Keyword template after a dependent qualifier: a member template named
  after a scope operator whose qualifier is dependent, and followed by an
  argument list, needs the template keyword.
- F6 Keyword template after a member access: after a dot or arrow on a
  dependent object, a member template followed by an argument list needs
  the template keyword; otherwise the opening angle bracket is less-than.
- F7 Keyword template banned positions: the template keyword must not
  follow the qualifier of a name being declared, and must not come
  before a destructor tilde.
- F8 Keyword template on a non-template member: the template keyword
  on a member that is not a template fails at instantiation.
- F9 Keyword template where it is not needed: the template keyword is
  allowed before a name that lookup already finds as a template.
- F10 Keyword template with no argument list: the template keyword
  before a name with no following argument list is allowed only when
  the name is a class template or alias template, and this use is
  deprecated.
- F11 Lookup after typename: a name marked with typename is resolved by
  ordinary qualified lookup at instantiation; if it finds a non-type
  member that hides a type, the specialization fails.
- F12 Typename needed with independent qualifier in a dependent
  expression: a scope operator after the declared type of a dependent
  expression is dependent even if the object turns out to be an
  independent class, so a type name there still needs typename.
- F13 Typename on a template-id: typename can introduce a qualified
  template-id, optionally with the template keyword before the final
  name.
- F14 Typename on a using-declaration: a using-declaration of a
  dependent name marked with typename introduces a type name.
- F15 Typename where it is not needed: typename is allowed before a
  qualified name that lookup already finds as a type.

### G. Explicit instantiation

- G1 Attributes and specifiers banned: an explicit instantiation must
  not carry attributes, any storage class other than thread-local, or
  inline, constexpr, or consteval on a function or variable.
- G2 Class instantiation covers members: an explicit instantiation of a
  class specialization also instantiates, with the same kind, each
  direct non-template member that is not explicitly specialized and
  whose constraints the arguments meet.
- G3 Class instantiation definition covers only defined members: an
  explicit instantiation definition of a class instantiates only those
  members whose definitions are visible at that point.
- G4 Constraints must hold: an explicit instantiation of a constrained
  template must meet its constraints.
- G5 Declaration before definition: if both a declaration with extern
  and a definition appear in one file, the declaration must come first.
- G6 Deduced trailing arguments: trailing template arguments may be left
  out when deduction from the declared function type finds them; an
  empty list can then be left out entirely.
- G7 Default arguments not used: an explicit instantiation does not
  instantiate default arguments, so a default that would fail for the
  arguments is not an error.
- G8 Definition must exist: the definition of a function, variable, or
  member being explicitly instantiated as a definition must be present
  in that translation unit; the definition of a class must be reachable.
- G9 Destructor selection: an explicit instantiation of a destructor
  must name the destructor the class selected.
- G10 Explicit specialization wins: an explicit instantiation after an
  explicit specialization for the same arguments has no effect.
- G11 Extern suppresses some instantiation: an instantiation declaration
  stops implicit instantiation of a non-inline function, a static data
  member, or a variable with a stated type; it does not stop
  instantiation of inline functions, functions or variables with deduced
  types, constant-usable variables, or class specializations.
- G12 Extern without a definition elsewhere: an entity named in an
  instantiation declaration and used in a way that would instantiate it
  must have an instantiation definition somewhere in the program; if not,
  the program is broken with no diagnostic required. This holds for
  inline functions too.
- G13 Implicit special members: an explicit instantiation must not name
  an implicitly declared special member function.
- G14 Internal linkage: a template whose linkage is internal cannot have
  its specializations named in an extern instantiation.
- G15 No new name: an explicit instantiation introduces no name;
  the template declaration must be visible before it.
- G16 Single declarator, no initializer: an explicit instantiation of a
  function or variable has one declarator and no initializer; for a
  class it uses a class key and a template-id; for a variable template
  the declarator is a template-id.
- G17 Type must match: the declared type in an explicit instantiation
  must match the type of the specialization, except that an array bound
  can be left out and a placeholder declaration can be named with its
  deduced type; a function with a deduced return type cannot be named
  with a written return type.
- G18 Variable declaration is not a definition: the declaration inside an
  explicit instantiation of a variable does not conflict with the
  definition it produces.

### H. Implicit instantiation

- H1 Class declared not defined: if the chosen template has no
  definition yet where the specialization is made, the class stays
  incomplete, and a use that needs completeness fails.
- H2 Class instantiation makes member declarations only: instantiating a
  class makes the declarations of its member functions, nested classes,
  scoped enumerations, static data, templates declared as members,
  and friend declarations, but not their definitions.
- H3 Class instantiation makes some definitions: deleted functions,
  unscoped enumerations, and anonymous union members are defined with
  the class.
- H4 Class needed complete: a class specialization is instantiated when
  a complete type is needed, or when completeness changes meaning, such
  as a pointer conversion to a base, a call whose overloads differ by
  class pointer, or deletion through a pointer.
- H5 Constant evaluation need: a function or variable that a constant
  evaluation could use is instantiated even when the evaluation is not
  required or does not reach it; an unevaluated conditional operand
  that is not potentially constant evaluated does not trigger it.
- H6 Constraints not substituted with the class: constraints on members
  are checked only when needed, so a class with constructors whose
  constraints all fail is valid until a constructor is needed.
- H7 Default argument on use: a default function argument is
  instantiated only when a call uses it, each one separately, with
  lookup as if it were in the function at that point.
- H8 Default arguments not with the class: instantiating a class does
  not instantiate the default arguments or exception specifiers of its
  member functions.
- H9 Default member initializer on use: a template used inside a
  default member initializer or a default argument is instantiated only
  when a constructor or aggregate initialization uses it, or when
  checking the default itself needs it.
- H10 Defined-in-class friend twice: a friend function defined in a class
  template is defined once per class specialization, so two
  specializations that define the same function give a redefinition.
- H11 Exception specifier on need: the exception specifier of a function
  template specialization is instantiated only when it is needed.
- H12 Function on need: a function template specialization or member
  function is instantiated only when its definition is needed, such as
  a call that is evaluated, or when its existence changes meaning.
- H13 Infinite recursion: recursive instantiation has an
  implementation-defined depth limit, and an endless chain is undefined.
- H14 Overload resolution needs only declarations: considering a
  function template in overload resolution instantiates its declaration,
  not its body.
- H15 Pointer does not instantiate: naming a class specialization only
  as a pointer or reference type does not instantiate it; a member
  access through the pointer does.
- H16 Possible but not required: if overload resolution can choose
  without a class definition, it is unspecified whether that class is
  instantiated.
- H17 Redeclaration conflicts: an instantiated declaration that comes
  from a definition counts as a definition for redeclaration checks,
  so two partial specializations of a member template that become the
  same after substitution conflict.
- H18 Static member initialization on need: a static data member is
  instantiated, and its initialization and side effects happen, only
  when its definition is needed.
- H19 Unused branch not instantiated: a discarded branch of a
  compile-time if is not instantiated; inside a generic lambda the
  branch waits until the call operator is instantiated.
- H20 Unused members not instantiated: an implementation must not
  instantiate a non-virtual member function, member class, static data
  member, member template, function template, or variable template that
  no use needs.
- H21 Variable template on need: a variable template specialization is
  instantiated when its definition is needed.
- H22 Virtual member possibly instantiated: it is unspecified whether an
  unused virtual member function of a class specialization is
  instantiated.

### I. Points of instantiation

- I1 Class before the use: the point of instantiation of a class
  specialization used from ordinary code is just before the
  namespace-scope declaration that uses it.
- I2 Class inside another specialization: a class specialization first
  needed from a dependent context in another specialization has its
  point just before the enclosing one's point.
- I3 Default argument point: the point of a default argument is the
  point of its function.
- I4 End of file is a point: a function or static data member
  specialization also has a point of instantiation at the end of the
  translation unit (or the end of the private module fragment).
- I5 Exception specifier point: follows the same pattern as functions,
  either at the point of the specialization that needs it or right
  after the declaration that needs it.
- I6 Explicit instantiation definition is a point: an explicit
  instantiation definition is a point of instantiation for its
  specializations.
- I7 Function after the use: the point of a function, member function,
  or static data member specialization used from ordinary code is just
  after the namespace-scope declaration that uses it.
- I8 Function inside another specialization: such a specialization
  needed from a dependent context in another specialization has the
  point of the enclosing specialization.
- I9 Modules: names with internal linkage in an imported unit cannot
  be used by an implicit instantiation in an importing unit; argument
  lookup at instantiation also sees declarations reachable from the
  instantiation context across module units.
- I10 Multiple points must agree: if two points of instantiation, in
  one file or across files, give a specialization different meanings,
  the program is broken with no diagnostic required.
- I11 One point per class: a class specialization has at most one point
  per translation unit.
- I12 Virtual function point: a virtual function that is instantiated
  has its point just after the point of its class.

## 3. Gaps and doubts

- Early checking of independent errors (B1, B3, B9). Clang and EDG
  diagnose many independent errors in unused templates, such as
  assigning an int to a pointer or calling an undeclared function with
  no dependent argument. GCC is more lenient for some cases and stricter
  for others. MSVC without its conformance mode delays almost all
  checking and accepts missing typename and template keywords. Probe
  each case with the template never instantiated.
- Missing typename (F2, F3, F4). The type-only positions are recent;
  older GCC and Clang versions still demand typename in alias targets,
  return types, and parameter lists. Check the matrix versions for each
  listed position separately, especially parameters of a member
  declared in a class and parameters of a qualified function name.
- Unqualified name in dependent base (A4). MSVC in permissive mode finds
  such names. GCC accepts with a permissive flag. Clang gives a
  recovery note. Expect agreement under strict modes only.
- Functions declared after the template (A13, A8). Older GCC versions
  found such functions by ordinary lookup at instantiation; current
  GCC rejects with a note. Probes with fundamental-type arguments
  separate the compilers best.
- The A14 and A15 pair (qualified versus unqualified member conflict at
  instantiation). Compilers differ; Clang and GCC may report the
  unqualified case as ambiguous too, or accept the qualified case.
- Equivalence of constant arguments for the current instantiation (C4).
  The parenthesized and different-type cases are rarely tested;
  compilers may disagree on whether a typename is then needed.
- Member not found in the current instantiation with no dependent base
  (C8). GCC and Clang both diagnose, but the time of diagnosis can
  differ; MSVC may defer.
- Never-instantiated template with no valid specialization (B7). A
  static assertion with a false independent condition is now exempt;
  older compilers reject it in an unused template, and Tidal SA-5
  already follows the newer behaviour. Check how each compiler treats a
  discarded branch that can never be valid.
- Meaning drift (B6). Implementations do not diagnose most cases; a
  probe can only show which binding each compiler uses, such as whether
  a constant defined after the template is seen.
- Unused virtual functions (H22). GCC and Clang instantiate virtual
  members when the vtable is emitted (usually with the key function or
  with the constructor); a probe with an invalid virtual body can give
  different outcomes across compilers and options.
- Class instantiation when not required (H16). Compilers differ on
  whether a class specialization is instantiated during overload
  resolution when it is not needed; a probe with a static assertion in
  the class body would show this.
- Constant evaluation triggers (H5). Clang and GCC differ on which
  unevaluated operands count as potentially constant evaluated, notably
  inside decltype and in braced initializers.
- Default member initializers and default arguments (H7, H9). The time
  of instantiation and of lookup in a default argument differs in edge
  cases, especially for lambdas in default arguments.
- Explicit instantiation of a class with members that fail (G2, G3).
  Members not defined yet, members with failing constraints, and
  members that fail for the arguments behave differently: GCC and Clang
  instantiate all defined members and reject a failing body; check
  whether constrained members are skipped on every compiler.
- Extern instantiation with inline functions (G12, G11). Optimizing
  builds inline the body; non-optimizing builds may need a definition
  from another file and fail to link. This is a link-time effect with no
  diagnostic, so probes need a two-file setup.
- Keyword template where not needed or with no argument list (F9, F10).
  Deprecation warnings differ; older compilers reject the form with no
  argument list.
- Assignment operator name dependent (A2). Rarely probed; expect quiet
  agreement but no evidence.
- Recursion depth (H13). Each compiler has a different default limit
  and a flag to change it; Tidal should state only that a limit exists.
- Defined-in-class friend twice (H10). Compilers diagnose at the second
  class instantiation; some only when the friend is used.
- Points of instantiation at end of file (I4, I10). Most compilers
  instantiate function bodies at the end of the file, so a function
  declared between the use and the end of file is visible to argument
  lookup. A probe can show this, but the program is then broken with no
  diagnostic required if the two points differ in meaning.
- Module rules (I9). Support for modules varies widely; probe only on
  compilers that support named modules.

## 4. Already covered by Tidal

- Angle bracket reading after a name, and the closing bracket rule:
  TARG-1, TARG-2, TARG-3, TARG-23, TARG-P1. Tidal does not yet cover the
  template keyword (all of group F that concerns it).
- Injected class name in a class template and in specializations:
  TARG-10, TARG-11, TARG-27, NAM-33 to NAM-36. These cover part of C5;
  the rest of group C is not covered.
- Lookup order of class members and template parameters for an
  out-of-class member, including independent bases: TPL-41, TPL-19,
  TPL-18. These touch A4 only from the side of independent bases.
- Discarded branches of compile-time if in templates: IF-13, IF-22,
  IF-23, IF-14, IF-15. These cover H19 and part of B7.
- Static assertions checked only on instantiation, and exempt in
  discarded branches: SA-5. This covers the static assertion exemption
  in B7.
- Exception specifier evaluated on need: EXS-9, EXS-10, EXS-6. These
  cover H11 in part; the point of instantiation of the specifier (I5)
  is not covered.
- Substitution during deduction, and which errors are hard errors
  (class bodies instantiated during substitution): DED-25, DED-28,
  DED-29. These cover part of H14.
- Explicit instantiation naming a specialization by deduction: DED-32.
  This covers part of G6.
- Explicit and partial specializations declared before first use:
  TSP-6, TSP-17. These relate to B6 and I10.
- Explicit specialization declared but not defined is incomplete:
  TSP-20. This is a sibling of H1.
- Per-specialization static members: TPL-47.
- Argument-dependent lookup in general: NAM-27, NAM-29 to NAM-32. These
  do not yet say which contexts a dependent call searches (A1, A13).
- Substitution of later parameters in a template-id: TARG-24.
- Markers that name this area as not yet specified: the templates index,
  the head of the declarations, deduction, and specialization pages,
  the evidence notes on the exception specification page (EXS-9 and
  EXS-10), and TPL-35 and DED-29 indirectly.
