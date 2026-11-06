# Implicit conversion sequences

An implicit conversion sequence describes how one argument becomes one
parameter of a function. Overload resolution
({rule}`OVL-25`) compares the sequences of two
functions, argument by argument. This page covers:

- the kinds of sequence and what each holds,
- the rank of a standard sequence,
- reference parameters and object parameters,
- sequences from a braced list,
- how two sequences for the same argument compare.

Depends on: standard conversions, initialization of references,
list initialization, constructors, conversion functions.

## Principles

:::{principle} ICS-P1
:rules: ICS-1 ICS-3 ICS-4 ICS-5 ICS-6 ICS-7 ICS-8 ICS-9 ICS-10 ICS-45 ICS-47

A sequence measures how far an argument is from a parameter. A sequence
has one of four kinds:

| Kind | Holds | Rules |
|---|---|---|
| Standard | at most three standard conversions, in a fixed order | ICS-1, ICS-2 |
| User-defined | a standard sequence, one constructor or conversion function, then a second standard sequence | ICS-3, ICS-4, ICS-5, ICS-45 |
| Ellipsis | an argument that matches `...` | ICS-6 |
| Ambiguous | two or more constructors or conversion functions, none of them best | ICS-7 |

An argument that names an {term}`overload set` first selects one function of the
set, and then has the sequence of an lvalue that names that function
(ICS-47).

A sequence holds at most one user-defined conversion (ICS-9, ICS-10).
Forming a sequence looks at the argument's type, {term}`cv-qualifiers <cv-qualifier>`, and
{term}`value category`, and for some arguments at the expression
itself: the width of a bit-field (CONV-1), a null pointer constant
(CONV-8), a string literal (ICS-38), and an {term}`overload set`. Faults such as access, narrowing, a deleted function, or
a bit-field are checked only for the function that is chosen (ICS-8).
So a function can win and then fail.
:::

:::{principle} ICS-P2
:rules: ICS-1 ICS-2 ICS-39

A standard sequence takes at most one step from each group, top to
bottom. The worst step gives the rank of the whole sequence:

```text
argument
   |
   v
1. lvalue transformation      lvalue to rvalue, array to pointer,   exact match
                              function to pointer
   |
   v
2. promotion                  integer promotion, float to double,   promotion
                              an enumeration to its fixed
                              underlying type
   or conversion              other arithmetic conversions,         conversion
                              pointer, member pointer, and bool
                              conversions, derived to base
   |
   v
3. qualification adjustment   cv-qualifiers added through           exact match
                              pointers, an array bound dropped
                              through a pointer, noexcept dropped
                              from a function pointer
   |
   v
parameter
```

A sequence with no step is the identity. Steps that keep the value and
its representation have exact-match rank. A promotion widens a value to
a type that arithmetic prefers, so it ranks above a conversion that can
change the value or its meaning.
:::

:::{principle} ICS-P3
:rules: ICS-11 ICS-12 ICS-13 ICS-14 ICS-40 ICS-41

A reference that binds directly to the argument adds no step, because
no value changes. A reference that binds to a temporary takes the
sequence of the conversion that makes the temporary. The object of a
member call is an argument too.

| Parameter | Argument | Sequence | Rules |
|---|---|---|---|
| Reference that binds directly | the argument, or the result of a conversion function | the identity, or derived to base | ICS-11 |
| Reference that binds a temporary | a value of another type | the conversion to the referenced type | ICS-11 |
| Lvalue reference that is not `const` | an rvalue | none | ICS-12 |
| Rvalue reference to an object type | an lvalue of that type or a derived class | none | ICS-12 |
| Implicit object parameter | the object | a standard sequence only; an rvalue binds when there is no ref-qualifier | ICS-13, ICS-40 |
| Explicit object parameter | the object | as for any parameter | ICS-41 |
| Object slot of a static member | any object | ties with every standard sequence | ICS-14 |
:::

:::{principle} ICS-P4
:rules: ICS-15 ICS-16 ICS-17 ICS-18 ICS-19 ICS-20 ICS-38

A braced list has no type, so its sequence depends on the parameter:

| Parameter | Sequence of the list | Rule |
|---|---|---|
| Not a class or reference | `{}`: the identity. One element: that element's sequence. | ICS-15 |
| `std::initializer_list<E>` | The worst element conversion to `E`. A designated list has none. | ICS-16 |
| Reference that {rule}`REF-6` binds to the one element | The element's sequence for the reference | ICS-20 |
| Any other reference | The list's sequence to a temporary of the referenced type | ICS-20 |
| Reference to an array of characters, from one string literal | The identity, if the literal can initialize the array | ICS-38 |
| Reference to an array | The worst element conversion | ICS-17 |
| Aggregate class | User-defined, or the sequence of a single element of the class | ICS-18 |
| Class with constructors | User-defined through the chosen constructor, or ambiguous | ICS-19 |

Each element converts on its own, so one list can use several
user-defined conversions. For a reference, {principle}`REF-P3` gives
which lists {term}`bind directly`, which make a temporary, and which have no
sequence.
:::

:::{principle} ICS-P5
:rules: ICS-21 ICS-22 ICS-23 ICS-24 ICS-25 ICS-26 ICS-27 ICS-28 ICS-29 ICS-30 ICS-31 ICS-32 ICS-33 ICS-34 ICS-35 ICS-36 ICS-37 ICS-42 ICS-43 ICS-44 ICS-46

Two sequences for the same argument compare in these steps. The first
step that tells the two sequences apart decides:

```text
sequences S1 and S2 for one argument
   |
   v
1. kind                standard > user-defined = ambiguous      ICS-21
                       > ellipsis
   |
   v
2. list targets        S1 initializes std::initializer_list    ICS-22
                       and S2 does not: S1 is better
                       arrays of one element type: fewer        ICS-23
                       elements, then a known bound, is better  ICS-24
   |
   v
3. both user-defined   same function or aggregate class:        ICS-25
                       compare the third parts by step 4
                       otherwise: tie
   |
   v
4. both standard       a. proper subsequence                    ICS-26
                       b. rank                                  ICS-27
                       c. the one same-rank tie-breaker that    ICS-32 to
                          applies                               ICS-36
                       d. rvalue reference to an rvalue         ICS-28
                       e. lvalue reference to a function        ICS-29
                       f. fewer cv-qualifiers added             ICS-31
                       g. fewer cv-qualifiers on the            ICS-30
                          referenced type
                       h. two direct bindings to one object:    ICS-46
                          the referenced type that converts
                          to the other by a qualification
                          adjustment is better
   |
   v
5. nothing decides     the sequences tie
```

The steps prefer the sequence that changes the argument least: no user
code over user code, a nearer type over a farther one, and a move from
an rvalue over a copy. Steps 4b and 4c look at the conversions, and
steps 4d to 4g look at how a reference binds and at cv-qualifiers.

Some steps never decide the same case, so their order is not
observable:

- Step 4a never disagrees with step 4b, because a subsequence never has
  a worse rank.
- ICS-23 and ICS-24 compare arrays of one element type, whose sequences
  have the same kind.
- The tie-breakers of step 4c look at different conversions, so at most
  one of them applies.
- Step 4e compares two bindings of one function, which no other step
  tells apart.
- Step 4f needs results that differ below the top level, and step 4g
  needs referenced types that differ only at the top level.
- Step 4h compares two direct bindings, which have no qualification
  adjustment for step 4f. Their referenced types differ below the top
  level or in a bound, which step 4g does not compare. No test observes
  the order of step 4h against step 4d.

Tests observe step 4b before 4d, and 4d before 4g. Step 1 before
step 2 ({question}`Q-0133`), 4c before 4d, and 4d before 4f
({question}`Q-0152`) are provisional. When a user-defined conversion
chooses between two conversion functions (ICS-37), the third parts
compare by step 4, and step 4c adds ICS-42 to ICS-44 for results of
different types.
:::

## Constraints

### Forming a sequence

:::{rule} ICS-1
:kind: required
:tests: overloading/conversion-sequences/standard-top-level-cv.cpp overloading/conversion-sequences/standard-three-steps.cpp

A {term}`standard conversion sequence` is the identity, or one to three
standard conversions. The sequence has at most one conversion from each
group of ICS-P2, in the order of the groups. Different top-level
cv-qualifiers on the argument and a non-reference parameter add no
conversion.
:::

:::{rule} ICS-2
:kind: required
:tests: overloading/conversion-sequences/rank-worst-step.cpp overloading/conversion-sequences/rank-class-value.cpp overloading/conversion-sequences/bit-field-promotion.cpp

Each conversion has a {term}`sequence rank`:

- exact match: the identity, an lvalue transformation, a qualification
  adjustment,
- promotion: an {term}`integer promotion` ({rule}`CONV-1`), the promotions of
  ICS-39, and `float` to `double`,
- conversion: every other conversion of ICS-P2.

The worst conversion gives the rank of the sequence. A class argument
for a parameter of the same class has exact-match rank. For a parameter
of a base class, the rank is conversion.
:::

:::{rule} ICS-39
:kind: required
:tests: overloading/conversion-sequences/fixed-enumeration-promotion.cpp overloading/conversion-sequences/fixed-enumeration-promotion-unsigned-char.cpp

An unscoped enumeration with a fixed underlying type `U` has a
promotion to `U` and to the type that `U` promotes to ({rule}`CONV-1`).
:::

:::{rule} ICS-3
:kind: required
:tests: overloading/conversion-sequences/user-defined-first-part.cpp overloading/conversion-sequences/user-defined-second-part.cpp overloading/overload-resolution/const-ref-parameter-through-converting-constructor.cpp

A {term}`user-defined conversion sequence` has three parts:

1. a standard sequence to the first parameter of a constructor, or to
   the object parameter of a conversion function,
2. that constructor or conversion function,
3. a standard sequence from the result to the parameter.

For a reference parameter, the binding of the reference belongs to the
third part.
:::

:::{rule} ICS-4
:kind: required
:tests: overloading/conversion-sequences/user-defined-candidates.cpp overloading/conversion-sequences/reference-conversion-value-result.cpp

These functions can be the second part of ICS-3:

- for a class parameter, the {term}`converting constructors <converting
  constructor>` of the class, and the conversion functions to the class
  or a class derived from it,
- for another parameter, the conversion functions whose result reaches
  the parameter by a standard sequence,
- for a reference that binds directly to the result, the conversion
  functions that {rule}`OVL-15` gives for the kind of reference.

ICS-45 adds `explicit` conversion functions in direct initialization
and for the operands of the built-in `!`, `&&`, and `||` ({rule}`OVL-9`).
{rule}`CPY-7` and {rule}`OVL-17` remove some constructors.
:::

:::{rule} ICS-45
:kind: required
:tests: overloading/conversion-sequences/user-defined-explicit.cpp overloading/conversion-sequences/user-defined-explicit-reference-value.cpp overloading/conversion-sequences/user-defined-explicit-rvalue-reference-value.cpp overloading/conversion-sequences/user-defined-explicit-reference-xvalue.cpp overloading/conversion-sequences/user-defined-candidates.cpp overloading/conversion-sequences/user-defined-explicit-reference-other-type.cpp

Direct initialization can also use an `explicit` conversion function
({rule}`CVF-3`) for these targets:

- a class `C`, when the result type is `C` or a reference to `C`, apart
  from cv-qualifiers. The result goes to the parameter of the
  constructor that copies or moves `C`.
- a target of another type, other than a reference, when the result
  reaches it by a qualification adjustment alone,
- a reference that binds directly to the result, when the result
  reaches the referenced type by a qualification adjustment alone.
  {rule}`OVL-15` gives the results that each kind of reference takes.
  So an rvalue reference takes a value ({question}`Q-0156`), and an
  lvalue reference takes only an lvalue
  ({question}`Q-0337`).

A function parameter is copy-initialized, so these functions never
serve it. The temporary that a reference binds is copy-initialized
too, also in direct initialization, so an `explicit` conversion
function cannot make it. These are among the direct-initialized
operands: the operand of
a cast, the variable of a direct initialization, the parameter of a
copy or move constructor in the direct initialization of a class
({rule}`OVL-40`), the operands of the built-in `!`, `&&`, and `||`
({rule}`OVL-9`), a member initializer, a parenthesized `new`
initializer, and a condition that converts to `bool`.
:::

:::{rule} ICS-5
:kind: required
:tests: overloading/conversion-sequences/template-conversion-exact.cpp overloading/conversion-sequences/template-conversion-exact-long.cpp

A conversion {term}`function template` serves a user-defined conversion only if
the third part of ICS-3 has exact-match rank.
:::

:::{rule} ICS-6
:kind: required
:tests: overloading/conversion-sequences/ellipsis-sequence.cpp overloading/conversion-sequences/ellipsis-braced-list.cpp overloading/conversion-sequences/ellipsis-braced-list-other-wins.cpp overloading/conversion-sequences/overload-set-to-ellipsis.cpp

An argument that matches the `...` of a function has an
{term}`ellipsis conversion sequence`. A braced list has no ellipsis
conversion sequence, so a function that needs one for a braced list is
not viable.
An argument that names an overload set has an ellipsis conversion
sequence only if {rule}`OVL-20` leaves one function (ICS-47).
:::

:::{rule} ICS-47
:kind: required
:tests: overloading/conversion-sequences/overload-set-argument-sequence.cpp overloading/conversion-sequences/overload-set-reference-or-pointer.cpp overloading/conversion-sequences/overload-set-to-ellipsis.cpp

An argument that names an overload set selects one function of the set
by the type of the parameter:

- If the parameter's type is a pointer or a reference to a function, or
  a pointer to a member function, that type is the target type of
  {rule}`OVL-18`, and OVL-18 and {rule}`OVL-19` select the function.
- Any other parameter, and `...`, give no target type, and
  {rule}`OVL-20` selects the function.

The sequence is the sequence of an lvalue that names the selected
function. If no single function remains, the candidate is not viable.
:::

:::{rule} ICS-7
:kind: required
:tests: overloading/conversion-sequences/ambiguous-loses-to-standard.cpp overloading/conversion-sequences/ambiguous-ties-user-defined.cpp overloading/conversion-sequences/chosen-ambiguous.cpp overloading/conversion-sequences/chosen-ambiguous-tie.cpp overloading/conversion-sequences/user-defined-choice-object-parameter.cpp overloading/conversion-sequences/chosen-ambiguous-constructor-conversion.cpp

If two or more constructors or conversion functions can convert an
argument, overload resolution chooses one ({rule}`OVL-25`):

- The argument's sequences to their first parameters or object
  parameters compare first.
- If those tie, ICS-37 and {rule}`OVL-27` to {rule}`OVL-30` decide.
- If no function is best, the sequence is the
  {term}`ambiguous conversion sequence`. It counts as user-defined, and
  the function that takes the argument stays a candidate.
:::

:::{rule} ICS-8
:kind: reject
:tests: overloading/conversion-sequences/chosen-ambiguous.cpp overloading/conversion-sequences/chosen-ambiguous-tie.cpp overloading/conversion-sequences/chosen-ambiguous-constructor-conversion.cpp overloading/conversion-sequences/chosen-list-ambiguous.cpp overloading/conversion-sequences/chosen-deleted-constructor.cpp overloading/conversion-sequences/chosen-deleted-conversion-function.cpp overloading/conversion-sequences/chosen-bit-field.cpp overloading/conversion-sequences/chosen-private-base.cpp overloading/conversion-sequences/chosen-ambiguous-base.cpp overloading/conversion-sequences/chosen-narrowing.cpp overloading/conversion-sequences/chosen-designator-order.cpp overloading/conversion-sequences/chosen-explicit-copy-list.cpp

A sequence forms in spite of these faults:

- the ambiguous sequence of two or more constructors or conversion
  functions (ICS-7, {rule}`CVF-12`),
- the ambiguous sequence of a braced list that no constructor
  initializes unambiguously (ICS-19, {rule}`CTOR-25`),
- a deleted constructor or conversion function ({rule}`DEL-1`),
- a bit-field bound to an lvalue reference that is not `const`
  ({rule}`REF-8`),
- a base that is ambiguous or not accessible ({rule}`CONV-17`),
- a narrowing conversion in a braced list ({rule}`LIST-2`),
- designators out of declaration order ({rule}`LIST-14`),
- an `explicit` constructor in copy-list-initialization
  ({rule}`ILC-3`, {rule}`ILC-8`, {question}`Q-0132`).

If the chosen function has such a sequence, the implementation rejects
the program. The rule named with each fault is the specific rule.
:::

:::{rule} ICS-9
:kind: required
:tests: overloading/conversion-sequences/two-user-defined-none.cpp overloading/conversion-sequences/two-user-defined-function-then-constructor.cpp

A sequence holds at most one user-defined conversion. A conversion that
needs two has no sequence, and {rule}`CTOR-22` and {rule}`CVF-10` reject
it. The element conversions of ICS-16 to ICS-19 are separate
conversions.
:::

:::{rule} ICS-10
:kind: required
:tests: overloading/conversion-sequences/user-defined-step-standard-only.cpp overloading/conversion-sequences/implicit-object-standard-only.cpp overloading/conversion-sequences/assignment-left-standard-only.cpp overloading/conversion-sequences/double-brace-standard-only.cpp overloading/conversion-sequences/object-explicit-assignment.cpp overloading/overload-resolution/init-copy-constructor-standard-only.cpp overloading/conversion-sequences/assignment-left-no-builtin.cpp overloading/overload-resolution/builtin-increment-from-conversion.cpp

Only a standard sequence can reach these parameters:

- the implicit object parameter of a member function,
- the left operand of a built-in `=`, but not of a compound assignment
  or of `++` and `--` ({principle}`OVL-P4`),
- the first parameter of a constructor, and the object parameter of a
  conversion function, that ICS-4 considers,
- a parameter of type `C` or reference to `C` of a constructor of class
  `C`, for an inner braced list, when a list with only that inner list
  initializes `C`.
:::

### References and object parameters

:::{rule} ICS-11
:kind: required
:tests: overloading/conversion-sequences/reference-direct-derived.cpp overloading/conversion-sequences/reference-temporary.cpp overloading/conversion-sequences/reference-conversion-result.cpp overloading/conversion-sequences/reference-conversion-value-result.cpp expressions/conditional/class-converts-to-int-reference-lvalue.cpp declarations/initialization/references/pointer-lvalue-binds-more-qualified-pointer.cpp

The sequence of a reference parameter depends on how the reference binds
({rule}`REF-1`, {rule}`REF-2`, {rule}`REF-3`):

- directly to the argument: the identity, or a derived-to-base
  conversion of conversion rank if the argument's class derives from
  the referenced class,
- directly to the result of a conversion function that ICS-4 gives: a
  user-defined sequence whose third part is one of those two. The
  result can be a class prvalue.
- to a temporary: the sequence that converts the argument to the
  referenced type, without its top-level cv-qualifiers.

A prvalue that is not of class type always binds a temporary
({rule}`REF-2`). So an `int*` prvalue for a `const int* const&`
parameter has the qualification adjustment to `const int*` as its
sequence, and an `int*` lvalue binds the same parameter directly with
the identity (REF-1).
:::

:::{rule} ICS-12
:kind: required
:tests: overloading/conversion-sequences/reference-no-sequence.cpp overloading/conversion-sequences/reference-function-rvalue.cpp overloading/conversion-sequences/reference-rvalue-other-type.cpp

No sequence binds these references ({rule}`REF-9`, {rule}`REF-10`):

- an lvalue reference that is not `const`, or is `volatile`, to an
  rvalue,
- an rvalue reference to an object type `T`, to an lvalue of type `T`
  or of a class derived from `T`, also when a conversion function
  returns the lvalue.

For an lvalue of another type, an rvalue reference binds a temporary.
An rvalue reference to a function type binds to a function lvalue.
ICS-40 gives the exception for the implicit object parameter.
:::

:::{rule} ICS-13
:kind: required
:tests: overloading/conversion-sequences/object-ref-qualifier.cpp overloading/conversion-sequences/object-conversion-function-class.cpp overloading/conversion-sequences/object-using-declaration.cpp overloading/conversion-sequences/object-rvalue-non-const.cpp overloading/conversion-sequences/object-explicit-user-defined.cpp

A non-static member function has an {term}`implicit object parameter`:

- Its type is an lvalue reference to the class, with the function's
  cv-qualifiers. With the ref-qualifier `&&`, it is an rvalue reference.
- For a conversion function, the class is the class of the object. So
  the object parameters of a conversion function of a base and one of a
  derived class tie, and ICS-37 decides between the two.
- For a function that a using-declaration names, the class is the class
  with the using-declaration.
:::

:::{rule} ICS-40
:kind: required
:tests: overloading/conversion-sequences/object-rvalue-non-const.cpp overloading/conversion-sequences/rvalue-reference-object-exception.cpp

If a member function has no ref-qualifier, its implicit object
parameter also binds to an rvalue, in spite of ICS-12. Two such
bindings compare as other lvalue reference bindings do, so ICS-30
applies. ICS-28 does not apply to such a binding.
:::

:::{rule} ICS-41
:kind: required
:tests: overloading/conversion-sequences/object-explicit-user-defined.cpp overloading/conversion-sequences/object-explicit-assignment.cpp

An explicit object parameter is an ordinary parameter. A user-defined
sequence can reach it, also when the function is an assignment
operator.
:::

:::{rule} ICS-14
:kind: required
:tests: overloading/conversion-sequences/static-object-slot.cpp overloading/conversion-sequences/static-object-slot-derived.cpp

A static member function that is a candidate in a member call has an
object slot. Any object matches the slot. The slot's sequence ties with
every standard sequence.
:::

### Sequences from a braced list

:::{rule} ICS-15
:kind: required
:tests: overloading/conversion-sequences/list-scalar.cpp overloading/conversion-sequences/list-scalar-pointer-and-pair.cpp

For a parameter that is not a class or a reference, a braced list has
this sequence:

- for an empty list, the identity,
- for one element that is not a braced list, the sequence of the
  element,
- for any other list, no sequence.
:::

:::{rule} ICS-16
:kind: required
:tests: overloading/conversion-sequences/list-initializer-list.cpp overloading/conversion-sequences/list-initializer-list-designated.cpp foundations/language-support/initializer-list-candidate-not-viable.cpp overloading/conversion-sequences/list-user-defined-same-function.cpp overloading/conversion-sequences/list-user-defined-first-element-int.cpp overloading/conversion-sequences/list-user-defined-first-element-long.cpp

For a parameter of type `std::initializer_list<E>` ({rule}`LSUP-1`), a
braced list has a sequence if each element converts to `E`. An element
can use a user-defined conversion. The sequence is the worst element
conversion, or the identity for an empty list. A designated list has no
sequence.

When the worst kind is a user-defined sequence, the list sequence is
the user-defined sequence of the first element that has one. ICS-25
then compares two list sequences by that element's conversion function
and third part ({question}`Q-0419`).
:::

:::{rule} ICS-17
:kind: required
:tests: overloading/conversion-sequences/list-array.cpp overloading/conversion-sequences/list-array-against-scalar.cpp overloading/conversion-sequences/array-list-user-defined-first-element.cpp

For a reference to an array of `E` that binds a temporary (ICS-20,
{principle}`REF-P3`), a braced list has a sequence if each element converts to `E` and these
conditions hold:

- A known bound is not less than the number of elements.
- Each element past the list converts from empty braces.

The sequence is the worst element conversion, chosen as ICS-16 says
when the worst kind is a user-defined sequence. ICS-38 gives one string
literal for an array of characters.
:::

:::{rule} ICS-38
:kind: required
:tests: overloading/conversion-sequences/list-string-literal.cpp overloading/conversion-sequences/list-string-literal-utf8.cpp overloading/conversion-sequences/list-string-literal-too-short.cpp overloading/conversion-sequences/list-string-literal-bound.cpp overloading/conversion-sequences/list-array.cpp overloading/conversion-sequences/list-string-literal-binds-literal.cpp

For a reference to an array of a character type, one string literal in
braces has the identity if the literal can initialize the array
({rule}`LIST-6`, {rule}`LIST-15`), and no sequence otherwise
({principle}`REF-P3`, {question}`Q-0155`). If the array has the literal's type, or is an
array of unknown bound of its element type, the reference binds directly
to the literal ({rule}`REF-6`).
:::

:::{rule} ICS-18
:kind: required
:tests: overloading/conversion-sequences/list-aggregate.cpp overloading/conversion-sequences/list-designated.cpp overloading/conversion-sequences/list-aggregate-nearest-base.cpp

For a parameter of an aggregate class, a braced list that can initialize
the aggregate ({rule}`LIST-3`) has a user-defined sequence. A designated
list needs the named members, in any order ({rule}`LIST-5`). A list with
one element of the class or a derived class has the element's sequence.
:::

:::{rule} ICS-19
:kind: required
:tests: overloading/conversion-sequences/list-class-constructor.cpp overloading/conversion-sequences/list-class-tie.cpp overloading/conversion-sequences/list-class-same-class.cpp overloading/conversion-sequences/list-class-list-constructor.cpp overloading/conversion-sequences/list-class-nearest-base.cpp

For a parameter of a class with constructors, a braced list chooses a
constructor as {rule}`ILC-2` and {rule}`ILC-3` give:

- If one constructor is best, the sequence is user-defined.
- If no viable constructor is best, the sequence is the ambiguous
  sequence.
- If the list has one element of the class or a derived class, and the
  best constructor takes no `std::initializer_list`, the sequence is
  the element's sequence as an argument ({question}`Q-0154`).

Each element can use its own user-defined conversion.
:::

:::{rule} ICS-20
:kind: required
:tests: overloading/conversion-sequences/list-reference.cpp overloading/conversion-sequences/list-reference-direct.cpp overloading/conversion-sequences/list-reference-temporary.cpp overloading/conversion-sequences/list-reference-class-element-scalar.cpp overloading/conversion-sequences/list-reference-class-element-class.cpp

For a reference parameter, a braced list binds the reference as
{rule}`REF-6` and {rule}`REF-14` give, by the table of
{principle}`REF-P3` ({question}`Q-0294`, {question}`Q-0157`):

- If the reference binds to the list's one element, the sequence is the
  element's sequence for the reference (ICS-11, ICS-12).
- If the list initializes a temporary, the sequence is the list's
  sequence to the referenced type, and ICS-12 applies to the binding.

One element that is an lvalue of the referenced array type also binds
the reference directly.
:::

### Comparing two sequences

:::{rule} ICS-21
:kind: required
:tests: overloading/conversion-sequences/kind-order.cpp overloading/conversion-sequences/initializer-list-over-kind.cpp

A standard sequence is better than a user-defined sequence or an
ellipsis sequence. A user-defined sequence is better than an ellipsis
sequence. The rule comes before ICS-22 to ICS-24 ({question}`Q-0133`).
:::

:::{rule} ICS-22
:kind: required
:tests: overloading/conversion-sequences/initializer-list-target.cpp overloading/conversion-sequences/initializer-list-over-rank.cpp overloading/conversion-sequences/initializer-list-over-kind.cpp overloading/conversion-sequences/initializer-list-reference.cpp

A list sequence to `std::initializer_list<E>`, or to a reference to it,
is better than a list sequence to any other type. The rule compares only
two sequences of the same kind (ICS-21, {question}`Q-0133`). The rule
comes before rank.
:::

:::{rule} ICS-23
:kind: required
:tests: overloading/conversion-sequences/array-fewer-elements.cpp overloading/conversion-sequences/array-fewer-elements-const-lvalue.cpp

If two list sequences go to references to arrays of the same element
type, the sequence to the array with fewer elements is better.
:::

:::{rule} ICS-24
:kind: required
:tests: overloading/conversion-sequences/array-known-bound.cpp overloading/conversion-sequences/array-known-bound-const-lvalue.cpp

If the two arrays of ICS-23 have the same number of elements, the array
of known bound is better than the array of unknown bound.
:::

:::{rule} ICS-46
:kind: required
:tests: overloading/conversion-sequences/array-known-bound-direct.cpp overloading/conversion-sequences/list-string-literal-bound.cpp overloading/conversion-sequences/reference-direct-fewer-qualifiers.cpp overloading/conversion-sequences/array-known-bound-direct-cv.cpp

Two references can bind directly to one object. If the referenced type
of one converts to the other's by a qualification adjustment, that
reference is better. These cases occur:

- fewer cv-qualifiers below the top level, as `int* const&` over
  `const int* const&`,
- an array of known bound over an array of unknown bound
  ({question}`Q-0188`).

The binding can come from a braced list of one element ({rule}`ICS-20`,
ICS-38, {principle}`REF-P3`).
:::

:::{rule} ICS-25
:kind: required
:tests: overloading/conversion-sequences/user-defined-same-function.cpp overloading/conversion-sequences/user-defined-same-aggregate.cpp overloading/conversion-sequences/user-defined-different-functions.cpp overloading/conversion-sequences/list-user-defined-same-function.cpp overloading/conversion-sequences/list-user-defined-first-element-int.cpp overloading/conversion-sequences/list-user-defined-first-element-long.cpp overloading/conversion-sequences/array-list-user-defined-first-element.cpp

Two user-defined sequences compare by their third parts, as standard
sequences, in these cases:

- Both use the same constructor or conversion function.
- Both initialize the same aggregate class.

Other user-defined sequences tie. The ambiguous sequence ties with every
user-defined sequence.
:::

:::{rule} ICS-26
:kind: required
:tests: overloading/conversion-sequences/subsequence-function-pointer.cpp overloading/conversion-sequences/subsequence-qualification.cpp overloading/conversion-sequences/derived-pointer-then-qualification.cpp

Two standard sequences are compared without their lvalue
transformations. If one is a proper subsequence of the other, the
shorter one is better. The identity is a subsequence of every other
sequence.

A pointer conversion followed by a qualification adjustment is two
steps. `Derived*` to `const Middle*` is the derived-to-base conversion
to `Middle*` and then the adjustment to `const Middle*`. So `Derived*`
to `Middle*` is a proper subsequence of it, and for a `Derived*`
argument, `f(Middle*)` is better than `f(const Middle*)`.
:::

:::{rule} ICS-27
:kind: required
:tests: overloading/conversion-sequences/rank-comparison.cpp overloading/conversion-sequences/rank-before-reference.cpp

Of two standard sequences, the one of better rank is better. Exact
match is better than promotion, and promotion is better than
conversion. ICS-28 to ICS-36 apply only to sequences of equal rank.
:::

:::{rule} ICS-32
:kind: required
:tests: overloading/conversion-sequences/pointer-to-bool.cpp overloading/conversion-sequences/tie-breaker-before-rvalue-reference.cpp

A sequence that converts a pointer or a member pointer to `bool` is
worse than a sequence of equal rank that does not. ICS-26 and ICS-27
come first.
:::

:::{rule} ICS-33
:kind: required
:tests: overloading/conversion-sequences/base-pointer-over-void.cpp overloading/conversion-sequences/array-pointer-tie-breaker.cpp overloading/conversion-sequences/tie-breaker-before-rvalue-reference.cpp

A conversion of a pointer to a class into a pointer to its base is
better than a conversion of the same pointer into a pointer to `void`.
ICS-26 and ICS-27 come first.
:::

:::{rule} ICS-34
:kind: required
:tests: overloading/conversion-sequences/fixed-enumeration.cpp overloading/conversion-sequences/fixed-enumeration-before-rvalue-reference.cpp

If two sequences promote an unscoped enumeration with a fixed
underlying type to different types (ICS-39), the promotion to the
underlying type is better. ICS-26 and ICS-27 come first.
:::

:::{rule} ICS-35
:kind: required
:tests: overloading/conversion-sequences/floating-equal-rank.cpp overloading/conversion-sequences/floating-before-rvalue-reference.cpp

A conversion from a floating-point type to a floating-point type of
equal rank is better than a conversion to an integer type or to a
floating-point type of other rank. Only the extended floating-point
types *(extended floating-point types referenced)* have equal ranks.
ICS-26 and ICS-27 come first.
:::

:::{rule} ICS-36
:kind: required
:tests: overloading/conversion-sequences/nearest-base.cpp overloading/conversion-sequences/array-pointer-tie-breaker.cpp overloading/conversion-sequences/tie-breaker-before-rvalue-reference.cpp overloading/conversion-sequences/list-class-nearest-base.cpp overloading/conversion-sequences/list-aggregate-nearest-base.cpp

Let `B` be a base of `M`, and `M` a base of `D`. These conversions are
better than the alternative in parentheses:

- a pointer, reference, or class value of `D` to `M` (to `B`),
- a member pointer of `B` to a member pointer of `M` (of `D`).

ICS-26 and ICS-27 come first.
:::

:::{rule} ICS-28
:kind: required
:tests: overloading/conversion-sequences/rvalue-reference-binding.cpp overloading/conversion-sequences/rvalue-reference-before-cv.cpp overloading/conversion-sequences/rvalue-reference-object-exception.cpp overloading/conversion-sequences/rank-before-reference.cpp overloading/conversion-sequences/tie-breaker-before-rvalue-reference.cpp overloading/conversion-sequences/fixed-enumeration-before-rvalue-reference.cpp overloading/conversion-sequences/floating-before-rvalue-reference.cpp overloading/conversion-sequences/rvalue-reference-before-qualification.cpp

If both sequences bind references to an rvalue, binding an rvalue
reference is better than binding an lvalue reference. A reference that
binds a temporary binds an rvalue. The rule does not
apply if either sequence binds the implicit object parameter of a
function without a ref-qualifier (ICS-40). ICS-32 to ICS-36 come first
({question}`Q-0152`).
:::

:::{rule} ICS-29
:kind: required
:tests: overloading/conversion-sequences/function-lvalue-reference.cpp overloading/conversion-sequences/function-lvalue-reference-cast.cpp

If both sequences bind references to a function lvalue, binding an
lvalue reference is better than binding an rvalue reference.
:::

:::{rule} ICS-31
:kind: required
:tests: overloading/conversion-sequences/qualification-fewer.cpp overloading/conversion-sequences/subsequence-qualification.cpp overloading/conversion-sequences/rvalue-reference-before-qualification.cpp overloading/conversion-sequences/qualification-array-bound.cpp overloading/conversion-sequences/qualification-reference-temporary.cpp

Two sequences can differ only in a qualification adjustment. If the
result of one converts to the other's result by a qualification
adjustment, that sequence is better. The adjustment can add
cv-qualifiers or drop an array bound ({rule}`CONV-16`,
{question}`Q-0153`). ICS-28 comes first ({question}`Q-0152`).
:::

:::{rule} ICS-30
:kind: required
:tests: overloading/conversion-sequences/reference-less-cv.cpp overloading/conversion-sequences/rvalue-reference-before-cv.cpp

If both sequences bind references to the same type apart from top-level
cv-qualifiers, the reference with fewer cv-qualifiers is better. Its
cv-qualifiers are a subset of the other's. ICS-28 comes first.
:::

### Choosing a conversion function

:::{rule} ICS-37
:kind: required
:tests: overloading/conversion-sequences/conversion-choice-second-part.cpp overloading/conversion-sequences/conversion-choice-promotion-result.cpp

When a user-defined conversion chooses between two conversion functions
whose object parameters tie (ICS-7), the function whose result has the
better third part of ICS-3 wins ({rule}`OVL-26`). The third parts
compare by step 4 of ICS-P5, with ICS-42 to ICS-44 for results of
different types. The object parameters of an inherited conversion
function and one of the derived class tie (ICS-13). For a reference to a function, {rule}`OVL-29` decides
between results that still tie ({question}`Q-0138`).
:::

:::{rule} ICS-42
:kind: required
:tests: overloading/conversion-sequences/conversion-choice-nearest-source.cpp overloading/conversion-sequences/conversion-choice-class-value.cpp

Let `B`, `M`, and `D` be as in ICS-36. When two results of different
types convert to one target, these conversions are better than the
alternative in parentheses:

- to a pointer, reference, or class value of `B`, one from `M` (from
  `D`),
- to a member pointer of `D`, one from a member pointer of `M` (of
  `B`).
:::

:::{rule} ICS-43
:kind: required
:tests: overloading/conversion-sequences/conversion-choice-void.cpp overloading/conversion-sequences/conversion-choice-void-const.cpp

When two results convert to `void*`, a conversion from a pointer to a
class is better than a conversion from a pointer to a class derived
from it.
:::

:::{rule} ICS-44
:kind: required
:tests: overloading/conversion-sequences/conversion-choice-equal-rank-source.cpp overloading/conversion-sequences/conversion-choice-equal-rank-extended-target.cpp

When two results convert to a floating-point type `A`, a conversion from
a floating-point type of the same rank as `A` is better. The other
result is an integer type or a floating-point type of other rank
*(extended floating-point types referenced)*.
:::

## Examples

% tidal-example: reject
```cpp
struct Base {};
struct Derived : Base {};
struct Text { Text(const char*); };

void show(long);            // 1
void show(Text);            // 2
void show(...);             // 3
show(5);                    // 1: standard beats user-defined
show("hi");                 // 2: user-defined beats the ellipsis

void put(int&);             // 4
void put(const int&);       // 5
int n = 0;
put(n);                     // 4: fewer cv-qualifiers
put(7);                     // 5: no sequence binds int& to an rvalue
put({n});                   // 4: the list binds to its element

void take(Base*);           // 6
void take(void*);           // 7
Derived d;
take(&d);                   // 6: a base pointer beats void*

void sum(std::initializer_list<long>);  // 8
void sum(int);              // 9
sum({1});                   // 8: the initializer-list target comes first

enum Level : short { low };
void pick(const short&);    // 10
void pick(int&&);           // 11
pick(low);                  // 10: ICS-34 comes before ICS-28

struct Pair { Pair(int, int); };
struct Point { int x, y; };
struct Holder { Point p; };
void hold(Pair);            // 12
void hold(Holder);          // 13
hold({{1, 2}});             // 13: Pair's copy constructor gets no
                            // user-defined sequence from {1, 2} (ICS-10)

struct Target;
struct Source { operator Target(); };
struct Target { Target(); Target(Source&); };
Source s;
void send(Target);          // 14
void send(...);             // 15
send(s);                    // rejected: Target(Source&) and operator Target()
                            // tie (ICS-7), and 14 still beats 15 (ICS-8)
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 110
of 141 tests.

### Divergences

- {question}`Q-0081` (ICS-19; settled on ILC-P2): whether a one-element
  list of a class with an initializer-list constructor uses that
  constructor. It does. Clang 18 differs.
- {question}`Q-0132` (ICS-8; open): whether a braced list has a
  sequence to a class parameter when the chosen constructor is
  `explicit`. It does, so the call chooses that function and is
  rejected, as on GCC. Clang and EDG choose the other function.
- {question}`Q-0133` (ICS-21, ICS-22; open): whether a user-defined list
  sequence to `std::initializer_list` beats a standard sequence. It does
  not, because the kinds are compared first, as on Clang. GCC and EDG
  choose the initializer list.
- {question}`Q-0134` (ICS-2; open): whether a `long long` bit-field of
  width 3 has a promotion to `int`. It does, as on Clang. GCC and EDG
  give it a conversion, so `int` and `long` tie.
- {question}`Q-0135` (ICS-17; settled on ICS-P5): whether a braced list
  prefers a scalar parameter to a reference to an array when both
  sequences are the identity. The two tie. GCC 16 prefers the scalar.
- {question}`Q-0136` (ICS-17, ICS-18; open): whether empty braces or a
  designated list have a sequence to a reference to an array. They do
  not, as on Clang. GCC and EDG differ.
- {question}`Q-0137` (ICS-26, ICS-33, ICS-36; open): whether the
  pointer tie-breakers apply after an array-to-pointer step. They do, as
  on Clang and EDG. GCC finds a tie.
- {question}`Q-0138` (ICS-37; settled on OVL-P3): whether the kind of a
  function reference breaks a tie between conversion functions to `F&`
  and `F&&`. It does ({rule}`OVL-29`). Clang 22 finds the two ambiguous
  for `F&&`.
- {question}`Q-0152` (ICS-28, ICS-31 to ICS-36; open): whether the
  same-rank tie-breakers come before the rvalue-reference preference,
  and that preference before the qualification step. Both do, as on
  Clang. GCC applies ICS-28 before ICS-34 and ICS-31 before ICS-28. EDG
  applies ICS-28 before every same-rank tie-breaker.
- {question}`Q-0153` (ICS-31; open): whether the qualification step
  compares array bounds, and a reference that binds a temporary. It
  compares both, as Clang does. GCC and EDG tie on the bound
  (`qualification-array-bound.cpp`). EDG ties on the temporary
  (`qualification-reference-temporary.cpp`).
- {question}`Q-0154` (ICS-19, ICS-36; open): whether the nearest-base
  tie-breaker applies to a one-element list of a derived class that is
  not an aggregate. It does, as on Clang and GCC. EDG finds a tie.
- {question}`Q-0155` (ICS-38; open): when one braced string literal has
  a sequence to a reference to a character array. It has one when the
  literal can initialize the array. GCC gives a UTF-8 literal no
  sequence to a `char` array. Clang gives a literal a sequence to an
  array that is too short, and ties a known and an unknown bound.
- {question}`Q-0156` (ICS-45; open): whether direct initialization of
  an rvalue reference can use an `explicit` conversion function that
  returns a value. It can, as on Clang. GCC and EDG reject it.
- {question}`Q-0157` (ICS-20; settled on {principle}`REF-P3`): whether a
  one-element list of a class binds a reference through the element's
  conversion function. It binds through the element only when the
  element has the referenced type or a derived class, and makes a
  temporary otherwise (REF-6, REF-14). GCC 16 binds through the
  conversion function.
- {question}`Q-0167` (ICS-38; settled on ICS-P5): whether one braced
  string literal prefers a reference to a `char` array over one to an
  `unsigned char` array. The two tie. GCC 16 picks `char`.
- {question}`Q-0188` (ICS-46; open): whether the reference to the known
  bound is better when two references bind directly to one array. It
  is, as on GCC and EDG. Clang finds a tie.
- {question}`Q-0294` (ICS-15 to ICS-20, ICS-38, ICS-46; open): how a
  braced list binds a reference, initializes an array, and forms a
  sequence. The rules follow the table of {principle}`REF-P3`. The
  question lists each split with its own question.
- {question}`Q-0321` (ICS-16; open): when a program needs the class
  `std::initializer_list`. It needs the class where a braced list
  initializes such an object or `auto` deduces one ({rule}`LSUP-1`). GCC
  checks every definition.
- {question}`Q-0337` (ICS-45; open): whether direct initialization of an
  lvalue reference to `const` can use an `explicit` conversion function
  that does not return an lvalue. It cannot, as on GCC. Clang accepts
  both forms, and EDG accepts an rvalue reference result.
- {question}`Q-0419` (ICS-16, ICS-17, ICS-25; open): which element's
  conversion a list sequence holds when the worst element conversion is
  user-defined. It holds the first such element's, as Clang and EDG do.
  GCC calls every such call ambiguous.

### Split tests

The table lists each test on which the compilers differ. The
Expected column gives the outcome that the rules predict. The other
cells give the outcome of the test on each compiler.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `array-known-bound-direct-cv.cpp` | ICS-46 | accept | reject | accept | accept | {question}`Q-0188` |
| `array-known-bound-direct.cpp` | ICS-46 | accept | reject | accept | accept | {question}`Q-0188` |
| `array-list-user-defined-first-element.cpp` | ICS-17, ICS-25 | accept | accept | reject | accept | {question}`Q-0419` |
| `array-pointer-tie-breaker.cpp` | ICS-33, ICS-36 | accept | accept | reject | accept | {question}`Q-0137` |
| `bit-field-promotion.cpp` | ICS-2 | accept | accept | reject | reject | {question}`Q-0134` |
| `chosen-explicit-copy-list.cpp` | ICS-8, ILC-8 | reject | accept | reject | accept | {question}`Q-0132` |
| `conversion-choice-equal-rank-extended-target.cpp` | ICS-44 | accept | reject | accept | accept | |
| `conversion-choice-equal-rank-source.cpp` | ICS-44 | accept | reject | accept | accept | |
| `fixed-enumeration-before-rvalue-reference.cpp` | ICS-28, ICS-34 | accept | accept | reject | reject | {question}`Q-0152` |
| `floating-before-rvalue-reference.cpp` | ICS-28, ICS-35 | accept | reject | accept | reject | {question}`Q-0152` |
| `floating-equal-rank.cpp` | ICS-35 | accept | reject | accept | accept | |
| `foundations/language-support/initializer-list-candidate-not-viable.cpp` | LSUP-1, ICS-16 | accept | accept | reject | accept | {question}`Q-0321` |
| `initializer-list-over-kind.cpp` | ICS-22, ICS-21 | accept | accept | reject | reject | {question}`Q-0133` |
| `list-array-against-scalar.cpp` | ICS-17 | accept | accept | reject | accept | {question}`Q-0135` |
| `list-class-list-constructor.cpp` | ICS-19 | accept | Clang 22: accept; Clang 18: reject | accept | accept | {question}`Q-0081` |
| `list-class-nearest-base.cpp` | ICS-19, ICS-36 | accept | accept | accept | reject | {question}`Q-0154` |
| `list-reference-class-element-class.cpp` | ICS-20 | accept | accept | reject | accept | {question}`Q-0157` |
| `list-reference-class-element-scalar.cpp` | ICS-20 | accept | accept | reject | accept | {question}`Q-0157` |
| `list-string-literal-bound.cpp` | ICS-38, ICS-46 | accept | reject | accept | accept | {question}`Q-0155`, {question}`Q-0188` |
| `list-string-literal-too-short.cpp` | ICS-38 | accept | reject | accept | accept | {question}`Q-0155` |
| `list-string-literal-utf8.cpp` | ICS-38 | accept | accept | reject | accept | {question}`Q-0155` |
| `list-user-defined-first-element-int.cpp` | ICS-16, ICS-25 | accept | accept | reject | accept | {question}`Q-0419` |
| `list-user-defined-first-element-long.cpp` | ICS-16, ICS-25 | accept | accept | reject | accept | {question}`Q-0419` |
| `list-user-defined-same-function.cpp` | ICS-16, ICS-25 | accept | accept | reject | accept | {question}`Q-0419` |
| `qualification-array-bound.cpp` | ICS-31 | accept | accept | reject | reject | {question}`Q-0153` |
| `qualification-reference-temporary.cpp` | ICS-31 | accept | accept | accept | reject | {question}`Q-0153` |
| `rvalue-reference-before-qualification.cpp` | ICS-28, ICS-31 | accept | accept | reject | accept | {question}`Q-0152` |
| `tie-breaker-before-rvalue-reference.cpp` | ICS-28, ICS-32, ICS-33, ICS-36 | accept | accept | accept | reject | {question}`Q-0152` |
| `user-defined-explicit-reference-value.cpp` | ICS-45 | accept | reject | accept | accept | {question}`Q-0337` |
| `user-defined-explicit-reference-xvalue.cpp` | ICS-45 | accept | reject | accept | reject | {question}`Q-0337` |
| `user-defined-explicit-rvalue-reference-value.cpp` | ICS-45 | accept | accept | reject | reject | {question}`Q-0156` |

### Notes

- ICS-8 is a `reject` rule because each fault is rejected once its
  function is chosen. Every test has a fallback function that the call
  would use if the fault removed the sequence.
- ICS-35, ICS-44: Clang has no `f32`, `f64`, or `f128` suffix
  ({rule}`PRIM-22`). GCC and EDG follow the rules. Only the extended
  floating-point types have equal ranks, and Clang has none of them. So
  every test of ICS-35 and ICS-44 needs an override for Clang. No
  matrix target has three floating-point types of one rank, so no rule
  orders two targets that both have the rank of the source.
- ICS-38: every compiler binds a reference to a braced string literal of
  the referenced type directly. An rvalue reference cannot bind to the
  literal, and an lvalue reference wins over an rvalue reference.
- ICS-6: every compiler rejects a braced list for an ellipsis, and
  chooses another viable function when one exists.
- ICS-10, ICS-41: every compiler accepts a user-defined sequence to the
  explicit object parameter of an assignment operator.

### Probe grids

The 11 probe grids of this page are in
`tests/overloading/conversion-sequences/grids/`. Each grid probes every
pair of a set of parameters against a set of arguments, and its table
shows which parameter each argument chooses. The tables are in
[`grids/tables.md`](https://github.com/sempuki/tidal/blob/main/tests/overloading/conversion-sequences/grids/tables.md),
which also says how to read them. `tools/probe_grid.py` prints each
table from its grid file.

### Sources

The table gives the source of each rule (decision 0044).

| Rule | Source |
|---|---|
| ICS-1, ICS-2 | grids arithmetic, pointers, classes; inventory A5, A8 to A13, B1, B3 to B5 |
| ICS-3 | grid user-defined; inventory A14 to A17 |
| ICS-4 | diagnostics for explicit conversion functions; probes of conversion functions that return a value; inventory E1 to E5; gap review E5, M6 |
| ICS-5 | probe of a defaulted conversion template; inventory A18 |
| ICS-6 | grids classes, user-defined; probes of braced lists for an ellipsis; inventory A4; front-end finding G51 |
| ICS-7 | grid user-defined (ToBoth); probes of a constructor against a conversion function; inventory A1 to A3; gap review E2 |
| ICS-8 | diagnostics in the `chosen-*` tests; inventory A7, C1, F6, F9, F19; gap review M1; front-end finding G43 |
| ICS-9, ICS-10 | probes; inventory D1, D3 to D7, G1, G3; gap review M9 |
| ICS-11, ICS-12 | grids references, list-references; inventory B2, C2 to C6; front-end finding G38 |
| ICS-13, ICS-14, ICS-40, ICS-41 | grid object-parameter; inventory B6, C7 to C10, D2; gap review S4, S5, M9 |
| ICS-15 to ICS-20 | grids lists, list-references, classes; inventory D8, D9, F1 to F18; gap review E1, M5, M7, M8; front-end finding G41 |
| ICS-21 | grids classes, user-defined; inventory H1 to H3 |
| ICS-22 to ICS-24 | grid lists; edg-overload (the list checks before the level comparison); inventory I1 to I3; gap review O3, M5; gap review of question cluster 2 |
| ICS-25 | grid user-defined; inventory L1 to L3 |
| ICS-26, ICS-31 | grids pointers, functions, order; edg-overload (identity against qualification); inventory J3, J7, J8; gap review E3; gap review of question cluster 2 |
| ICS-27 | grid arithmetic; inventory J4 |
| ICS-28 to ICS-30 | grids references, functions, object-parameter, order; edg-overload (reference comparison); inventory J1, J2, J5, J6; gap review O1, O2, S4 |
| ICS-32 to ICS-36 | grids arithmetic, pointers, member-pointers, classes, order; edg-overload (standard conversion comparison); inventory K1, K2, K4 to K9; gap review O1, O2, M4, M8, S1 |
| ICS-37 | probes of conversion functions; edg-overload (the source-class check after a user-defined conversion); inventory K3, K10, K11, L4; gap review E2, M2, S2 |
| ICS-38 | probes of braced string literals; gap review S3; front-end finding G49 |
| ICS-46 | probes of references to arrays of known and unknown bound; probes of direct bindings to pointers; front-end finding G49; gap review of question cluster 2 |
| ICS-39 | grid arithmetic (enum : short); gap review S1 |
| ICS-42, ICS-43 | probes of conversion functions; edg-overload (the source-class check after a user-defined conversion); inventory K10, K11; gap review S2 |
| ICS-44 | probes of conversion functions to extended floating-point types; gap review M3 |
| ICS-45 | probes of `static_cast` with explicit conversion functions; gap review E4; gap review of question cluster 2 |

Inputs: edg-overload.
