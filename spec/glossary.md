# Glossary

This glossary defines the terms that Tidal pages use. Each page links a
term to its entry here. A marker such as *(library referenced)* names a
topic that Tidal does not specify yet.

:::{glossary}
automatic storage
  Storage that exists from the declaration of an object to the end of its
  block. See {term}`storage duration`.

constant evaluation
  Evaluation of an expression during translation, before the program
  runs. It occurs where translation needs a value. Examples are these:

  - an array bound,
  - a `static_assert`,
  - the condition of `if constexpr`,
  - a call to a `consteval` function.

  An implementation rejects a program if constant evaluation reaches an
  operation that has undefined behavior.
  {doc}`/foundations/constant-evaluation` gives the rules.

constant initializer
  An initializer whose whole initialization of the variable is a constant
  expression. Such a variable's value is known during translation
  (CE-24).

contextual conversion
  The conversion of a value to `bool` where a true or false value is
  necessary, for example in an `if` condition. The value `e` is converted
  as `bool t(e);` converts it, so `explicit` conversion functions are
  used. {rule}`CONV-3` gives the values that convert to `false`.

cv-qualifier
  `const` or `volatile` applied to a type. A program cannot change a
  `const` object after its initialization. Each access to a `volatile`
  object is an observable effect ({rule}`TR-7`).

declarator
  The part of a declaration that gives one name and makes its type. A
  declarator adds pointers, references, arrays, and function parameters
  to the base type.

discarded statement
  The branch of an `if constexpr` statement that the condition does not
  select. These facts apply to a discarded statement:

  - It does not execute.
  - Its `return` statements do not take part in return type deduction.
  - A function or variable that only a discarded statement uses needs no
    definition.
  - In a template, a discarded statement is not instantiated.

immediate function context
  A part of a program where a call to a `consteval` function need not be a
  constant expression by itself. Examples are the body of a `consteval`
  function and the branch of `if consteval` that executes during
  {term}`constant evaluation`.

integer promotion
  The conversion of an integer operand that is narrower than `int`.
  These types are:

  - `bool`,
  - `char`, `signed char`, and `unsigned char`,
  - `short` and `unsigned short`.

  If `int` can hold all values of the type, the result is
  `int`. If not, the result is `unsigned int`. The types `wchar_t`,
  `char8_t`, `char16_t`, and `char32_t` promote to the narrowest standard
  integer type that can hold all their values. The candidates are tried by
  rank from `int` upward, the signed type of each rank before the unsigned
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
  These types are literal types:

  - scalar and reference types,
  - arrays of literal types,
  - closure types whose captures have literal types,
  - classes with an implicit, defaulted, or `constexpr` destructor whose
    bases and non-static data members have literal types that are not
    `volatile`. A union needs only one member that is not `volatile`.

  {rule}`CE-23` gives the full rule.

observable behavior
  The part of an execution that an implementation must reproduce:

  - each read and write of a `volatile` object, in program order,
  - the data that the program writes out *(library referenced)*,
  - the exit status.

  An implementation can change anything else (TR-P5).

placeholder type
  `auto` or `decltype(auto)` in the position of a type. The actual type
  comes from an initializer or from `return` statements. A concept name can
  come before the placeholder to constrain the type.

storage duration
  The time for which the storage of an object exists.

  - *Automatic* storage exists until the end of the block.
  - *Static* storage exists for all of the program run.
  - *Thread* storage exists for the life of a thread, with one object
    for each thread.
  - *Dynamic* storage comes from `new` and goes back with `delete`
    ({rule}`NEW-9`).

translation unit
  One source file after preprocessing, with all the files that it
  includes. Each translation unit is compiled separately. Then the
  translation units are linked into one program.

unsequenced
  The relation between two evaluations when the language does not set an
  order and the evaluations can overlap. The operands of most binary
  operators are unsequenced. A function call in one operand does not
  overlap with work in the other operand. The two can still occur in
  either order.

usual arithmetic conversions
  The conversions that give the two operands of a binary arithmetic
  operator a common type. The steps are:

  1. If one operand has a floating-point type, the other operand is
     converted to the floating-point type with the larger range. The order
     is `float`, `double`, `long double`. Extended floating-point types
     have their own ranks *(fundamental types referenced)*. If neither
     type's rank is greater than the other's, the operation is not valid.
  2. If not, both operands go through {term}`integer promotion`. If the
     types are then the same, that type is the common type.
  3. If both types are signed, or both are unsigned, the narrower operand
     is converted to the wider type.
  4. If the unsigned type is at least as wide as the signed type, the
     signed operand is converted to the unsigned type.
  5. If the signed type can hold all values of the unsigned type, the
     unsigned operand is converted to the signed type.
  6. If not, both operands are converted to the unsigned type that has the
     width of the signed type.

  Width means the conversion rank *(fundamental types referenced)*. For
  the standard integer types the order is `char`, `short`, `int`, `long`,
  `long long`.

value category
  The kind of an expression. There are three kinds:

  - An *lvalue* refers to an object or function that has an identity. A
    program can take its address.
  - An *xvalue* refers to an object whose resources a program can reuse,
    for example the result of `std::move`.
  - A *prvalue* computes a value, for example `a * b` or `42`. It has no
    identity.
:::
