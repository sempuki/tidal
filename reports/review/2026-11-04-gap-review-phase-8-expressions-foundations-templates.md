# Gap review: the text phase 8 wrote on the expressions, foundations, exceptions, and templates chapters

Date: 2026-11-04
Role: source reader

Reference: the C++23 working draft in the local reference tree.
The review covers each rule and principle on the pages under
expressions, foundations, exceptions, and templates whose prose phase 8
added or changed (commits dc90663 to 1b16115, test lines ignored). It
also reads the 25 held questions whose widened principle is on these
pages, and the questions Q-0377, Q-0378, Q-0380, Q-0382, Q-0383,
Q-0385, and Q-0386. Q-0379, Q-0381, and Q-0384 affect rules on other
pages and are left out. Later defect resolutions are mentioned only
where they matter. The draft already holds the resolutions that let a
constant-evaluated function never complete, that allow unknown
references in constant expressions, and that exempt narrow bit-fields
from narrowing.

## Result

Most of the new text agrees with the reference. The gaps are these:

- COND-16 puts a cv condition on the base-class row of its table. The
  reference has no such condition for a base class. In one kind of
  program the two readings differ on whether both operands convert.
- CMP-P3 says that comparisons inside one complete object are accepted
  in constant evaluation. That holds for array elements and for later
  members of one class. A relational comparison of two base
  subobjects, of a base and a member, of zero-size members, or of union
  members has no fixed result, even inside one object.
- PFX-22 leaves out that an argument of the null pointer type becomes
  a pointer to void. It also says a scoped enumeration keeps its type
  with no further note. The reference makes passing such an argument
  conditionally supported.
- UNW-2 now says the captures of a closure are initialized in the order
  of the capture list (through LAM-28). The reference leaves that order
  unspecified.
- TARG-13 leaves out floating-point conversions and promotions. The
  reference's list for converted constants has neither.
- INST-P3 says an explicit instantiation in an enclosing namespace must
  qualify the template name, for a class, a function, and a variable
  template alike. The reference keeps that need only for the function
  case.
- Smaller doubts: INST-33, LAM-P3, PFX-P3, TR-4, LSUP-7, CE-9, DEP-30,
  NAM-21, STG-12, INST-P1, ORD-P1, and CNS-7 with CNS-10 and CNS-35.

Of the 25 held questions, the reference agrees with the provisional
answer in 21. It contradicts Q-0235 in part (for the class and the
variable forms it supports option 2). It is unclear for Q-0221 and
Q-0274, and for Q-0290 it agrees with small doubt. In every row the
principle's model gives the provisional answer.

Of the seven listed questions, five agree with the reference: Q-0377,
Q-0378, Q-0380, Q-0382, and Q-0385. Two contradict it. For Q-0383 the
reference makes the declaration of the literal operator not valid with
no diagnostic required. For Q-0386 the reference supports option 3.

## Rules

| Rule | Verdict | Note |
|---|---|---|
| ADD-4, BIT-4, MUL-5 | agrees | When the common type is unsigned, the usual arithmetic conversions turn a negative signed operand into a large unsigned value. |
| ASN-5 | agrees | The reference defines the compound form by the plain form with the left side evaluated once. |
| CAST-P3, CAST-6 | agrees | A byte array element and an object that the array hosts are not pointer-interconvertible, so the cast result stays on the byte. |
| CMP-P1 | agrees | The three-way operator rejects a narrowing conversion other than integer to floating-point. |
| CMP-P3 | gap | See "CMP-P3" below. |
| CMP-P5 | agrees | The three-way operator takes an enumeration operand only against an integer operand or two arithmetic operands. An unscoped enumeration against a floating-point value fits neither case. |
| CMP-13 | agrees | Term change only. |
| COND-P2 | agrees | The null pointer rows match. The built-in candidates for the conditional operator are each pair of promoted arithmetic types, floating types included, and a same-type pair for pointer, pointer-to-member, and scoped enumeration types. |
| COND-3 | agrees | For two glvalues of one non-class type that differ only in cv-qualifiers, neither converts, and after the lvalue-to-rvalue step both are the same type, so the result is a prvalue. For class type the same pair goes to the overload step, and that step fails. |
| COND-5 | agrees | Both convert, or one conversion is ambiguous: not valid. Neither converts and overload resolution fails: not valid. |
| COND-16 | extra condition | See "COND-16" below. The rest agrees, including the note that access, bit-fields, and deleted conversion functions do not count, and that the conversion can still fail later. Tidal's term "bind directly" excludes a temporary, which matches the reference's need for a glvalue. |
| LAM-P3 | agrees, narrower condition | The reference allows a capture default only when the innermost enclosing scope is a block scope, or in a default member initializer whose innermost scope is that class. "Has no enclosing function" is looser. A lambda in a default argument, or in a member declaration of a local class outside any body, has an enclosing function but no block scope, and the reference rejects a capture default there. |
| MPTR-P3 | agrees | The reference has no special rule for a converted member pointer that passes through an object or a function result. |
| MUL-P3 | agrees | Advice only. |
| NEW-P1 | agrees | The reference converts the array size to the size type by an implicit conversion. A floating value and a class with a suitable conversion work. A scoped enumeration has no implicit conversion. |
| NEW-P2 | agrees | The reference makes an array delete undefined when the dynamic type differs from the static type, with no exception for a virtual destructor. |
| NEW-P3 | agrees, one doubt | The reference does access and ambiguity checks on the deallocation function for every allocation of class type, whether or not the initialization can throw. It does not say outright that this selection is a use for the deleted-function rule. See Q-0221 below. |
| NEW-P4 | agrees | The nothrow forms are replaceable global allocation functions, and constant evaluation may use those. |
| NEW-22 | agrees | The reference's selection order: a class's functions pick the form without the size. Otherwise a complete type picks the sized form, for an array only when the element is a class with a non-trivial destructor. The rest is unspecified. |
| PFX-P2 | agrees | |
| PFX-P3 | agrees, two missing details | See "PFX-P3" below. |
| PFX-22 | gap | See "PFX-22" below. |
| PRIM-P1, PRIM-1 | agrees | The table matches the reference's lists, including the size suffix in a non-decimal base. The reference also lets a literal that no listed type holds take an extended integer type of the matching signedness. On the matrix there is none, so "rejected" holds there. |
| PRIM-P6 | agrees | Each operand of a fold is a cast expression in the grammar. |
| PRIM-27 | agrees | The initial operand joins the innermost pair in both binary forms. |
| UN-3 | agrees | |
| CE-P1 | agrees | The draft no longer has the rule that a function marked for constant evaluation must be able to produce a constant. |
| CE-P4, CE-19 | agrees | The conditions of the constant if statement and the static assertion allow narrowing to bool. The other bool places do not. |
| CE-3 | agrees | A constant expression of reference kind must refer to an object of static storage that is not a temporary, a suitable temporary, or a function. |
| CE-8 | agrees | The draft excludes every throw expression from a core constant expression, a rethrow included. |
| CE-9 | agrees, one doubt | Reading after a lifetime ends is core-language undefined behavior. The reference requires constant evaluation to reject it. The sentence "such an operation" follows the sentence about CE-26, so it can be read as an operation that need not be detected. The intended reading agrees. |
| CE-18 | agrees | The draft holds the resolution on unknown references. A call of a static member through such a reference reads nothing. |
| CONV-P2, CONV-21 | agrees | The table matches the reference's list, including the bit-field exception and the unscoped enumeration source. |
| CONV-P3 | agrees | A null pointer constant is an integer literal with value zero or a prvalue of the null pointer type. A parenthesized literal is the same expression. A character literal and the bool literal are not integer literals. |
| CONV-19, CONV-20 | agrees | |
| SEQ-P1 | agrees | The function part is sequenced before the arguments. The initialization of each parameter is indeterminately sequenced with the others. The draft counts the implied object argument among the arguments for an explicit object parameter. |
| KW-3 | agrees | The only class marker in the grammar is the final marker, and the reference treats that word as the marker when it follows a class name and comes before a brace or colon. |
| KW-4 | agrees | The reference makes a declaration of a literal operator with a reserved suffix not valid, with no diagnostic required. This conflicts with OPR-9, which KW-4 now names. See Q-0383. |
| LSUP-7 | doubt | The library reserves its non-allocating placement forms, and a program that supplies its own versions of them breaks that reservation. A program that defines them itself has undefined behavior by the library rules. The same doubt as the earlier review of LSUP-5: LSUP-P2's model is Tidal's own. |
| NAM-3 | agrees | A declaration in the outermost block of a function body that conflicts with a parameter makes the program ill-formed. |
| NAM-6, NAM-37 | agrees | See Q-0316. |
| NAM-21 | agrees, one missing case | See Q-0377. The reference gives the same treatment to the body of an enumeration defined with a qualified name. |
| NAM-30 | agrees | |
| OBJ-11 | agrees | The repair matches the earlier review. |
| OBJ-12 | agrees | The repair matches the earlier review. |
| ODR-10 | agrees | A type alias names a type, and the names in the definitions must refer to the same entities. |
| ODR-15 | agrees | The four new items match the earlier review. |
| STG-P2, STG-29 | agrees | |
| STG-8 | agrees | The reference says any call of a function named operator new creates objects implicitly, so a class's own function counts. |
| STG-12 | agrees, one doubt | The reference's exceptions for indeterminate values name an "unsigned" ordinary character type and the byte type. The draft does not define the phrase. One reading includes plain char where char is unsigned. On the matrix targets char is signed, so STG-12 holds there. |
| STG-18, STG-19 | agrees | An assignment to a union member can begin that member's lifetime and create its object. A reference to the old member does not reach the new object, because the two objects differ in type. |
| STG-23 | agrees | The reference's aliasing list does not name an enumeration and its underlying type. |
| TR-4 | doubt on the classification | The reference waives the diagnostic for a used function with no definition. For the main function it states only that the program contains exactly one. That rule has no waiver, and a freestanding implementation decides whether it needs the function at all. In practice the linker reports both. |
| TYP-1, TYP-2, TYP-4 | agrees | |
| TYP-3 | agrees | The digits have consecutive values on every implementation, not only on the matrix. The letters are a matrix fact. |
| UNW-11 | agrees | The sized global deallocation function is a usual function that every unit declares. A global placement allocation function with one extra size parameter therefore matches it, and the program is ill-formed. |
| UNW-2 | contradicts on one clause | See "UNW-2" below. |
| TARG-13 | missing case | See "TARG-13" below. |
| CTAD-P1 | agrees | The result of a guide is a simple template-id in the grammar, so a cv-qualifier cannot appear. |
| CPT-P3 | agrees | The reference forbids a default argument and a trailing ellipsis and sends the parameters to the rules for function parameters. A placeholder type is allowed only in the parameters of functions and lambdas. |
| CPT-26 | agrees | |
| CNS-P2, CNS-10 | agrees | The reference makes a different result for identical atomic constraints and arguments at two points ill-formed, no diagnostic required. Whether a change of access counts depends on CNS-7. |
| CNS-7 | silent | The reference does not say in which context access is checked during satisfaction. There is a core issue on this point, and I do not know its resolution. |
| CNS-35 | silent | The reference has no text on a satisfaction check that needs its own result. |
| CNS-27 | agrees | The reference treats two operands that do not involve template parameters as equivalent when their concept-ids name the same specialization, and its own note uses that very sum. Two expressions with template parameters that are functionally equivalent but not equivalent make the program ill-formed with no diagnostic required when the meaning depends on the difference. CNS-40 covers the declaration case. |
| CNS-37 | silent on the detail | The reference has no specific text on matching a constrained member template for an explicit specialization. Nothing contradicts CNS-37. |
| CNS-40 | agrees | The repair matches the earlier review. |
| TPL-6 | agrees | |
| TPL-23 | agrees | See Q-0385. |
| DED-P3 | agrees | The reference's own example deduces the type of a constant parameter from an array bound as the size type. A parameter that an alias template does not use cannot be deduced. |
| DEP-9, DEP-17 | agrees | |
| DEP-30 | agrees, one doubt | The override item fits the hypothetical instantiation rule. For a default argument that does not depend on the parameters, the reference does not say clearly whether the rule reaches it, since a default argument is instantiated only when used. |
| INST-P1 | agrees, one missing detail | The reference says a local class and its members, with their default arguments, exception specifications, and default member initializers, are instantiated with the enclosing entity. Type constraints and requires clauses of the local class are excluded from that. |
| INST-P3 | contradicts in part | See "INST-P3" below. |
| INST-22 | agrees | An explicit instantiation may not carry the constant-expression specifier, and the types must match. |
| INST-27, INST-28 | agrees | The exceptions list matches the reference: inline functions, deduced types, potentially-constant variables, and specializations of templated classes. The no-diagnostic rule applies to inline functions too. |
| INST-31 | agrees | |
| INST-33 | doubt | See "INST-33" below. |
| ORD-P1 | agrees, two missing details | The reference also gives the object parameter the cv-qualifiers of the member. When the choice depends on itself, the parameter is not taken as an rvalue reference. |
| PACK-P2 | unclear | See Q-0274 below. |

## Principle uses

| Principle | Question | Test file | The model's answer | Matches the provisional answer? | Reference verdict |
|---|---|---|---|---|---|
| NEW-P1 | Q-0036 | `array-floating-size-exact.cpp`, `array-non-integral-size.cpp` | The size converts by an implicit conversion, so a floating size is accepted (option 1). | yes | agrees |
| LAM-P3 | Q-0037 | `capture-default-at-namespace.cpp` | No enclosing function and no default member initializer, so rejected (option 1). | yes | agrees |
| CMP-P5 | Q-0050 | `three-way-enum-float.cpp` | An unscoped enumeration against a floating value has no lossless common type, so rejected (option 1). | yes | agrees |
| CMP-P3 | Q-0051 | `one-past-end-adjacent-constant.cpp` | Equality of a past-the-end pointer with another complete object depends on placement, so constant evaluation rejects it (option 1). | yes | agrees |
| PRIM-P1 | Q-0064 | `int-too-large-signed-decimal.cpp` | A decimal literal with no unsigned suffix has only signed types, none holds the value, so rejected (option 1). | yes | agrees, with the extended-integer allowance noted above |
| NEW-P2 | Q-0091 | `delete-derived-array-through-base-constant.cpp` | The array form through a base pointer is undefined, so constant evaluation rejects it (option 1). | yes | agrees |
| CONV-P3 | Q-0100 | `char-zero-not-null.cpp` | A character literal is a number, so no null pointer (option 1). | yes | agrees |
| COND-P2 | Q-0102 | `zero-and-nullptr.cpp` | A null pointer constant with the null pointer literal gives the null pointer type (option 1). | yes | agrees |
| PRIM-P6 | Q-0194 | `fold-operand-binary.cpp` | An operand holding a binary operator needs parentheses (option 1). | yes | agrees |
| DED-P3 | Q-0209 | `match-bound-type.cpp` | A bound counts elements, so the type of a size expression (option 1). | yes | agrees |
| ORD-P1 | Q-0214 | `member-object-rvalue.cpp` | Against an rvalue reference, the object parameter is an rvalue reference, so neither is more specialized (option 1). | yes | agrees |
| NEW-P3 | Q-0221 | `deallocation-deleted.cpp`, `deallocation-deleted-no-throw.cpp`, `deallocation-placement-deleted.cpp` | The expression uses the function in every case, so a deleted one is not valid (option 1). | yes | unclear, leans to option 1 |
| INST-P3 | Q-0235 | `explicit-unqualified-outside-namespace.cpp`, `explicit-unqualified-outside-namespace-variable.cpp` | Outside the template's namespace the name must be qualified, for all three kinds (option 1). | yes | contradicts for the class and variable forms (option 2); agrees for the function form |
| CTAD-P1 | Q-0242 | `guide-result-qualified.cpp` | The result only names the specialization, so no cv-qualifier (option 1). | yes | agrees |
| INST-P1 | Q-0253 | `local-class-default-argument.cpp` | A local class is instantiated whole with its function, default arguments included (option 1). | yes | agrees |
| CPT-P3 | Q-0259 | `requires-parameter-default.cpp`, `requires-parameter-ellipsis.cpp` | Nothing calls the expression, so no default argument and no ellipsis (option 1). | yes | agrees |
| CNS-P2 | Q-0264 | `satisfaction-changes.cpp` | A changed result is not valid, no diagnostic required (option 1). | yes | agrees |
| PACK-P2 | Q-0274 | `using-declaration-empty-pack-outer-name.cpp` | The name was bound at the definition and means nothing in the empty instantiation, no diagnostic required (option 1). | yes | unclear, leans to option 2 |
| CPT-P3 | Q-0290 | `requires-parameters-void-template.cpp` | The list has the form of a function's list, so a lone void means no parameters (option 1). | yes | agrees, small doubt |
| NEW-P4 | Q-0303 | `constant-evaluation-nothrow.cpp` | The nothrow forms are global functions of the implementation, so accepted (option 1). | yes | agrees |
| NEW-P1 | Q-0313 | `array-scoped-enumeration-size.cpp` | A scoped enumeration converts only with a cast, so no size (option 1). | yes | agrees for the array size; the other two places are on other pages |
| PFX-P3 | Q-0316 | `member-access-class-being-defined.cpp` | Inside the class definition a member declaration sees earlier members, so accepted (option 1). | yes | agrees |
| SEQ-P1 | Q-0319 | `explicit-object-order-unspecified.cpp` | The copy into the explicit object parameter is part of the arguments, in an order left open (option 1). | yes | agrees |
| MPTR-P3 | Q-0353 | `memptr.cpp`, `mixin.cpp` | The converted value names the same member wherever it goes (option 1). | yes | agrees |
| COND-P2 | Q-0373 | `two-classes-scoped-enumeration.cpp`, `two-classes-float.cpp` | Promoted arithmetic pairs with float among them, and a candidate for each scoped enumeration (option 1). | yes | agrees |

## Questions

| Question | Provisional answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0377 | The class body sees the qualifier's members (option 1). | agrees | option 1 | The reference puts the part of a redeclaration after a qualified class name, declarator name, or enumeration name inside the scope that the qualifier names. A specialization is covered too, so the tuple protocol example is valid. |
| Q-0378 | The deleted conversion counts, and a const and a volatile class lvalue do not convert (option 1). | agrees | option 1 | The reference ignores access, bit-fields, and deleted functions when it asks whether a sequence can be formed, so both operands of form 1 convert and the program is ill-formed. In form 2 neither reference binding works, and the same-class row needs the target to carry every qualifier of the source, which fails both ways. The overload step then finds no candidate. |
| Q-0380 | The size type (option 1). | agrees | option 1 | The reference's list for the size suffix in a non-decimal base is the signed type and then the size type. |
| Q-0382 | An identifier (option 1). | agrees | option 1 | The grammar's only class marker is the final marker. |
| Q-0383 | The literal calls the literal operator (option 1). | contradicts | neither option | See below. |
| Q-0385 | The definition repeats the whole head (option 1). | agrees | option 1 | The reference asks that a member defined outside the class template have a template head equivalent to the class's, and equivalence of heads includes the requires clauses. |
| Q-0386 | Every such use needs all three classes (option 1). | contradicts | option 3 | See below. |

### COND-16

- In the reference, the third step has three rows. The same-class row
  has the cv condition that COND-16 states. The base-class row has no
  cv condition. Its target is the base with the operand's own
  cv-qualifiers, and the operand converts whenever a sequence to that
  target exists.
- COND-16 adds "if T has every one of them, otherwise X does not
  convert" to the base row. Take a const derived lvalue against a base
  prvalue. In the reference the derived operand converts to a const
  base prvalue. In COND-16 it does not convert.
- Often the end result is the same, because a const base prvalue and a
  base prvalue still differ in type and go to the overload step, which
  has no candidate for a class type. The results differ when the base
  operand also converts to the derived type, for example through a
  converting constructor of the derived class. The reference then sees
  two conversions and rejects the expression as ambiguous. COND-16
  sees one and gives a derived prvalue.
- The second example sentence of COND-16 (neither operand converts)
  follows from the extra condition, so the reference does not support
  it either.

### CMP-P3

- In the reference, a relational comparison of pointers has a fixed
  result in two cases only. One is different elements of one array, or
  their subobjects. The other is different non-static data members of
  one object, or their subobjects, where the later member compares
  greater. That second case excludes members of zero size and members
  of a union.
- Every other relational comparison has no required result, also
  inside one complete object. Two base subobjects, a base and a member,
  and two members of a union are examples. Constant evaluation rejects
  a comparison whose result is unspecified.
- For equality, only the past-the-end case across two complete objects
  is unspecified. Inside one complete object, equality compares
  addresses and has a fixed result.
- So the closing paragraph of CMP-P3 is right for equality and for the
  two ordered relational cases. It is too wide for relational
  comparisons of other subobjects.

### PFX-P3

- The reference lets a member access name a member of a class that is
  not complete when the access appears in that class's definition. It
  adds that the program is ill-formed with no diagnostic required if
  the lookup result would differ in the complete class. PFX-P3 does not
  mention that condition.
- The reference's complete-class contexts are a function body, a
  default argument, a default template argument, an exception
  specification, and a default member initializer. They include those
  of nested classes defined inside the class. PFX-P3 names only the
  member function body.

### PFX-22

- The reference converts an argument of the null pointer type to a
  pointer to void before the call. PFX-22 says every type it does not
  list keeps its type, and that includes the null pointer type.
- The reference makes passing a potentially evaluated argument of
  scoped enumeration type through an ellipsis conditionally supported,
  with implementation-defined meaning. The same holds for a class with
  an eligible non-trivial copy or move constructor, or a non-trivial
  destructor. PFX-15 lists the class case only. PFX-22 states plainly
  that a scoped enumeration keeps its type, with no note on its
  support.

### UNW-2

- The reference initializes the closure's members in the order in
  which the closure type declares them. It states that this order is
  unspecified, and it says the same of the unnamed members for
  captures. Destruction runs in reverse of that order.
- So the reference does not tie the order to the capture list, for
  explicit captures or for init captures. The new clause in UNW-2, and
  LAM-28 that it cites, state an order that the reference leaves open.
  The compilers may all follow the list order, which would make this a
  matrix fact.

### TARG-13

- In the draft the conversions allowed in a converted constant are: user
  conversions, the lvalue, array, and function conversions,
  qualification conversions, integral promotions, integral conversions
  that do not narrow, null pointer and null member pointer conversions
  from the null pointer type, and function pointer conversions.
- No floating-point promotion and no floating-point conversion is on
  that list. So a float argument for a double parameter, or a double
  argument for a float parameter, is not a valid constant template
  argument in the draft, even with an exact value.
- TARG-13 forbids conversions between integer and floating types, but
  says nothing on conversions between two floating types. I am not
  sure whether a later resolution added exact floating conversions to
  the list.

### INST-P3

- The C++23 draft has no rule that an explicit instantiation stand in
  the namespace of its template when the name is unqualified. Older
  drafts had such a rule.
- In the draft, a declarator that names a function template
  specialization considers only lookup results that are nominable in
  the scope where the declarator appears. A function template found
  only through a using-directive is not nominable there, so the
  unqualified function form fails. That agrees with option 1 for the
  function.
- For a class, the explicit instantiation's elaborated type specifier
  is looked up as usual, and no rule restricts it to the template's
  namespace. For a variable template, the restriction on nominable
  results covers function template specializations only. So the draft
  accepts the class and variable forms, as option 2 says.
- INST-P3's last sentence, that the same placement holds for all three
  kinds, goes beyond the reference.

### INST-33

- The reference says that instantiating a class instantiates the
  declarations, not the definitions, of its static data members, and
  that the initialization of a static data member does not happen
  until a use needs its definition.
- A static data member with the constant-expression specifier and an
  in-class initializer is a definition. Read literally, its
  initializer waits for a use.
- The compilers instantiate such an initializer with the class, as
  INST-33 says. The reference does not state that behavior. The rule
  has the compilers' support and not clear support from the text.

### Q-0221

- The reference does access and ambiguity checks on the deallocation
  function of an allocation of class type, with no condition on
  whether the initialization can throw. All three compilers agree on
  the access case.
- The deleted-function rule forbids any reference to a deleted function,
  implicit or explicit. The draft does not say outright that the
  selection of the deallocation function by the expression is such a
  reference, and its use rules do not name the expression's
  deallocation function.
- So the text leans to option 1 through the access check, without a
  direct statement.

### Q-0274

- In the reference, a name that finds a using-declarator whose final
  name is dependent refers to a member of the class being defined
  whose meaning waits for the arguments, so the call is dependent. The rule on hypothetical
  instantiations covers only constructs that do not depend on a
  parameter, so it does not reach this call.
- A note in the reference says that a using-declarator is never
  dependent in a specialization and is replaced during the lookup for
  that specialization. With an empty pack the replacement is nothing,
  and the lookup then goes on to the namespace and finds the
  namespace function. That reading gives option 2, as GCC and EDG do.
- The reference also turns a name of a non-static member used in a
  member function into a member access through the object. It does not
  say how that step interacts with a name whose meaning changes in the
  specialization. The answer is unclear, and the note leans to
  option 2. PACK-P2's model gives option 1.

### Q-0290

- The reference introduces the parameters of a requires expression
  with a function parameter clause and refers to the function rules.
  The rule that a single unnamed parameter of non-dependent void type
  means an empty list belongs to that clause. So option 1 fits.
- The reference does not repeat that rule for requires expressions.
  This is the small doubt.

### Q-0383

- The reference reserves literal suffix identifiers that do not start
  with an underscore. A declaration of a literal operator with such a
  suffix makes the program ill-formed, no diagnostic required.
- So the program in both tests is not valid, and an implementation may
  either call the operator or reject the literal. Neither option fits
  the reference. Option 1 calls the program valid, and option 2 calls
  the rejection required.
- KW-4's class, no-diagnostic, matches the reference. OPR-9's class,
  recommended, with the program valid, does not.
- The form of the operator with a space between the quotes and the
  suffix is deprecated in the draft. For the second test, the
  reference reads the token as a user-defined literal with the
  two-character suffix, because that suffix is not an integer suffix.
  EDG's reading of a built-in suffix followed by extra text does not
  match. Under the no-diagnostic rule this does not change the class.

### Q-0386

- The reference says that the three comparison category classes are
  not predefined. A library declaration of such a class must precede
  each use of that class, including an implicit use by the built-in
  operator or by a defaulted operator with a deduced type. Otherwise
  the program is ill-formed.
- The condition is stated per class: the class that the use needs.
  The reference does not ask for the classes that the use does not
  yield. So each use needs only the class it yields, as Clang and GCC
  do. That is option 3.
- For a defaulted operator over integer members, the deduced type is
  the strong class. The rule for the common comparison type chooses
  among the classes, but the reference does not describe that choice
  as a use of the other two.
- The reference asks for a standard library declaration. A program
  that declares the classes itself is outside the reference, as the
  earlier review noted for LSUP-5.
