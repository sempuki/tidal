# Glossary

This glossary defines the terms that Tidal pages use. Each page links a
term to its entry here. A marker such as *(library referenced)* names a
topic that Tidal does not specify yet.

:::{glossary}
alias template
  A template whose declaration is a `using` alias. A specialization of an
  alias template is the type that the alias gives, and no new type
  ({rule}`TPL-28`).

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

associated class
  A class that an argument type adds to argument-dependent lookup, for
  example the argument's own class and its bases. The friends of an
  associated class are found ({rule}`NAM-27`, {rule}`NAM-29`).

associated namespace
  A namespace that an argument type adds to argument-dependent lookup,
  for example the namespace that declares the argument's class
  ({rule}`NAM-27`).

automatic storage
  Storage that exists while the block of an object runs, until the end
  of the block. A jump past the declaration does not remove the object
  ({rule}`JMP-16`). See {term}`storage duration`.

best viable function
  The viable function that is better than each other viable function of
  a use. Overload resolution calls it ({rule}`OVL-34`).

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

copy-initialization
  The initialization of an object from an initializer after `=`
  ({doc}`/declarations/initialization/forms`). Passing an argument and
  returning a value are also copy-initialization. An `explicit`
  constructor is not used in copy-initialization ({rule}`CTOR-9`).

cv-qualifier
  `const` or `volatile` applied to a type. A program cannot change a
  `const` object after its initialization. Each access to a `volatile`
  object is an observable effect ({rule}`TR-7`).

declarator
  The part of a declaration that gives one name and makes its type. A
  declarator adds pointers, references, arrays, and function parameters
  to the base type.

default initialization
  The initialization of an object that nothing initializes. A scalar
  keeps no value, and a class object runs its default constructor
  ({rule}`INIT-9`). A union with no default member initializer gets no
  active member ({rule}`UNI-4`).

default template argument
  The argument that a template parameter takes when a template-id gives
  none for it, written after `=` in the template head ({rule}`TPL-13`,
  {rule}`TARG-9`).

discarded statement
  The branch of an `if constexpr` statement that the condition does not
  select. These facts apply to a discarded statement:

  - It does not execute.
  - Its `return` statements do not take part in return type deduction.
  - A function or variable that only a discarded statement uses needs no
    definition.
  - In a template, a discarded statement is not instantiated.

ellipsis conversion sequence
  The sequence of an argument that matches the `...` of a function. It is
  worse than every other sequence ({rule}`ICS-6`, {rule}`ICS-21`).

explicit specialization
  A declaration that starts with `template <>` and defines one
  specialization of a template for one argument list ({rule}`TSP-1`,
  {rule}`TSP-2`).

full expression
  An expression that is not part of a larger expression, such as the
  expression of an expression statement or a condition. The temporaries
  of a full expression end with the full expression ({rule}`VC-9`).

function template
  A template whose declaration declares a function. Each specialization
  is a function ({rule}`TPL-1`).

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

  {rule}`CE-23` gives the full rule.

member template
  A template declared in a class body. Such a template declares a
  function, a nested class, or a static variable with a head of its own
  ({rule}`TPL-24`).

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

partial specialization
  A declaration with its own template head that defines the
  specializations of a class template or variable template whose
  arguments match a pattern ({rule}`TSP-9`, {rule}`TSP-14`).

placeholder type
  `auto` or `decltype(auto)` in the position of a type. The actual type
  comes from an initializer or from `return` statements. A concept name
  can come before the placeholder to constrain the type.

primary template
  The declaration of a class template or variable template that is not
  an explicit or partial specialization. A specialization that no other
  declaration covers comes from it ({rule}`TSP-13`).

promoted arithmetic type
  A floating-point type, or an integer type that
  {term}`integer promotion` leaves unchanged, such as `int` and `long`
  ({principle}`OVL-P4`).

rewritten candidate
  A candidate for a comparison that uses another comparison operator,
  such as `!(x == y)` for `x != y`, or the same operator with the
  operands swapped ({rule}`DCMP-6`, {rule}`OVL-13`).

sequence rank
  The rank of a {term}`standard conversion sequence`: exact match,
  promotion, or conversion. The worst conversion in the sequence gives
  the rank ({rule}`ICS-2`).

specialization
  The class, function, variable, or type that a template and one list
  of template arguments give. Two template-ids with the same template
  and the same arguments name one specialization ({rule}`TARG-19`).

standard conversion sequence
  An {term}`implicit conversion sequence` of at most three standard
  conversions, in a fixed order ({rule}`ICS-1`).

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

temporary object
  An object that the program makes from a prvalue, without a name. A
  temporary object ends with its full expression unless a rule makes
  its life longer ({rule}`VC-5`, {rule}`VC-8`, {rule}`VC-6`).

trivial
  A special member function that is not user-provided and does only
  the work of the class's parts, so that nothing virtual and no default
  member initializer is involved ({rule}`CTOR-23`, {rule}`DTOR-9`,
  {rule}`CPY-27`).

translation unit
  One source file after preprocessing, with all the files that it
  includes. Each translation unit is compiled separately. Then the
  translation units are linked into one program.

type parameter
  A template parameter that takes a type, declared with `class` or
  `typename` ({rule}`TPL-5`).

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

variable template
  A template whose declaration declares a variable. Each specialization
  is a variable ({rule}`TPL-26`).

viable function
  A candidate function whose parameters fit the arguments and whose
  constraints hold, with a conversion sequence for each argument
  ({rule}`OVL-22`, {rule}`OVL-23`).
:::
