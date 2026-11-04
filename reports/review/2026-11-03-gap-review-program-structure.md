# Gap review: program structure

Date: 2026-11-03
Role: source reader

Reference: the C++23 working draft in the local reference tree. The
review compares the live rules of the pages on translation, use and
the one-definition rule, fundamental types, and keywords with that
draft only. It also reads the fourteen provisional questions whose
Affects line names a rule on these pages and that have no gap-review
reading. Later defect resolutions and later standards are mentioned
only where they matter.

## Result

Two provisional answers conflict with the reference. Q-0129 and Q-0358
both go against it, and in both cases the reference supports option 2.
Q-0176 and Q-0350 rest on points the reference does not settle. The
other ten answers agree with the reference.

Most rules agree. The gaps are these:

- ODR-5 ties the use of the deallocation function to a constructor that
  can throw. The reference has no such condition. The reference also
  makes the definition of any constructor of a class use the class's own
  allocation and deallocation functions.
- ODR-3 lets a whole class constant be copied without a use, and lets a
  member of a class constant be read when only that member is not
  mutable. The reference counts both as uses.
- ODR-3 treats a name in a discarded statement as no use. The reference
  removes only the need for a definition there. The use still counts
  for the capture rules of ODR-2.
- ODR-3 misses one case where the reference finds no use: a variable
  named in an expression whose value is discarded and never read.
- ODR-6 says a virtual function that is not pure is used when an object
  of its class is created. The reference makes it used in every case.
- ODR-8 does not forbid two definitions in two units of a function or
  variable that the program never uses. The reference forbids them.
- ODR-10 has no exception for names of constants with internal linkage.
  The reference allows those names to refer to different objects in
  each unit under four conditions.
- TR-1 and TR-P6 (with TR-13, TR-14, and TR-15) ask for rejection. The
  reference asks only for a diagnostic for a broken rule, and asks for
  nothing at all past a resource limit.
- TR-5 says constant and run-time results are the same and that
  constant evaluation happens only where a constant is needed. The
  reference leaves floating-point results free to differ, and it tries
  constant evaluation on more initializers than those that need a
  constant.
- TR-3 leaves out several forms of the main function that the reference
  rejects.
- TYP-1 puts the size of the boolean type below the size of the
  character types. TYP-4 derives each width from the size. The
  reference requires neither.
- TYP-7 does not reject a sign specifier with double.

## Rules

### Translation

| Rule | Verdict | Note |
|---|---|---|
| TR-P1 | agrees, one doubt | The classes match the reference's kinds of rule. See "Rejection and diagnostics" below. |
| TR-P3 | agrees, loosely | The principle says a program cannot give the main function a form other than the two. The reference requires those two forms and lets each implementation accept others. TR-3 states the rule correctly through FN-6. |
| TR-P4, TR-P5 | agree in part | See "Constant evaluation" below. |
| TR-P6 | extra condition | See "Rejection and diagnostics" below. |
| TR-1 | agrees, one doubt | See "Rejection and diagnostics" below. |
| TR-3 | missing cases | See "The main function" below. |
| TR-10 | agrees, with exceptions | Undefined operations of the core language make an evaluation not constant in the reference. Undefined operations of the library, and a failed assumption attribute, are excepted. For those the reference leaves it unspecified whether the evaluation is constant. An evaluation that would pass an implementation limit is also not constant. |
| TR-12 | agrees | The grammar rules are rules that need a diagnostic. |
| TR-2 | agrees | Leaving the end of the main function returns zero. A return from the main function leaves the function and then ends the program with that value. The argument count can be zero in the reference, and the array of strings ends with a null pointer. |
| TR-4 | agrees, one doubt | The reference requires exactly one main function at global scope as a rule that needs a diagnostic. TR-4 lists a missing main function as a link failure under a required rule. A link error is a diagnostic, so the outcome agrees. |
| TR-5 | wider than the reference | See "Constant evaluation" below. |
| TR-7 | agrees, two notes | See "Observable behavior" below. |
| TR-8 | agrees | The reference sets no requirement on an execution that reaches an undefined operation, including the steps before it. A later standard adds points in the execution past which undefined behavior cannot reach back. C++23 has no such points. |
| TR-9 | agrees | The environment supplies the arguments. The reference allows an argument count of zero, and allows the first string to be empty. The matrix facts are a valid instance. |
| TR-11 | agrees | The reference makes the status that the environment receives implementation-defined, except for the success and failure values. |
| TR-13 | agrees, one doubt | The reference names an implementation-defined limit on the total depth of nested instantiations, which can span several templates. Endless instantiation is undefined. See Q-0351. |
| TR-14, TR-15 | extra condition | The reference has no rule that rejects a large array or class type. It recommends a minimum object size as a guideline only. See "Rejection and diagnostics" below. |
| TR-6 | no counterpart | The reference does not split diagnostics into warnings and errors, and asks for no option that turns one into the other. The rule is Tidal's own. |

### Use and the one-definition rule

| Rule | Verdict | Note |
|---|---|---|
| ODR-P1 | agrees in part | The table's row for a discarded branch is too wide. See "Uses" below. |
| ODR-P2 | agrees in part | The first row needs "at most one in the program" for an entity that is not used. See ODR-8. |
| ODR-P3 | agrees | A const variable that is also volatile keeps external linkage in the reference, so the last sentence of the first paragraph holds only for a const variable that is not volatile. |
| ODR-15 | agrees, one missing case | The reference also makes a declaration outside the class of a static data member that was already defined inside the class with constexpr a declaration only. That form is deprecated. |
| ODR-1 | agrees, narrower | The reference forbids a second definition in one unit of any templated entity, default argument, and default template argument as well. Other pages may cover these. |
| ODR-2 | agrees, missing cases | The reference also forbids a use of an enclosing automatic variable in a default argument of a local function declaration and in a default argument of a lambda's parameter. A discarded statement does not exempt a use. See "Uses" below. |
| INC-3 | agrees | The reference requires that an include directive name a file the implementation can process. |
| ODR-3 | missing and extra conditions | See "Uses" below and Q-0129. |
| ODR-4 | agrees, one missing case | A constructor's definition also makes the destructors of its bases and members potentially invoked, so the constructor uses them as well. |
| ODR-5 | extra condition | See Q-0358. The reference also makes the definition of every destructor use the class's deallocation function, virtual or not. |
| ODR-6 | extra condition on virtual functions | The reference makes a virtual function that is not pure used in every case, and requires it to be defined or pure, with no diagnostic. No object needs to exist. The default-argument part agrees with the reference's treatment of templates. For a function that is not a template, the reference does not state the point directly. |
| ODR-7 | agrees | The reference lets classes, enumerations, inline functions, and inline variables have a definition in each unit, all treated as one entity. |
| ODR-12 | agrees | Templates and their members follow the same rule. A static local variable in a function template specialization is one object. |
| ODR-13 | agrees | An explicit specialization is not a templated entity in the reference, so one that is not inline follows the rule for ordinary functions and variables. |
| INC-1 | agrees, one missing note | The reference lets an implementation set a limit on nested inclusions and recommends a minimum. |
| INC-4 | agrees | The effect follows from the reference's rules for macro definitions and conditional inclusion. |
| ODR-8 | missing condition | See "Definitions across units" below. |
| ODR-9 | agrees | A definition in another unit names a different entity, and the reference requires a definition of every used function and variable with no diagnostic. |
| ODR-10 | missing exceptions | See "Definitions across units" below. |
| ODR-14 | agrees | The reference requires both declarations before the use, with no diagnostic. Inside one unit, the reference applies the same requirement. |
| INC-2 | agrees | The reference makes the search implementation-defined. If the search for a quoted name fails, the reference repeats the search as for the angle-bracket form. |
| INC-5 | agrees | The reference makes the effect of a pragma implementation-defined, and ignores a pragma that the implementation does not know. The once pragma is not in the reference. |
| ODR-11 | no counterpart | The reference asks for no warning here. The rule is Tidal's own and fits the reference. |

### Fundamental types

| Rule | Verdict | Note |
|---|---|---|
| TYP-P2 | agrees, loosely | The principle says that a signed and an unsigned type of the same size share a rank. The reference gives the shared rank only to a signed type and its own unsigned type. On ILP32 the signed int type and the unsigned long type have the same size and different ranks. TYP-8 states the rule correctly. |
| TYP-5 | agrees | |
| TYP-6 | agrees | |
| TYP-7 | missing case | The reference lets signed and unsigned combine only with char, short, long, and int. TYP-7 does not list a sign with double, or with long double. The fifth bullet does not cover it, because a sign is not a base type in TYP-P1. |
| TYP-8 | agrees | The reference has the same order and the same equal ranks. It also covers extended integer types, which the matrix does not have. |
| TYP-9 | agrees | The reference also ranks extended floating-point types, which an implementation can provide. The page does not mention them. |
| TYP-4 | extra condition | See "Widths" below. |
| TYP-11 | agrees | A null pointer and a null member pointer also convert to false. |
| TYP-13 | agrees, narrower | The reference fixes the underlying type of char8_t as unsigned char, and gives char16_t and char32_t unsigned underlying types on every implementation. TYP-13 states the second fact for the matrix only. |
| TYP-1 | extra condition | See "Widths" below. The reference also requires that the type of the null pointer literal has the size of a pointer to void. |
| TYP-2 | agrees | Alignment is implementation-defined in the reference. Each unsigned type has the alignment of its signed type. |
| TYP-3 | agrees, one difference | In the reference, plain char has an underlying type, either signed char or unsigned char, chosen by the implementation. The table says "none". The underlying types of char16_t and char32_t are the least-width unsigned types of 16 and 32 bits, which are themselves the implementation's choice. |
| TYP-10 | agrees | The representation and the accuracy of floating-point operations are implementation-defined. |
| TYP-14 | agrees | Rounding of a converted value and of a floating literal is the implementation's choice in the reference, and so is the accuracy of operations. |
| TYP-12 | classification | The reference gives no label to the padding between members, as item O found for CLS-31. The reference labels only alignment as implementation-defined. |

### Keywords

| Rule | Verdict | Note |
|---|---|---|
| KW-P1 | agrees | |
| KW-1 | agrees, one exception | The list matches the reference's keywords, with the alternative spellings added, which the reference reserves as well. The reference allows a keyword as an attribute token. |
| KW-2 | agrees, narrower | The reference makes each alternative spelling the same token as its operator in every respect except spelling. So the spellings also work where the token is not an operator, as in a reference declarator or a destructor name. The reference also has two-character spellings for brackets, braces, and the number sign, which the page does not mention. |
| KW-3 | agrees, one missing note | The four words match the reference. The words for import and module also start a module or import directive at the beginning of a line, in a form the reference describes. Modules are out of scope. |
| KW-4 | agrees, narrower | The reference reserves the first two kinds of name for any use, in any scope and as macro names, and needs no diagnostic. KW-4 forbids declaring them only. |

## Questions

| Question | Provisional answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0012 | Each limit is implementation-defined, with no minimum (option 1). | agrees | option 1 | The reference recommends a minimum object size, and says that the recommended minimums do not decide conformance. |
| Q-0024 | The two forms everywhere, others implementation-defined (option 1). | agrees | option 1 | The reference requires the form with no parameters and the form with a count and an array of strings. It leaves the rest of the type to the implementation, and recommends that extra parameters come after the array. |
| Q-0120 | EDG's alignment is valid (option 1). | agrees | option 1 | Alignment is implementation-defined. |
| Q-0127 | The guard is required and the once pragma implementation-defined (option 3). | agrees | option 3 | The guard's effect follows from required preprocessing rules. Pragmas are implementation-defined. |
| Q-0128 | No diagnostic, with a recommended warning (option 1). | agrees | option 1 | The reference needs no diagnostic. A call inside an inline function that nothing calls is still a use in the reference, so GCC's silence is allowed only because no diagnostic is required. |
| Q-0129 | A copy of a whole class constant is no use (option 1). | contradicts | option 2 | See below. |
| Q-0130 | Both cases are uses, EDG differs (option 1). | agrees | option 1 | The reference denies the exception to a variable with any mutable subobject, and a reference binding reads no value. Both uses happen where the variable cannot be used, so the programs are not valid. |
| Q-0176 | An initializer that runs at run time is an observable write (option 1). | silent, leans to option 2 | unclear | See below. |
| Q-0297 | One object for each specialization (option 1). | agrees | option 1 | The reference gives internal linkage only to a const variable that is not a template, and notes that a const specialization of a variable template can have external linkage. That wording came from a core defect resolution in the C++20 cycle, which GCC 14 lags. |
| Q-0298 | A difference that does not fit is undefined (option 1). | agrees | option 1 | The reference makes a difference outside the range of the result type undefined. A position inside the array, and one past its end, is a valid operand of addition, so EDG's rejection of a position past the largest difference does not follow from any rule. |
| Q-0315 | A constant template argument uses what it names (option 1). | agrees | option 1 | A template argument is not an unevaluated operand, so the reference counts it as potentially evaluated. An address taken there names the function and uses it. |
| Q-0350 | Clang has a defect in both cases (option 1). | silent | unclear | See below. |
| Q-0351 | Both implementation-defined, with no minimum (option 1). | agrees | option 1 | The reference names one limit on total depth, which can span several templates, and recommends a minimum of 1024 as a guideline only. EDG counts the depth of one template, and GCC's default is below the guideline. Neither choice breaks a requirement. |
| Q-0358 | A non-throwing constructor leaves the deallocation function unused (option 1). | contradicts | option 2 | See below. |

### Q-0129 in more detail

- The reference exempts a read of a constant variable only when the
  read converts a value of a type that is not a class type and not
  volatile. A copy of a whole class object makes no such conversion.
  The copy constructor takes the object by reference, and a reference
  binding needs the object itself.
- So the copy uses the variable. Inside a lambda with no captures, that
  use happens where the variable cannot be used, and the program is not
  valid. Clang's rejection matches the reference.
- The same exception also needs a variable with no mutable subobject at
  all. Reading a member that is not mutable is a use when some other
  member is mutable. ODR-3 lets that read through.
- These limits entered the reference through a core defect resolution
  in the C++20 cycle. GCC and EDG appear to keep the older rule. I know
  of no later resolution that removes the limits.

### Q-0358 in more detail

- In the reference, a new-expression names the matching deallocation
  function whenever the lookup finds a single match. The constructor's
  exception specification plays no part in that naming. The function
  is called only if initialization throws, but the use does not depend
  on the call.
- Separately, the reference makes the definition of any constructor of
  a class use the class's allocation and deallocation functions that are
  not placement forms. The test class defines a constructor, so that
  definition alone uses the deallocation function.
- So the program in the test uses a function it never defines. ODR-8
  needs no diagnostic, so a link failure and a running program are both
  allowed. GCC 16 at the default level conforms. The test's expectation
  of a run is one allowed result, and ODR-5's condition on a non-throwing
  constructor has no support in the reference.
- I know of no later resolution that changes this.

### Q-0176 in more detail

- The reference makes accesses through volatile glvalues observable. An
  access is a read or a change of a value. The reference lists the
  changes of a scalar object as assignment and increment or decrement.
  Initialization is not on that list, and an initializer does not act
  through a glvalue.
- For a class object, the reference states that volatile semantics do
  not apply while the object is under construction. They start when the
  constructor of the most derived object ends. That supports option 2
  for class types.
- For a scalar, the reference does not say. Option 1 is a reasonable
  reading of intent, but the text does not require it. The meaning of a
  volatile access is also implementation-defined in the reference.

### Q-0350 in more detail

- The reference has no rule that rejects a class whose size the size
  type cannot hold. It requires an implementation to accept and run
  correctly every valid program within its resource limits, and to
  document those limits where they are known.
- A wrong result from the size operator is not a correct execution. So
  Clang conforms only if the class is past one of its limits, and Clang
  documents no such limit. The reference does not say which reading
  wins. Option 1 relies on STG-1, which goes beyond the reference.

## Detail

### Rejection and diagnostics

- The reference requires at least one diagnostic for a program that
  breaks a rule that needs one. After the diagnostic, an implementation
  can still translate and run the program. The reference allows this
  explicitly for extensions.
- TR-P1 defines the reject class by the diagnostic, which agrees. TR-1
  says "rejects", which asks for more. The tests run with strict flags,
  so the difference changes no test.
- For resource limits, the reference requires only documentation, where
  the limits are known. It requires nothing for a program past a limit,
  not even a diagnostic. TR-P6, TR-13, TR-14, and TR-15 ask for
  rejection. The instantiation depth is the one limit the reference
  calls implementation-defined.

### The main function

The reference rejects these forms, and TR-3 does not list them:

- a main function that is inline, static, constexpr, or consteval,
- a main function defined as deleted,
- a main function declared with a language linkage specification,
- a variable named main at global scope,
- any entity in any namespace that is called main and has C linkage,
- a main function that is a coroutine (out of scope).

The reference also requires a declared return type of int, so a
deduced return type is not valid. TR-3 says "return int", which a
reader could take to allow a deduced type.

### Constant evaluation

- The reference leaves it unspecified whether a floating-point
  evaluation during translation gives the same result as at run time.
  It recommends consistent results only. TR-5 states the same result
  for every function, with no exception for floating point.
- The reference evaluates as constants more expressions than those
  where a constant is needed. The initializer of a static or thread-local
  variable is tried for constant initialization. If
  the trial fails, the variable gets dynamic initialization and nothing
  is rejected. An implementation can also perform such initialization
  statically when it is not required. TR-5 and TR-P4 say constant
  evaluation happens only where a constant is needed.
- TR-P5 says constant evaluation runs the abstract machine exactly. The
  exceptions under TR-10 above show where the reference does not
  require that.

### Observable behavior

- The reference counts accesses through volatile glvalues, the data
  written to files, and the prompting order of interactive devices. It
  does not list the exit status. TR-7 and TR-P5 add the exit status.
- The reference speaks of volatile glvalues, and TR-7 speaks of volatile
  objects. An access to a volatile object through a glvalue that is not
  volatile is undefined in the reference. An access to an ordinary
  object through a volatile glvalue is observable.

### Uses

- The reference exempts a variable named in an expression whose value
  is discarded, when the value is never read. ODR-3 has no such
  exception, so ODR-2 would reject a lambda with no captures that names
  a local variable only to discard it.
- The reference counts a use inside a discarded statement as a use. A
  definition is not needed for it, but the capture rules still apply.
  In a function that is not a template, a lambda with no captures that
  takes the address of a local variable inside a discarded branch is
  not valid in the reference. ODR-3 and the ODR-P1 table say there is no
  use. Inside a template, the discarded branch is not instantiated,
  which is a different case.
- The reference exempts reads through subscripting of an array operand
  and through member access, as well as through parentheses, the
  conditional operator on glvalues, and the comma operator. ODR-3 names
  member reads and the last three. It does not name an element read
  from a constant array.
- The reference does not count a call of a pure virtual function
  through an unqualified name as naming that function.

### Definitions across units

- ODR-8: the reference forbids definitions of one function or variable
  that is not inline and not templated in two units, with no
  diagnostic, whether or not the program uses the entity. ODR-8 states
  only the case of a used entity.
- ODR-10: the reference lets a name in matching definitions refer to a
  different object in each unit when the object is a const object of
  internal or no linkage that is not volatile, has the same literal type
  and value everywhere, is initialized by a constant expression, and is
  not used in any of the definitions. A reference of internal linkage
  that refers to one entity everywhere is also allowed. ODR-10 requires
  each name to refer to the same entity, so it makes the common case of
  an inline function reading a const value from a header not valid.
- The reference adds further conditions to matching definitions: the
  same language linkage, the same constant initialization of const
  objects, equal values of constant expressions, the same closure types
  for lambdas, and the same functions chosen by implicit constructors
  and defaulted comparisons. ODR-10 does not list them.

### Widths

- The reference makes the width of each integer type implementation-
  defined, above a minimum for each signed type. The value bits can be
  fewer than the bits of the object, and a byte can have more than
  eight bits. TYP-4 derives the width from the size in bytes and makes
  the result required. The matrix facts agree, but the rule states them
  for every implementation.
- TYP-4 says "every integer type", and the reference counts the boolean
  type as an integral type. The boolean type has two values (TYP-11),
  so the formula does not apply to it.
- The reference orders the sizes of the five signed integer types only.
  TYP-1 orders the boolean type and the character types by rank as
  well, which forces the boolean type to size 1. The reference makes the
  boolean type's size the implementation's choice.
