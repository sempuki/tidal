# Glossary

This glossary defines the terms that Tidal pages use. Each page links a
term to its entry here. A marker such as *(library referenced)* names a
topic that Tidal does not specify yet.

:::{glossary}
abstract class
  A class with a pure virtual function that it declares, or that it
  inherits without a final overrider. A program cannot create an object
  of an abstract class ({rule}`VIRT-8`).

aggregate candidate
  A deduction guide that an aggregate class template gets for one
  initializer list. Its parameters are the types of the members that the
  elements initialize ({rule}`CTAD-9`).

alias template
  A template whose declaration is a `using` alias. A specialization of an
  alias template is the type that the alias gives, and no new type
  ({rule}`TPL-28`).

alignment
  A number of bytes, always a power of two. The address of every object
  of a type is a multiple of the type's alignment ({rule}`ALN-1`,
  {rule}`ALN-4`). `alignof` gives the alignment of a type, and
  `alignas` can make the alignment of a declaration stricter.

ambiguous conversion sequence
  The sequence of an argument that two or more user-defined conversions
  can convert equally well. The sequence counts as user-defined, and a
  call that chooses its function is not valid ({rule}`ICS-7`,
  {rule}`ICS-8`).

argument-dependent lookup
  Lookup of the name in an unqualified function call in the namespaces
  and classes that the argument types add. {rule}`NAM-31` says when the
  lookup happens, and {rule}`NAM-29` says what the lookup finds. See
  {term}`associated namespace`.

arithmetic type
  An {term}`integral type` or a floating-point type. The floating-point
  types are `float`, `double`, `long double`, and the extended
  floating-point types *(extended floating-point types referenced)*.
  {doc}`/foundations/types` lists the fundamental types. An enumeration
  is a separate kind of type, and an unscoped enumeration promotes to an
  integral type ({rule}`CONV-1`).

associated class
  A class that an argument type adds to argument-dependent lookup, for
  example the argument's own class and its bases. The friends of an
  associated class are found ({rule}`NAM-27`, {rule}`NAM-29`).

associated constraints
  The constraints of a declaration, joined into one conjunction in a
  fixed order ({rule}`CNS-1`).

associated namespace
  A namespace that an argument type adds to argument-dependent lookup,
  for example the namespace that declares the argument's class
  ({rule}`NAM-27`).

atomic constraint
  A part of a constraint that normalization does not split: an
  expression, with a mapping of template parameters to arguments. Two
  atomic constraints are identical only if they come from the same
  expression in the source ({rule}`CNS-3`, {rule}`CNS-4`).

automatic storage
  Storage that exists while the block of an object runs, until the end
  of the block. A jump past the declaration does not remove the object
  ({rule}`JMP-16`). See {term}`storage duration`.

best viable function
  The viable function that is better than each other viable function of
  a use. Overload resolution calls it ({rule}`OVL-34`).

bind directly
  Said of a reference that refers to the object or function that its
  initializer names, with no {term}`temporary object` in between
  ({rule}`REF-1`, {rule}`REF-2`, {rule}`REF-6`).

built-in candidate
  A candidate function that stands for a built-in operator, with fixed
  parameter types. A class operand reaches it through a conversion
  function ({rule}`OVL-9`).

candidate function
  A function that overload resolution considers for a use. The context
  of the use decides the candidates ({rule}`OVL-1`, {principle}`OVL-P2`).

class template
  A template whose declaration declares a class. Each specialization is a
  class ({rule}`TPL-1`).

class template argument deduction
  The deduction of the template arguments of a class template from an
  initializer, when the program writes the template name with no
  argument list ({rule}`CTAD-1`).

common initial sequence
  The longest run of members, from the first, in which two
  standard-layout structs match in type, width, alignment, and
  `[[no_unique_address]]`. A union of the two structs can read that
  run through either struct ({rule}`UNI-15`, {rule}`UNI-10`).

complete
  Said of a type whose size and layout the implementation knows. A class
  is complete after its definition ({rule}`TYN-6`). An enumeration with
  a fixed underlying type is complete from its first declaration on,
  also inside its enumerator list. An enumeration without a fixed
  underlying type is complete after its closing brace ({rule}`ENUM-10`).
  An opaque enumeration declaration ({rule}`ENUM-7`) makes the
  enumeration complete and declares no enumerator, so a use that needs
  the enumerators needs the definition ({rule}`ENUM-20`,
  {rule}`NAM-16`).

concept
  A named constraint on template arguments, defined with `concept` at
  namespace scope ({rule}`CPT-1`).

concept-id
  The name of a concept with a template argument list. A concept-id is
  a constant of type `bool` ({rule}`CPT-1`).

constant evaluation
  Evaluation of an expression during translation, before the program
  runs. Constant evaluation occurs where translation needs a value, for
  example in these places:

  - an array bound,
  - a `static_assert`,
  - the condition of `if constexpr`,
  - a call to a `consteval` function.

  An implementation rejects a program if constant evaluation reaches an
  operation that has undefined behavior.
  {doc}`/foundations/constant-evaluation` gives the rules.

constant initializer
  An initializer whose whole initialization of the variable is a
  constant expression. Such a variable's value is known during
  translation (CE-24).

constant parameter
  A template parameter that takes a value, such as `N` in
  `template <int N>`. Its type is a {term}`structural type`
  ({rule}`TPL-6`), and its argument is a constant expression
  ({rule}`TARG-12`).

constraint
  A condition on template arguments that the implementation checks
  during translation. See {term}`concept`, {term}`requires clause`, and
  {term}`type constraint`.

contextual conversion
  The conversion of a value to `bool` where a true or false value is
  necessary, for example in an `if` condition. The value `e` is
  converted as `bool t(e);` converts `e`, so `explicit` conversion
  functions are used. {rule}`CONV-3` gives the values that convert to
  `false`.

converting constructor
  A constructor that is not `explicit`. An implicit conversion can use a
  converting constructor to make an object of its class from the
  constructor's arguments ({rule}`CTOR-10`).

copy candidate
  The deduction guide of every class template that takes a
  specialization of the template and gives the same specialization, so a
  copy keeps its type ({rule}`CTAD-7`).

copy-initialization
  The initialization of an object from an initializer after `=`
  ({doc}`/declarations/initialization/forms`). Passing an argument and
  returning a value are also copy-initialization. An `explicit`
  constructor is not used in copy-initialization ({rule}`CTOR-9`).

current instantiation
  The specialization that a class template, or one of its members or
  nested classes, is making. Names of the current instantiation are
  looked up at the definition ({rule}`DEP-15`, {rule}`DEP-16`).

cv-qualifier
  `const` or `volatile` applied to a type. A program cannot change a
  `const` object after its initialization. Each access to a `volatile`
  object is an observable effect ({rule}`TR-7`).

declarator
  The part of a declaration that gives one name and makes its type. A
  declarator adds pointers, references, arrays, and function parameters
  to the base type.

deduction
  The step that finds the template arguments of a function template from
  a use, such as the argument types of a call ({rule}`DED-1`,
  {principle}`DED-P2`).

deduction failure
  The end of a {term}`deduction` that gives no specialization. Any step
  of the deduction can fail ({principle}`DED-P2`). Examples are values
  that do not agree, a template parameter with no value, a
  {term}`substitution failure`, and constraints that are not satisfied
  ({rule}`CNS-11`). The template adds no candidate, and the program
  stays valid. A use of the name is not valid only when no function
  remains ({rule}`DED-35`).

deduction guide
  A function template that takes the arguments of an initializer and
  gives a specialization of a class template. The implementation makes
  guides from the constructors, and a program can declare more
  ({rule}`CTAD-5`, {rule}`CTAD-11`).

default constructor
  A constructor that a call with no arguments can call. It has no
  parameters, or each parameter has a default argument. A class that
  declares no constructor gets one implicitly ({rule}`CTOR-2`).

default initialization
  The initialization of an object that nothing initializes. A scalar
  keeps no value, and a class object runs its default constructor
  ({rule}`INIT-9`). A union with no default member initializer gets no
  active member ({rule}`UNI-4`).

default new alignment
  The alignment that a global allocation function without an alignment
  parameter gives. A `new` expression for a type with a larger
  alignment passes the alignment to the allocation function
  ({rule}`NEW-27`, {rule}`NEW-28`).

default template argument
  The argument that a template parameter takes when a template-id gives
  none for it, written after `=` in the template head ({rule}`TPL-13`,
  {rule}`TARG-9`).

dependent name
  A name whose meaning can differ between specializations of a
  template. Lookup of the name waits for the template arguments
  ({rule}`DEP-7`, {rule}`DEP-8`).

dependent type
  A type in a template that can differ between specializations, such as
  a template parameter or `Box<T>` ({rule}`DEP-1`).

discarded statement
  The branch of an `if constexpr` statement that the condition does not
  select. These facts apply to a discarded statement:

  - It does not execute.
  - Its `return` statements do not take part in return type deduction.
  - A function or variable that only a discarded statement uses needs no
    definition.
  - In a template, a discarded statement is not instantiated.

dynamic type
  The type of the {term}`most derived class` object that a glvalue refers
  to. A virtual call uses the dynamic type ({rule}`VIRT-1`), and the
  static type of the glvalue can be a base of the dynamic type.

eligible
  Said of a special member function that is not deleted, whose
  constraints are satisfied, and that no special member of the same
  kind beats by being more constrained ({rule}`CNS-21`).

ellipsis conversion sequence
  The sequence of an argument that matches the `...` of a function. It is
  worse than every other sequence ({rule}`ICS-6`, {rule}`ICS-21`).

exception object
  The object that a `throw` makes from its operand ({rule}`THR-1`). The
  implementation provides its storage, and the object ends when the
  last {term}`handler` for it ends ({rule}`THR-8`).

explicit instantiation
  A declaration that starts with `template` or `extern template` and no
  `<`. It names a specialization to instantiate. See
  {term}`explicit instantiation definition` and
  {term}`explicit instantiation declaration`.

explicit instantiation declaration
  An explicit instantiation with `extern`. Another translation unit has
  the definition of the named specialization ({rule}`INST-27`,
  {rule}`INST-28`).

explicit instantiation definition
  An explicit instantiation without `extern`. The translation unit
  instantiates and defines the named specialization ({rule}`INST-16`).

explicit specialization
  A declaration that starts with `template <>` and defines one
  specialization of a template for one argument list ({rule}`TSP-1`,
  {rule}`TSP-2`).

final overrider
  The override of a virtual function that a virtual call selects for
  the class of the object. Each virtual function of a class must have
  exactly one ({rule}`VIRT-5`, {rule}`VIRT-1`).

forwarding reference
  A function parameter of type `T&&`, where `T` is a type parameter of
  the function template itself, with no qualifiers. An lvalue argument
  makes `T` an lvalue reference type ({rule}`DED-8`).

full expression
  An expression that is not part of a larger expression, such as the
  expression of an expression statement or a condition. The temporaries
  of a full expression end with the full expression ({rule}`VC-9`).

function parameter pack
  A function parameter declared with `...`. It stands for zero or more
  parameters, one for each element of the pack in its type
  ({rule}`PACK-1`).

function template
  A template whose declaration declares a function. Each specialization
  is a function ({rule}`TPL-1`).

fundamental alignment
  An alignment that the implementation supports for objects in every
  kind of storage. The alignment of each fundamental type is a
  fundamental alignment. An alignment above the largest fundamental
  alignment is an *extended alignment*, which `new` also supports
  ({rule}`ALN-4`). The largest fundamental alignment is 16 on the x86-64
  targets ({rule}`ALN-5`).

handled exception
  The {term}`exception object` of the {term}`handler` that started most
  recently and has not ended. A handler starts when its parameter is
  initialized and ends when control leaves its block by any route
  ({rule}`TRY-15`). When an inner handler ends, the exception of the
  outer handler is the handled exception again. A rethrow passes on the
  handled exception ({rule}`THR-6`, {rule}`TRY-8`).

handler
  A `catch` clause of a try block: a parameter declaration and a block.
  The first handler whose parameter type matches the exception object
  runs ({rule}`TRY-1`, {rule}`TRY-2`).

header
  A file that holds declarations for several translation units to
  share. Each unit that names the file in `#include "file"` gets the
  file's text ({rule}`INC-1`).

immediate function context
  A part of a program where a call to a `consteval` function need not be
  a constant expression by itself. Examples are the body of a
  `consteval` function and the branch of `if consteval` that executes
  during {term}`constant evaluation`.

implicit conversion sequence
  The description of how one argument becomes one parameter of a
  function. The sequence is standard, user-defined, or ellipsis, and
  overload resolution compares the sequences of two functions
  ({rule}`ICS-21`).

implicit object parameter
  The parameter of a non-static member function that receives the
  object of a call. Its type is a reference to the class
  ({rule}`ICS-13`).

implicit-lifetime type
  A type whose objects can exist without a constructor call: a scalar,
  an array, a simple class, or a cv-qualified form of one. Some
  operations create objects of these types implicitly
  ({rule}`STG-7`, {rule}`STG-8`).

implied object argument
  The object that a call passes to the object parameter of a member
  function, for example `x` in `x.f()` ({rule}`OVL-4`).

include guard
  The `#ifndef`, `#define`, and `#endif` lines around the text of a
  {term}`header`. The lines make every inclusion after the first in a
  translation unit add no text ({rule}`INC-4`).

injected class name
  The name that a class declares for itself inside its own scope. Inside
  `C`, the name `C` finds the class first ({rule}`NAM-33`). A derived
  class inherits the injected class name of each base ({rule}`NAM-35`).

instantiation
  The making of a specialization from its template, with the template
  arguments in place of the parameters. A use that needs a definition
  instantiates it ({principle}`INST-P1`).

integer promotion
  The conversion of an integer operand that is narrower than `int`.
  The types narrower than `int` are the following:

  - `bool`,
  - `char`, `signed char`, and `unsigned char`,
  - `short` and `unsigned short`.

  If `int` can hold all values of the type, the result is `int`. If not,
  the result is `unsigned int`. The types `wchar_t`, `char8_t`,
  `char16_t`, and `char32_t` promote to the narrowest standard integer
  type that can hold all their values. The candidates are tried by rank
  from `int` upward, the signed type of each rank before the unsigned
  type. Unscoped enumerations and bit-fields also promote (CONV-1).

integral type
  `bool`, a character type, or a signed or unsigned integer type. The
  character types are `char`, `signed char`, `unsigned char`, `wchar_t`,
  `char8_t`, `char16_t`, and `char32_t` ({doc}`/foundations/types`).

layout-compatible
  Said of two types with the same layout: the same type apart from
  cv-qualifiers, two enumerations with one underlying type, or two
  standard-layout classes whose members match in full ({rule}`UNI-16`).

lifetime
  The time during which an object exists in its storage. The lifetime
  of a class object starts when its initialization completes and ends
  when its destructor completes. The lifetime of a scalar object starts
  with its storage. The release or reuse of the storage also ends the
  lifetime ({rule}`OBJ-1`, {rule}`OBJ-10`).

linkage
  The property that controls if two declarations of a name refer to the
  same entity.

  - A name with *external* linkage refers to the same entity in all
    {term}`translation units <translation unit>`.
  - A name with *internal* linkage refers to the same entity in one
    translation unit only.
  - A name with no linkage refers only to the entity of its own
    declaration.

literal type
  A type whose objects constant evaluation can create and destroy.
  The following types are literal types:

  - scalar and reference types,
  - arrays of literal types,
  - closure types whose captures have literal types,
  - classes with an implicit, defaulted, or `constexpr` destructor whose
    bases and non-static data members have literal types that are not
    `volatile`. A union needs only one member that is not `volatile`.
    The class is an aggregate, or has a `constexpr` constructor other
    than a copy or move constructor.

  {rule}`CE-23` gives the full rule.

member template
  A template declared in a class body. Such a template declares a
  function, a nested class, or a static variable with a head of its own
  ({rule}`TPL-24`).

memory location
  The storage of one scalar object, or of a group of adjacent
  bit-fields. A bit-field of width zero ends such a group. Two
  {term}`unsequenced` accesses conflict only when they reach the same
  memory location ({rule}`SEQ-4`).

more constrained
  The relation that orders two declarations by their constraints
  ({rule}`CNS-23`).

more specialized
  One template is more specialized than another when deduction finds the
  other's parameters from its invented types, and not the reverse
  ({rule}`ORD-1`).

most derived class
  The class that an object was created as. Its constructors initialize
  the virtual bases ({rule}`DER-13`).

natural alignment
  The alignment of the type of an entity, before any `alignas` on its
  declaration. `alignas` must not make the alignment weaker than the
  natural alignment ({rule}`ALN-2`).

non-allocating placement form
  An `operator new` or `operator new[]` with a `void*` parameter after
  the size, which returns its `void*` argument and allocates nothing.
  An array `new` that uses it asks for no extra storage
  ({rule}`LSUP-7`, {rule}`NEW-33`).

non-deduced position
  A place in a parameter type where a template parameter takes no value
  from the argument, such as the part before `::` in a qualified name
  ({rule}`DED-13`).

normal form
  A constraint written as conjunctions and disjunctions of atomic
  constraints ({rule}`CNS-2`).

object representation
  The `sizeof` bytes of an object, viewed as `unsigned char`
  ({rule}`STG-11`).

observable behavior
  The part of an execution that an implementation must reproduce:

  - each read and write of a `volatile` object, in program order,
  - the data that the program writes out *(library referenced)*,
  - the exit status.

  An implementation can change anything else (TR-P5).

operator function
  A function whose name is `operator` followed by an operator, such as
  `operator+`. An expression that uses the operator can call the
  operator function ({rule}`OPR-1`).

ordinary encoding
  The encoding of character and string literals with no prefix. The
  encoding maps each character to one or more `char` code units
  ({rule}`PRIM-30`). The implementation chooses the encoding, and the
  matrix targets use UTF-8 ({rule}`PRIM-32`).

overload set
  The functions and function templates that one name finds where the
  program names them without calling them, as in `&f` or in an argument
  `f`. A target type selects one function from the set
  ({rule}`OVL-18`). Without a target type, {rule}`OVL-20` selects it. A
  set can hold a single function. A set of one plain function and one
  function template is an overload set, and OVL-20 selects from it as
  from a larger set.

pack
  A name for a list of zero or more elements. Template parameters,
  function parameters, and init captures can be packs
  ({principle}`PACK-P1`).

pack expansion
  A pattern followed by `...`. The expansion stands for a list with one
  copy of the pattern for each element of the packs in it
  ({rule}`PACK-5`).

parameter mapping
  The part of an atomic constraint that gives an argument to each
  template parameter that its expression uses ({rule}`CNS-4`).

partial ordering
  The comparison that decides which of two templates is more
  specialized ({principle}`ORD-P2`).

partial specialization
  A declaration with its own template head that defines the
  specializations of a class template or variable template whose
  arguments match a pattern ({rule}`TSP-9`, {rule}`TSP-14`).

placeholder type
  `auto` or `decltype(auto)` in the position of a type. The actual type
  comes from an initializer or from `return` statements. A concept name
  can come before the placeholder to constrain the type.

point of instantiation
  The place in a translation unit where a specialization is made. The
  specialization sees the declarations before the place
  ({rule}`INST-30`, {rule}`INST-31`).

pointer-interconvertible
  Said of two objects at one address, such as a union and its member,
  when a cast turns a pointer to one into a pointer to the other
  ({rule}`STG-16`).

polymorphic class
  A class that declares or inherits a virtual function
  ({rule}`VIRT-11`).

primary template
  The declaration of a class template or variable template that is not
  an explicit or partial specialization. A specialization that no other
  declaration covers comes from it ({rule}`TSP-13`).

promoted arithmetic type
  A floating-point type, or an integer type that
  {term}`integer promotion` leaves unchanged, such as `int` and `long`
  ({principle}`OVL-P4`).

prospective destructor
  One of several destructors that a class declares with different
  constraints. At the end of the class, the destructor is chosen from
  them ({rule}`CNS-19`).

requires clause
  The keyword `requires` and a constraint, after a template parameter
  list or after the declarator of a function ({rule}`CPT-8`,
  {rule}`CPT-11`).

requires expression
  An expression that tests whether types and expressions are valid for
  template arguments. Its value is a constant of type `bool`
  ({rule}`CPT-24`).

rewritten candidate
  A candidate for a comparison that uses another comparison operator,
  such as `!(x == y)` for `x != y`, or the same operator with the
  operands swapped ({rule}`DCMP-6`, {rule}`OVL-13`).

satisfied
  Said of a constraint that is true for the given template arguments
  ({rule}`CNS-5` to {rule}`CNS-7`).

scalar type
  An {term}`arithmetic type`, an enumeration, a pointer, a pointer to
  member, or `std::nullptr_t`, with any cv-qualifiers.
  {doc}`/foundations/types` lists the fundamental types. A scalar object
  has no parts, and its lifetime starts with its storage
  ({rule}`OBJ-1`).

sequence rank
  The rank of a {term}`standard conversion sequence`: exact match,
  promotion, or conversion. The worst conversion in the sequence gives
  the rank ({rule}`ICS-2`).

side effect
  A change to the state of the program that an evaluation makes, apart
  from computing its value. A write to an object is a side effect, and
  so is a read of a `volatile` object ({rule}`SEQ-4`).

special member function
  One of the six member functions that the implementation can declare
  for a class:

  - the {term}`default constructor` ({rule}`CTOR-2`),
  - the copy constructor ({rule}`CPY-1`),
  - the move constructor ({rule}`CPY-2`),
  - the copy assignment operator and the move assignment operator
    ({rule}`CPY-3`),
  - the destructor ({rule}`DTOR-1`).

specialization
  The class, function, variable, or type that a template and one list
  of template arguments give. Two template-ids with the same template
  and the same arguments name one specialization ({rule}`TARG-19`).

standard conversion sequence
  An {term}`implicit conversion sequence` of at most three standard
  conversions, in a fixed order ({rule}`ICS-1`).

standard-layout
  Said of a class whose object starts with its first member, with no
  virtual parts, no members split over classes or accesses, and no two
  subobjects of one type at its start ({rule}`CLS-19`). Scalar types,
  arrays of standard-layout types, and their cv-qualified forms are also
  standard-layout ({rule}`CLS-29`).

storage duration
  The time for which the storage of an object exists.

  - *Automatic* storage exists until the end of the block.
  - *Static* storage exists for all of the program run.
  - *Thread* storage exists for the life of a thread, with one object
    for each thread.
  - *Dynamic* storage comes from `new` and goes back with `delete`
    ({rule}`NEW-9`).

structural type
  A type that a constant parameter can have. Scalar types and lvalue
  references are structural. A literal class is structural when its bases
  and members are public, not `mutable`, and structural ({rule}`TPL-6`).

subobject
  An object inside another object: a base subobject, a non-static data
  member, or an array element. A virtual base gives one subobject that
  every path to it shares ({rule}`DER-6`).

substitution failure
  A type or an expression that is not valid, made when template
  arguments replace template parameters, in a place where the program
  stays valid. In the function type of a function template, the
  substitution failure is a {term}`deduction failure` ({rule}`DED-24`).
  In an atomic constraint, the constraint is not satisfied
  ({rule}`CNS-7`). An error in the body of a class or function that the
  substitution instantiates is not a substitution failure
  ({rule}`DED-29`, {rule}`CNS-33`).

subsume
  A constraint subsumes another if each way to satisfy the first shares
  an atomic constraint with each part of the second ({rule}`CNS-22`).

surrogate candidate
  A candidate for a call on a class object that a conversion function to
  a pointer or reference to function gives. The call converts the object
  and then calls the result ({rule}`OVL-6`).

template
  A declaration with a template head. Each argument list for its
  parameters names one {term}`specialization` ({principle}`TPL-P1`).

template argument
  A type, a constant expression, or a template, given in a template-id
  for one template parameter ({rule}`TARG-5`).

template head
  The part `template <...>` that starts a template declaration and
  lists its parameters ({rule}`TPL-1`).

template parameter pack
  A template parameter declared with `...`. It takes zero or more
  arguments of its kind ({rule}`TPL-30`, {rule}`TARG-21`).

template template parameter
  A template parameter that takes a class template or alias template,
  declared with a template head of its own ({rule}`TPL-12`,
  {rule}`TARG-17`).

template-id
  The name of a template followed by a template argument list in angle
  brackets, such as `Box<int>` ({rule}`TARG-1`).

templated context
  The definition of a {term}`templated entity`. The implementation checks
  the code there once for each template argument list. A rule that says
  "in a template" means in a templated context. Examples are the body of
  a function template, the body of a generic lambda also at namespace
  scope, and a member function of a local class in a function template
  ({rule}`IF-13`, {rule}`SA-5`, {rule}`LAM-15`).

templated entity
  An entity that has template parameters of its own or from an
  enclosing entity. These entities are templated ({rule}`CPT-11`,
  {rule}`INST-36`):

  - a template, and the call operator of a generic lambda
    ({rule}`LAM-15`),
  - an entity that a templated entity defines: a member, a friend
    defined in a class template, a local class, a local enumeration, and
    each lambda's closure type.

  A function that a templated entity only declares, such as a function
  declared at block scope, is not templated. A specialization of a
  template is not templated.

temporary object
  An object that the program makes from a prvalue, without a name. A
  temporary object ends with its full expression unless a rule makes
  its life longer ({rule}`VC-5`, {rule}`VC-8`, {rule}`VC-6`).

translation unit
  One source file after preprocessing, with all the files that it
  includes. Each translation unit is compiled separately. Then the
  translation units are linked into one program.

trivial
  A special member function that is not user-provided and does only
  the work of the class's parts, so that nothing virtual and no default
  member initializer is involved ({rule}`CTOR-23`, {rule}`DTOR-9`,
  {rule}`CPY-27`).

trivially copyable
  Said of a type whose objects a byte copy can copy. Scalar types,
  classes that meet CPY-19, arrays of these types, and their
  cv-qualified forms are trivially copyable ({rule}`CPY-28`). A class
  meets {rule}`CPY-19` if the class can be copied or moved by at least
  one eligible function, every such function is trivial, and the
  destructor is trivial and not deleted.

type concept
  A concept whose first template parameter is a type parameter. Only a
  type concept can be used in a type constraint ({rule}`CPT-18`).

type constraint
  A concept name, with optional template arguments, before a type
  parameter or a placeholder ({rule}`CPT-17`, {rule}`CPT-20`).

type parameter
  A template parameter that takes a type, declared with `class` or
  `typename` ({rule}`TPL-5`).

type-dependent
  Said of an expression in a template whose type can differ between
  specializations ({rule}`DEP-2`).

unordered
  The relation between two evaluations when the language does not set
  an order and the evaluations cannot overlap. One evaluation finishes
  before the other starts, and the implementation picks the order each
  time. Two arguments of a call ({rule}`SEQ-1`), and a call against the
  rest of its expression ({rule}`SEQ-8`, {rule}`ASN-11`), are unordered.

unsequenced
  The relation between two evaluations when the language does not set an
  order and the evaluations can overlap. The operands of a built-in
  operator are unsequenced, unless a rule fixes their order
  ({rule}`SEQ-8`). A function call in one operand does not
  overlap with work in the other operand. The call and the other work
  can still occur in either order.

unwinding
  The destruction of objects that takes place when an exception leaves
  a {term}`full expression` or a block on its way to a handler. The
  temporaries of the full expression end first, then the automatic
  objects of each block, innermost block first ({rule}`UNW-1`).

usable in constant expressions
  Said of a variable or a temporary object whose value a constant
  evaluation can read, also if the evaluation did not create it. The
  kinds are a `constexpr` variable, a `const` integer or enumeration
  variable with a {term}`constant initializer`, a reference variable
  with a constant initializer, and a `const` temporary that a
  `constexpr` reference extends ({rule}`CE-3`).

use
  A point where the running program needs a variable or a function
  itself: its storage, its address, or its code. A used entity needs a
  definition. Reading the value of a constant and naming an entity in an
  unevaluated operand are not uses ({rule}`ODR-3`).

user-defined conversion sequence
  An {term}`implicit conversion sequence` that calls one constructor or
  conversion function between two standard sequences ({rule}`ICS-3`).

usual arithmetic conversions
  The conversions that give the two operands of a binary arithmetic
  operator one common type. The steps are in {rule}`CONV-19`.

value category
  The kind of an expression. Each expression has one of these kinds:

  - An *lvalue* refers to an object or function that has an identity. A
    program can take the address of an lvalue.
  - An *xvalue* refers to an object whose resources a program can reuse,
    for example the result of `std::move`.
  - A *prvalue* computes a value, for example `a * b` or `42`. A prvalue
    has no identity.

value representation
  The bits of an object's bytes that decide its value. The other bits
  are padding ({rule}`STG-11`).

value-dependent
  Said of an expression in a template whose value can differ between
  specializations, while its type does not ({rule}`DEP-4`).

variable template
  A template whose declaration declares a variable. Each specialization
  is a variable ({rule}`TPL-26`).

viable function
  A candidate function whose parameters fit the arguments and whose
  constraints hold, with a conversion sequence for each argument
  ({rule}`OVL-22`, {rule}`OVL-23`).

virtual base
  A base named with `virtual` in a base list. An object holds one
  subobject of a virtual base, however many paths lead to it
  ({rule}`DER-6`).
:::
