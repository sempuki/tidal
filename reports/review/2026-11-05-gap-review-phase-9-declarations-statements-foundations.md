# Gap review: the text phase 9 wrote on the declarations, statements, and foundations chapters

Date: 2026-11-05
Role: source reader

Reference: the C++23 working draft in the local reference tree (the
draft that became the C++23 standard). The review covers the rules and
principles on the declarations, statements, and foundations pages whose
prose changed between commit 1b16115 and HEAD. Lines that list tests,
rules, or exceptions are left out. Part A compares each changed rule
and principle with the draft. Part B applies nine principles, without
the rule text, to the programs of nine questions and checks each
answer against the draft. Part C reads fifteen questions against the
draft. Later defect resolutions are mentioned only where they may
change an answer.

## Result

Most of the changed text agrees with the reference. The fixes that
phase 9 made for the last review's gaps on fallthrough, nodiscard,
TYN-P1, LNK-1, NS-P2, IF-P4, and INIT-P2 close those gaps. Of the nine
principle uses, five give the provisional answer, and the use for
Q-0299 gives it in part. SB-P2 gives no answer
for Q-0376 and Q-0395, and DEFARG-P2 gives the opposite answer for
Q-0370. The reference supports the provisional answer in each of those
three questions. In Part C, the reference reads against four answers
(Q-0328, Q-0357, Q-0389, Q-0393).

The gaps are these:

- ENUM-P1 and ENUM-10 now give the first enumerator with no initializer
  the type int. The reference leaves that type to the implementation,
  among the signed integer types. ENUM-10 also gives an enumerator
  whose initializer has an unscoped enumeration type that enumeration
  type. The reference gives it the underlying type of that enumeration.
- LSUP-P1 and LSUP-2 say that a use of the three-way operator needs only
  the ordering class it yields. A defaulted three-way operator whose
  return type is the weak or the partial class also uses the strong
  class, because its result when all parts compare equal is the strong
  class's equal value converted to the return type.
- DEFARG-P1 still forbids a default argument to read the value of a
  local constant of an enclosing function. The reference allows a use
  of a local that is not an odr-use (Q-0389).
- DEFARG-P2 lets each later declaration add defaults with no limit. The
  reference makes a program ill-formed when a default added outside the
  class turns a constructor into a default, copy, or move constructor,
  or an assignment operator into a copy or move assignment operator
  (Q-0370).
- SB-P2 does not say which `get` the tuple protocol calls or what type
  the index has, so it cannot answer Q-0376 or Q-0395.
- SINIT-5 lets any first use of a function or variable of the unit end
  the deferral. The reference counts only uses that are not caused by
  the initialization of a static or thread variable, and only uses of
  entities that are not inline.
- TR-4 files a program with no main function under LNK-7, which needs
  no diagnostic. The reference gives that requirement no such
  exemption.
- NAM-21 leaves out the underlying type that follows the qualified name
  of an enumeration, and does not say that a friend declaration gets no
  such lookup.
- CONV-P3 says a conversion through a virtual base is not implicit. For
  a pointer to an object, the conversion to a pointer to a virtual base
  is implicit. Only pointers to members exclude virtual bases.

## Rules

| Rule | Verdict | Note |
|---|---|---|
| ALN-P1 | agrees with a doubt | The literal type and the constant conversion function match. The reference asks for exactly one target type among the conversion functions that are not explicit, and allows two such functions with the same target. See "ALN-P1, ALN-8". |
| ALN-8 | agrees with a doubt | Same doubt as ALN-P1. |
| ATTR-P1 | agrees | The fallthrough target must carry a label of the same switch, and nodiscard goes on a function, a class, or an enumeration. A lambda's call operator also counts as a function. |
| ATTR-2 | agrees | The reference also limits a fallthrough inside a loop to a target within the same pass of the loop body, which the loop case of ATTR-2 covers. |
| CX-P3 | agrees | The reference neither evaluates nor checks as a constant an immediate call in a default argument at the declaration. |
| ENUM-P1 | extra condition | The first enumerator with no initializer gets a signed type of the implementation's choice. The reference does not fix it as int. The enumeration-typed initializer case is still missing. See "ENUM-P1, ENUM-10". |
| ENUM-P3 | agrees with a doubt | The reference makes the enumeration ill-formed when no integral type fits. GCC's 128-bit type is not an extended integer type by GCC's own account, so GCC's silent acceptance under strict options is doubtful. See "ENUM-P3, ENUM-5". |
| ENUM-16 | agrees | The reference forbids omitting the name of a scoped enumeration. |
| ENUM-10 | contradicts | Two rows differ from the reference. See "ENUM-P1, ENUM-10". |
| ENUM-5 | agrees with a doubt | A wider type is allowed only if it counts as an integral type of the implementation. Otherwise a diagnostic is required. See "ENUM-P3, ENUM-5". |
| DEFARG-P1 | contradicts | The new paragraph on immediate calls agrees. The list of names a default argument cannot use still includes a local constant read for its value. See "DEFARG-P1". |
| DEFARG-10 | agrees | A friend defined in the class and a member of a nested class get the complete-class treatment, up to the outermost enclosing class. |
| DEFARG-3 | agrees | Same reading as DEFARG-10. |
| FN-P5 | agrees | The predefined variable is local to the body, has static storage, and can be named in the member initializers of a constructor. |
| INIT-P2 | agrees with a doubt | See "INIT-P2, INIT-1". |
| INIT-1 | agrees with a doubt | See "INIT-P2, INIT-1". |
| LIST-14 | agrees | The reference allows one designator for a member of each anonymous union. |
| SINIT-P2 | agrees with a doubt | The scalar case and the recursion case agree. The class member case is clear for a class with a non-trivial constructor and less clear for an aggregate with no constructor (Q-0175). |
| SINIT-7 | agrees | An inline static data member defined in the class has its definition inside the class, and its order is partial, as SINIT-3 says. |
| SINIT-3 | agrees with a doubt | Same doubt as SINIT-P2 on aggregates. |
| SINIT-4 | agrees with a doubt | The reference adds a second condition for early initialization, that the dynamic form would not change any other static object before that object's own initialization. |
| SINIT-5 | missing case | See "SINIT-5". |
| SINIT-6 | agrees with a doubt | The reference limits the choice to thread variables that are not inline. OBJ-11 says so, and SINIT-6 does not. |
| LSP-8 | agrees | The reference makes the clash ill-formed with no diagnostic required when the units differ. The matrix result is outside the rule, as the text says. |
| NS-P2 | agrees with a doubt | See "NS-P2, NS-12". |
| NS-12 | agrees with a doubt | See "NS-P2, NS-12". |
| STO-P1 | agrees | The reference destroys a statically initialized object in the order a dynamic initialization at its place would give. |
| LNK-1 | agrees | The list matches the reference, apart from module linkage, which Tidal does not cover. |
| STO-10 | agrees | Same reading as STO-P1. |
| SB-5 | agrees | The reference picks the member form only for a member function template that takes a value as its leading parameter. |
| TYN-P1 | agrees | The list now matches the reference, the lambda and the recursive limit on member classes included. |
| TYN-9 | agrees | Same as TYN-P1. |
| CE-9 | agrees | A read after the end of a lifetime is core undefined behavior, and the reference also bars reading an object whose lifetime did not start in the evaluation. |
| CONV-P2 | agrees | The reference treats integer to floating conversion as narrowing unless the source is a constant that round-trips. The three-way operator exempts exactly this conversion. |
| CONV-P3 | contradicts | The new sentence on overload resolution agrees. The old sentence that lists a virtual base among bases that block an implicit conversion is wrong for object pointers. See "CONV-P3, CONV-17". |
| CONV-8 | agrees | A null pointer constant of integer type converts to the null pointer type. |
| CONV-17 | agrees with a doubt | For an inaccessible base, the reference says plainly that access plays no part in forming the sequence. For an ambiguous base it says only that a program needing the conversion is ill-formed. See "CONV-P3, CONV-17". |
| KW-4 | agrees | The reference reserves suffixes with no leading underscore for future use and suffixes with a double underscore for implementations. Both need no diagnostic. |
| LSUP-P1 | missing case | See "LSUP-P1, LSUP-2". |
| LSUP-2 | missing case | See "LSUP-P1, LSUP-2". |
| NAM-21 | missing case | See "NAM-21". |
| OBJ-4 | agrees | Only a term link changed. |
| OBJ-11 | agrees | Only a term link changed. |
| TR-P1 | agrees | Breaks that show only at link time in the reference are of the kind that needs no diagnostic, such as a missing definition of a used function. |
| TR-4 | wrong classification | Correct for a used function with no definition. Wrong for a program with no main function. See "TR-4". |
| TR-13 | agrees with a doubt | The reference describes the limit as one on the total depth, which can span more than one template, and leaves an endless recursion undefined. EDG's count per template fits that description poorly. See "TR-13". |
| VC-P3 | agrees with a doubt | See "VC-P3". |
| IF-P4 | agrees | A reference with a constant initializer is usable in constant expressions, so its initializer is tried as a constant. A call inside a constant evaluation is part of it. |
| IF-25 | agrees | Same as IF-P4. |

## Principle uses

| Principle | Question | Test file | The model's answer | Matches the provisional answer? | Reference verdict |
|---|---|---|---|---|---|
| ODR-P1 | Q-0129 | `foundations/odr/class-constant-copy-in-lambda.cpp` | The return copies a whole class constant, so the copy constructor binds a reference to it. That binding is a use, and a lambda with no capture cannot use the local. Reject. | yes | supports with no doubt |
| REF-P3 | Q-0165 | `declarations/initialization/references/braced-function-overload-set.cpp` | A braced overload set for a function reference binds directly to the matching function. Accept. | yes | silent (unclear) |
| REF-P3 | Q-0165 | `declarations/initialization/references/braced-function-noexcept.cpp` | The element's type differs from the referenced function type by the exception specification, and a function type has no temporary. Reject. | yes | supports with no doubt |
| LSUP-P2 | Q-0299 | `foundations/language-support/alignment-enumeration-unscoped.cpp`, `alignment-enumeration-wrong-underlying-type.cpp`, `alignment-enumeration-class.cpp` | The program's declaration disagrees with the library on the kind of the name, so the program is not valid. The principle does not say whether a diagnostic is needed. | partly (it gives "not valid" and leaves the diagnostic open) | supports with a doubt |
| ODR-P1, ODR-P2 | Q-0358 | `foundations/odr/new-noexcept-constructor-uses-deallocation.cpp` | The new expression uses the class's deallocation function whatever the constructor's exception specification. The function has external linkage and no definition, and the fault shows only at link, if at all. Not valid, no diagnostic needed. | yes | supports with no doubt |
| DEFARG-P2 | Q-0370 | `declarations/functions/default-arguments/outside-default-makes-default-constructor.cpp`, `outside-default-makes-copy-constructor.cpp`, `outside-default-makes-move-constructor.cpp` | A later declaration can add defaults, and the principle names no limit for constructors. Accept. | no | supports the provisional answer (reject) with no doubt |
| SB-P2 | Q-0376 | `declarations/structured-bindings/tuple-index-size-type.cpp` | The principle says the elements are the results of get, and does not give the type of the index. No answer. | no (not decided) | supports the provisional answer with no doubt |
| CONV-P2, LSUP-P1 | Q-0386 | `foundations/language-support/ordering-strong-only-builtin.cpp`, `ordering-strong-only-builtin-runtime.cpp`, `ordering-strong-only-defaulted.cpp`, `ordering-partial-not-declared.cpp`, `overloading/overload-resolution/builtin-three-way.cpp`, `builtin-three-way-mixed.cpp` | Each use needs only the class it yields. The two strong-only built-in programs and the defaulted program run. The program that compares two double values and declares only the strong class is rejected. The two decltype programs are accepted, and int against double gives the partial class, since the operator lets that narrowing through. | yes | supports with a doubt |
| ATTR-P1 | Q-0390 | `declarations/attributes/fallthrough-into-outer-switch.cpp` | The next statement carries a label of the outer switch, which is not the switch of the fallthrough. Reject. | yes | supports with no doubt |
| SB-P2 | Q-0395 | `declarations/structured-bindings/tuple-member-variable-template-get-uses-free.cpp`, `tuple-member-class-template-get-uses-free.cpp` | The principle says the elements come from get, and does not say when the member form is chosen. No answer. | no (not decided) | supports the provisional answer with no doubt |

## Questions

| Question | Verdict | Reason |
|---|---|---|
| Q-0018 | supports with a doubt | The reference makes the enumeration ill-formed when no integral type of the implementation fits, and the implementation chooses among the types that fit. A wider type is fine only if it counts as an extended integer type, and GCC documents none. |
| Q-0175 | supports with a doubt | For a class with a non-trivial constructor, the reference makes any reference to a member before the constructor starts undefined. For an aggregate with no constructor, the answer rests on the lifetime rule, and the reference's own example on scalars shows that a read of the zero before dynamic initialization is meant to work. |
| Q-0184 | silent | The question is about what Tidal's Evidence records. The reference says nothing on vendor options. |
| Q-0318 | supports with a doubt | C++23 gives no explicit rule that indirection through a null pointer is undefined. The answer rests on the indirection result naming the object the pointer points to, and a null pointer points to none. A later defect resolution (CWG2823) makes such an indirection explicitly undefined, which would remove the doubt. |
| Q-0325 | silent | The reference leaves the mapping from integers to pointers to the implementation and has no notion of an object that exists at a fixed address without being created. |
| Q-0328 | reads against | Zero initialization sets padding bits to zero. So a padding byte of a static object, or of a class object that is value initialized with no user-provided constructor, has a value before the program writes it. The part on copies through plain char is unclear in the reference, which speaks of unsigned ordinary character types without saying whether a plain char that is unsigned counts. |
| Q-0357 | reads against, with doubt | The reference's guarantee for byte copies excludes potentially overlapping subobjects, and a base subobject is one. So the copy into the base has no meaning in the reference (undefined by omission). Option 2 is closer. |
| Q-0359 | supports with a doubt | The reference makes an access to an object defined volatile through a glvalue that is not volatile undefined. Whether a byte read of the object representation counts as such an access is not spelled out. |
| Q-0372 | silent | The reference does not say what a byte read gives when the byte holds bit-field bits and bits that no member covers. It does say that zero initialization zeroes padding bits. |
| Q-0386 | supports with a doubt | The reference asks only that a declaration of the class that a use needs precede the use. The tests' uses need only the strong class. Two doubts remain: the reference asks for the library's own declaration, and a program that declares names in namespace std itself is undefined by the library rules. Also, a defaulted operator that returns the weak or partial class uses the strong class too. |
| Q-0389 | reads against | The reference forbids a local of an enclosing function in a default argument only where the local is odr-used. Reading the value of a local integer constant that is usable in constant expressions is not an odr-use, so option 2 holds. |
| Q-0390 | supports with no doubt | The reference asks that the next statement carry a case or default label of the same switch statement. |
| Q-0393 | reads against | The operand must be a constant converted to bool using only a fixed list of conversions. The conversion from the null pointer type to bool is not on that list, so option 2 holds. |
| Q-0395 | supports with no doubt | The reference chooses the member form only when the search for get in the class finds a member function template that takes a value as its leading parameter. A member variable template or class template does not count. |
| Q-0402 | supports with a doubt | The reference makes lookups in the implicit definition of a defaulted comparison behave as from the function body, which sits inside the class, before the later declaration of the ordering class. So the ordering class must be declared before the defaulted declaration. The reference does not say plainly where the return type is deduced. |

## ALN-P1, ALN-8

The reference converts an operand of literal class type to an integral
or unscoped enumeration type by the contextual implicit conversion. That
conversion looks at the conversion functions that are not explicit and
asks that exactly one target type be possible. Two such functions with
the same target type, such as one const and one not const, are allowed,
and overload resolution then picks one. The chosen function must be a
constant function. Tidal's text asks for exactly one function. A class
with two functions to the same type would be rejected by Tidal and
accepted by the reference. The test of two functions to different
integer types agrees with both readings.

## ENUM-P1, ENUM-10

The reference fixes the type of each enumerator inside the list, for an
enumeration with no fixed type, this way:

- With an initializer of unscoped enumeration type, the enumerator gets
  the underlying type of that enumeration. ENUM-10 says it gets the
  enumeration type itself. ENUM-P1 says only "the type of the
  initializer", which a reader takes to be the enumeration type.
- With no initializer on the first enumerator, the enumerator gets a
  signed integer type that the reference leaves unspecified. ENUM-P1
  and ENUM-10 now say int. Every matrix compiler may well use int, but
  the reference does not require it, so the class of that row is
  unspecified, with int as the matrix's choice.

The other rows agree.

## ENUM-P3, ENUM-5

The reference makes an enumeration ill-formed when no integral type can
hold all its values, so a diagnostic is required. An implementation can
add extended integer types, and these count as integral types. GCC
states that it has no extended integer types, and treats its 128-bit
type as an extension. So GCC's acceptance with no diagnostic under
strict options is doubtful. ENUM-5 says the implementation "can use a
wider type or reject". That fits the reference only if the wider type is
one of the implementation's integral types. The doubt is the same as
for Q-0018.

## DEFARG-P1

The new paragraph on immediate calls matches the reference. The list of
names that a default argument cannot use still includes the locals of
an enclosing function, with an exception only for unevaluated
operands. The reference bars a local only where the default argument
odr-uses it. A read of the value of a local integer constant with a
constant initializer is not an odr-use, so the reference allows it.
Q-0389 holds the matrix's answer against the reference.

## INIT-P2, INIT-1

The rules now require that the value have the byte type, which fixes
the last review's gap. Two small points remain:

- The reference also lets a discarded-value expression produce such a
  value without undefined behavior. INIT-1 lists only copies.
- The reference speaks of unsigned ordinary character types. Whether
  plain char counts when the implementation makes it unsigned is not
  clear. Tidal names unsigned char only.

## SINIT-5

The reference lets an implementation defer the dynamic initialization
of a variable that is not inline. A deferred initialization must happen
before any odr-use of a function or variable of the same unit that is
not inline. A use counts only if it does not come, at any remove, from
the initialization of a static or thread variable. SINIT-5 differs in two
ways:

- It counts any first use, including a use made by another variable's
  initializer. In the reference such a use does not force the
  initialization.
- It counts uses of inline entities. The reference excludes them, and
  gives inline variables a separate choice that ends at the first use
  of that variable alone.

The new sentence that a use in a constant initializer "counts as a use
before any dynamic initialization" has no clear source in the
reference. A use made by a constant initializer is caused by the
initialization of a static variable, so in the reference it does not
force any deferred initialization.

## NS-P2, NS-12

The new sentences agree with the reference's example for using
directives, where a variable in one nominated namespace and a class in
another make a qualified name ambiguous. Qualified lookup treats the
inline namespace set the same way. The doubt is that the reference
states the rule that a class is dropped in favor of other declarations
as a general rule of lookup, with a note that limits it to one scope.
The example settles the reading that NS-12 takes.

## CONV-P3, CONV-17

CONV-P3 lists a virtual base among the bases that give no single path,
and says a conversion through such a base is not implicit. For a
pointer to an object, the reference allows the implicit conversion to a
pointer to an unambiguous and accessible virtual base. Only the
conversion of a pointer to member excludes a virtual base. CONV-17 says
this correctly, and CONV-P3 does not.

On overload resolution, the reference says that a conversion sequence
ignores access, so a call that chooses a conversion to an inaccessible
base is formed and then rejected. For an ambiguous base, the reference
says only that a program needing the conversion is ill-formed. Treating
the ambiguous case like the inaccessible one is the likely reading, and
compilers do so, but the text is less direct.

## LSUP-P1, LSUP-2

The reference ties the requirement to each use of an ordering class,
including an implicit use. It agrees that a use of one class needs no
declaration of the others. One case is missing. A defaulted three-way
operator returns, when all parts compare equal, the strong class's
equal value converted to the return type. So a defaulted operator whose
return type is the weak or the partial class also uses the strong
class and its conversion to the return type. LSUP-P1's table names a
conversion only when a part gives a stronger class, and LSUP-2 says
the other classes are not needed.

A second doubt covers the whole page. The reference asks that a
standard library declaration of the class precede the use. A program
that declares the class in namespace std itself is undefined by the
library rules. Tidal's tests rely on the matrix compilers accepting a
program's own declaration.

## NAM-21

The reference puts in the scope of the named class or namespace
everything after the qualified name of a declaration that is not a
friend declaration. For an enumeration, that part includes the
underlying type and the enumerator list. NAM-21 names the list and
leaves out the underlying type. NAM-21 also does not say that a friend
declaration with a qualified name gets no such lookup.

## TR-4

LNK-7 needs no diagnostic, and that fits a used function with no
definition, which the reference also exempts. The reference requires a
program to contain exactly one main function in the global scope, in a
hosted environment, and gives that requirement no exemption from
diagnosis. A diagnostic at link time counts, since linking is part of
translation. So a program with no main function must be diagnosed, and
TR-4 should not file it under LNK-7. Every matrix compiler fails such a
link, so no test result changes.

## TR-13

The reference describes an implementation-defined limit on the total
depth of recursive instantiation, which can involve several templates,
and leaves the result of an endless recursion undefined. EDG counts the
depth of each template on its own. That count is a choice the
implementation documents, so it can stand as implementation-defined,
but it does not match the reference's description of a total depth.
The reference's annex of limits suggests at least 1024, and EDG's 64 is
below it. The annex is guidance only.

## VC-P3

The reference has two cases where temporaries of an array element's
initialization end before the next element: a default constructor for
an element with no initializer, and a copy constructor for an element
while the whole array is copied, as in a lambda capture or an implicit
copy constructor. In each, the temporaries of the default arguments end
before the next element. VC-P3 says "by a constructor with a default
argument", which also covers an element that has its own initializer
in braces. The reference treats such elements as part of one full
expression. VC-P3 also names only REF-4 as a way to extend a
temporary's life. The reference adds the range-for initializer, which
`statements/range-for.md` covers.
