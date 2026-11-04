# Gap review: rules that phase 7 added or reclassified

Date: 2026-11-04
Role: source reader

Reference: the C++23 working draft in the local reference tree. The
review compares the 13 rules that phase 7 added, the 7 rules whose class
phase 7 changed, and the questions Q-0362 to Q-0368 with that draft
only. Later defect resolutions are mentioned only where the compilers
may follow them. For CLS-35 the reference leaves most of the layout to
the implementation, so the review checks only the facts that the
reference fixes.

## Result

Four provisional answers agree with the reference: Q-0363, Q-0365,
Q-0367, and Q-0368. Two contradict it. For Q-0362 and Q-0366 the
reference supports option 2. For Q-0364 the reference is unclear, and
its text read literally supports option 2.

Most rules agree. The gaps are these:

- COND-16 leaves out the special targets for two operands of related
  class types, and the cv condition on them.
- ODR-15 lists too few exceptions. Several kinds of variable
  declaration are not definitions in the reference.
- OBJ-11 ties a deferred initialization to the first use of the object
  itself. The reference ties it to the first use of any thread-duration
  variable of the same translation unit.
- OBJ-12 leaves out block variables of thread storage duration, and
  drops the condition that the call happens while an object of static
  or thread storage duration is destroyed.
- TR-3, which Q-0366 affects, rejects only C language linkage on the
  main function. The reference rejects any linkage specification on it.
- DEFARG-10 does not mention two exceptions to checking at the point of
  the default argument.
- CTOR-17 does not cover a member of the standard initializer-list type
  set from a braced list in a constructor's member initializer list.
- LSUP-5 and STG-25 promise results that the reference does not give.
- PRIM-35 calls the escape conditionally supported in a string literal.
  The reference gives it only an implementation-defined code unit
  sequence there.

## Rules

| Rule | Verdict | Note |
|---|---|---|
| ATTR-20 | agrees | The reference makes the behavior of an attribute that it does not specify implementation-defined, and an implementation ignores such an attribute when it does not recognize it. Both rows of the matrix table fit. |
| CINIT-3 | agrees | The reference asks that the initializing declaration carry the specifier when any declaration does, and waives the diagnostic when no declaration with the specifier is reachable from the initializing one. A later declaration in the same unit is such a case. CINIT-2 covers the diagnosed order. With modules or several units, "comes after" is a looser test than reachability. |
| CLS-35 | agrees within what the reference fixes | See "CLS-35" below. |
| CNS-40 | agrees, one narrower condition | See "CNS-40" below. |
| COND-16 | missing conditions | See "COND-16" below. |
| DEFARG-10 | agrees, two missing details | See "DEFARG-10" below. |
| DEFARG-11 | agrees | The reference says the same for function templates. It asks that no other declaration be reachable from the friend declaration, or reach it. Without modules this is the whole translation unit. |
| DEFARG-12 | agrees | The reference also names copy and move assignment operators. An assignment operator cannot take a default argument in any case, so nothing is lost. The reference treats members of class templates apart: their default arguments must all appear on the declaration in the class. DEFARG-12 does not say whether another rule covers that case. |
| OBJ-12 | missing condition, narrower scope | See "OBJ-12" below. |
| ODR-15 | missing exceptions | See "ODR-15" below. |
| PRIM-35 | agrees for character literals, doubt for strings | The set of characters matches. The escape uses a basic character that no other escape form starts with, so octal digits, the simple escape letters, and the five letters of the named, braced octal, universal, and hexadecimal forms are out. A character literal made of one such escape is conditionally supported with an implementation-defined value. For a string literal the reference says only that each such escape gives an implementation-defined code unit sequence. It does not label the string case conditionally supported. |
| TR-16 | agrees, one missing item | The reference leaves the match between translation and run time unspecified, in a note. Next to that note it recommends that implementations give consistent results. TR-16 and TR-5 do not mention the recommendation. |
| UNW-11 | agrees | The reference makes the program ill-formed when the lookup for a placement form finds a usual deallocation function that would match the allocation function if it were treated as a placement form. The reference's own example is the one UNW-11 describes, with the extra size parameter. |
| CNS-39 | agrees | The reference makes a program ill-formed with no diagnostic needed when no satisfaction check of an atomic constraint could be well-formed and no check is performed. A non-dependent expression whose type is not bool is such a constraint. When a check reaches it, the error is diagnosed, as CNS-9 says. |
| TARG-22 | agrees | Two template arguments that are templates are the same only when they name the same template. An alias template is a template of its own, even when its specializations equal those of the class template. A long-standing core issue asks whether such an alias should count as the class template. As far as I know, it has no resolution that changes C++23. |
| STG-25 | doubt | See "STG-25" below. |
| OBJ-11 | missing condition | See "OBJ-11" below. |
| LSUP-5 | doubt on the classification | See "LSUP-5" below. |
| CTOR-17 | agrees, one missing case | See "CTOR-17" below. |
| DCMP-8 | agrees | The reference has one general rule for any function defaulted after its first declaration: a program where such a function would end up deleted is not valid. The rule applies when the class definition sees the defaulting, whether or not anything uses the function. |

## Questions

| Question | Provisional answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0362 | The member of the anonymous union can share the name (option 1). | contradicts | option 2 | See below. |
| Q-0363 | The handler never matches (option 1). | agrees | option 1 | The reference turns a handler of array or function type into a pointer handler. A handler of reference to array or reference to function is not adjusted. The exception object has pointer type after the decay, and none of the match cases pairs a reference to an array or a function with a pointer. Clang follows the reference. |
| Q-0364 | Argument-dependent lookup always adds its functions (option 1). | unclear | option 2 on a literal reading | See below. |
| Q-0365 | The result is destroyed (option 1). | agrees | option 1 | The reference says outright that when a destructor of a local or a temporary throws during a return statement, the returned object is destroyed as well. Its example also shows the case with a handler in the same function and a second return statement: the first result is destroyed, and the second return builds a new one. This came from a core defect resolution that Clang appears not to implement. |
| Q-0366 | Accept the form (option 1). | contradicts | option 2 | See below. |
| Q-0367 | Follow the review in each form (option 1). | agrees | option 1 | See below. |
| Q-0368 | The write is outside the lifetime and undefined (option 1). | agrees | option 1 | The reference starts the lifetime of an object of any type when its storage is obtained and its initialization is complete. It makes no exception for scalars. A write, or a read, during the variable's own initializer touches the object before its lifetime begins, which is undefined, so constant evaluation rejects it. I know of no resolution that gives scalars an earlier start. |

### Q-0362

- The reference has a list of pairs of declarations of one name that do
  not conflict in one scope. One pair is a type and a non-static data
  member. The reference excludes a data member of an anonymous union
  from that pair by name.
- A member of an anonymous union is bound in the scope of the class
  that contains the union. So the nested class and the union member
  have one name in one scope and conflict. The program is ill-formed.
- Clang follows the reference. GCC and EDG accept. The answer goes
  against NAM-P10's general model, so NAM-39 needs the exception if
  Tidal follows the reference.

### Q-0364

- The reference builds the non-member operator candidates by the
  unqualified lookup of the operator function name in the call that the
  operator expression is rewritten to. It points to both the ordinary
  lookup rules and the argument-dependent lookup rules for that call.
- The argument-dependent lookup rules turn the lookup off when ordinary
  lookup finds a function declared in a block scope. Read literally, a
  block declaration of an operator function then turns it off for the
  operator too, as EDG does.
- The reference also says to ignore member functions in that lookup.
  If that step came before the test, a member found by ordinary lookup
  inside a member function would not turn argument-dependent lookup
  off. The text does not fix the order of the two steps. This makes the
  whole rule less clear than it looks, and the block-scope case could
  have been missed by the authors. I know of no resolution on the point.
- Clang and GCC keep argument-dependent lookup. The provisional answer
  is a defensible reading of the intent, and the literal text supports
  option 2.

### Q-0366

- The reference forbids declaring the main function with a linkage
  specification of any kind, braced or not, and whatever the language
  named.
- The reference also makes the linkage of the main function
  implementation-defined. It separately rejects C language linkage for any function or variable
  called main, whatever its namespace, which TR-3 already states.
- So EDG follows the reference, and TR-3's bullet on main is narrower
  than the reference.

### Q-0367

- In the reference, a variable usable in constant expressions with no
  mutable subobjects is not used when the name is one of the potential
  results of an expression of non-class type that is read by an
  lvalue-to-rvalue conversion. Also, any variable of non-reference type
  is not used when the name is one of the potential results of a
  discarded-value expression to which that conversion is not applied.
- Forms 1 and 2 are discarded-value expressions. The conversion is not
  applied to a non-volatile operand there, so neither form is a use.
  The second case does not need the variable to be usable in constant
  expressions, nor a type without mutable members.
- For form 3, the potential results of a subscript with an array
  operand include those of the array. Whether the element is read or
  discarded, the array is not used.
- For form 4, taking the address is a use. The reference exempts a
  discarded statement only from the need for a definition. A note on
  lambda captures says that capture is decided by the program text,
  also inside a discarded statement. So a lambda with no captures that
  takes the address there is ill-formed, as GCC and EDG find.

### CLS-35

- The reference fixes these layout facts: non-static data members of
  nonzero size, outside unions, have increasing addresses in
  declaration order; an empty base whose class is standard-layout has
  zero size; two subobjects of one type never share an address unless
  one contains the other; an object of nonzero size that is not a
  potentially overlapping subobject takes its full size in bytes that
  no unrelated object uses; a standard-layout class has its first
  member at its own address; and the size of a class leaves room for
  arrays, so it is a multiple of the alignment.
- The table and the end-padding conditions agree with all of these
  facts. The rows "Two empty subobjects of one type" and "Any other
  member" restate required behavior, and the order of members is
  required by CLS-10. The other rows are choices that the reference
  leaves open.
- The reference makes the zero size of an empty base whose class is
  not standard-layout implementation-defined, and alignment
  implementation-defined. The place of the pointer for virtual calls,
  padding, the order of bases, and the reuse of end padding are left
  open with no label. CLS-34's class of unspecified fits that.
- The end-padding conditions for the matrix are a property of the
  platform ABI, and the reference says nothing on them.

### CNS-40

- The reference treats an abbreviated function template and the
  written form with the invented parameter as functionally equivalent
  and not equivalent. Its own example says so.
- The reference makes a program ill-formed with no diagnostic needed
  when two function templates that do not correspond have one name,
  corresponding signatures, would declare one entity, and accept and are
  satisfied by the same template argument lists.
- CNS-40 asks for the same constraints. The reference asks only that
  the same argument lists pass, so two templates with different but
  logically equal constraints also fall under the rule. CNS-40 also
  does not state that both declarations must declare one entity, for
  example in the same namespace.
- The draft carries editorial notes on this wording that ask what two
  of its terms mean, so the rule itself is not settled text.

### COND-16

- The reference uses COND-16's three steps for an lvalue, an xvalue,
  and then a prvalue target. The third step applies when the other
  operand is a prvalue, or when neither reference binding works and one
  operand has class type. COND-16 gives the same order.
- In the third step the target is not always the other operand's type.
  When both operands have the same class type apart from cv-qualifiers,
  the target is the other type only when it is at least as
  cv-qualified. When the other type is a base class of the operand's
  type, the target is that base with the operand's own cv-qualifiers.
  Otherwise the target is the other operand's type after the
  lvalue-to-rvalue, array-to-pointer, and function-to-pointer
  conversions. COND-16 states none of these three cases.
- The reference ignores access, bit-fields, and whether a conversion
  function is deleted when it decides whether a sequence can be formed.
  COND-16 does not say so. A later error from the conversion itself can
  still make the program ill-formed.
- The reference also runs the process for two glvalues of one value
  category and one type apart from cv-qualifiers. COND-3's first rows
  give the same results for that case.

### DEFARG-10

- The reference checks a default argument where it appears, with an
  exception for an immediate function call inside it. That call is
  neither evaluated nor checked as a constant expression at that point.
  DEFARG-10 does not mention the exception.
- A default argument of a member function is a complete-class context.
  Names declared later in the class are found, and the check happens
  as if at the end of the class. DEFARG-10 says "at the declaration",
  which is not the point of the check for a member function.

### OBJ-12

- In the reference, the behavior is undefined in this case. A block
  variable with static or thread duration has already ended. Its
  function then runs while some other static or thread-duration object
  is being destroyed, and control reaches the block variable's
  definition again.
- OBJ-12 leaves out block variables of thread storage duration.
- OBJ-12 does not require that the call happen during such a
  destruction. A function registered to run at program exit can reach
  the same point. The reference does not name that case, so OBJ-12 is
  wider than the reference there. The general lifetime rules may cover
  the case anyway once the object is used.

### ODR-15

The reference says these variable declarations are also not
definitions, and ODR-15 does not list them:

- a parameter in a function declarator that is not part of a function
  definition,
- a declaration outside the class of a static data member that was
  defined in the class with the constant-expression specifier, a form
  the reference marks deprecated,
- an explicit instantiation declaration of a variable template or
  static data member,
- an explicit specialization with no initializer, when it specializes
  a static member variable that belongs to a class template, or a
  member variable template.

ODR-15 with LSP-3 handles the braced linkage specification correctly.
The reference says braces do not change whether a declaration is a
definition.

### STG-25

- The reference grants the byte-copy result only when neither the
  source nor the target is a potentially overlapping subobject. For a
  base or a marked member, the reference does not say what a byte copy
  into it does.
- STG-25 says the other subobject whose bytes are written takes the
  value its new bytes give. The reference does not state that result,
  and it leaves even the target's own value open. The case is
  undefined by omission in the reference. STG-25 and Q-0357 promise
  more than the reference gives.

### OBJ-11

- The reference makes it implementation-defined whether the dynamic
  initialization of a thread-duration variable at namespace scope that
  is not inline happens before the thread's initial function starts or
  is deferred. That matches OBJ-11 and SINIT-6.
- If the initialization is deferred, it happens before the thread's
  first use, other than for initialization, of any non-inline
  thread-duration variable defined in the same translation unit. The
  trigger is not only the first use of the object itself. OBJ-11 says
  "when the thread first uses the object", so it leaves out an earlier
  start caused by another variable of the same unit.
- The reference also makes it implementation-defined in which threads
  and at which points a deferred initialization happens. Inline
  variables and block variables follow other rules.

### LSUP-5

- The reference's library rules make the behavior undefined when a
  program adds a declaration to the standard namespace, unless a rule
  allows it. No rule allows a program to declare the alignment
  enumeration.
- A note in the reference says the implicit declarations of the
  allocation functions do not declare that type, and a program may name the type
  only after the library's own declaration of it.
- So a program that declares the type itself, with any shape, has
  undefined behavior in the reference. LSUP-5 implies the right shape
  is valid and calls another shape not valid with no diagnostic. The
  reference gives no special status to the right shape. The class
  matches the compilers, but the reference supports undefined for
  every shape. LSUP-P2's model of a program that supplies library
  declarations is Tidal's own choice.

### CTOR-17

- The reference makes it ill-formed to bind a temporary to a reference
  member in a member initializer, and also from a default member
  initializer. The second case applies only where a constructor uses
  the default member initializer, so a defaulted constructor that is
  never used is fine. CTOR-17's tests fit both.
- The reference also uses the first case for a member of the standard
  initializer-list type set from a braced list in a member initializer.
  The backing array then binds as if to a reference member, and the
  program is ill-formed. CTOR-17 does not cover that case.
- The reference states the default member initializer case without
  limiting it to constructors. The text does not make clear whether it also covers
  an aggregate whose initialization falls back on that initializer. CTOR-17 speaks only of constructors, which avoids the
  doubt.
