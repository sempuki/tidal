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
:rules: ICS-1 ICS-3 ICS-4 ICS-5 ICS-6 ICS-7 ICS-8 ICS-9 ICS-10 ICS-45

A sequence measures how far an argument is from a parameter. A sequence
has one of four kinds:

| Kind | Holds | Rules |
|---|---|---|
| Standard | at most three standard conversions, in a fixed order | ICS-1, ICS-2 |
| User-defined | a standard sequence, one constructor or conversion function, then a second standard sequence | ICS-3, ICS-4, ICS-5, ICS-45 |
| Ellipsis | an argument that matches `...` | ICS-6 |
| Ambiguous | two or more constructors or conversion functions, none of them best | ICS-7 |

A sequence holds at most one user-defined conversion (ICS-9, ICS-10).
Forming a sequence looks only at the argument's type, cv-qualifiers, and
value category. Faults such as access, narrowing, a deleted function, or
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
                              pointers, noexcept dropped from a
                              function pointer
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
user-defined conversions.
:::

:::{principle} ICS-P5
:rules: ICS-21 ICS-22 ICS-23 ICS-24 ICS-25 ICS-26 ICS-27 ICS-28 ICS-29 ICS-30 ICS-31 ICS-32 ICS-33 ICS-34 ICS-35 ICS-36 ICS-37 ICS-42 ICS-43 ICS-44

Two sequences for the same argument compare in these steps. The first
step that tells the two sequences apart decides:

```text
sequences S1 and S2 for one argument
   |
   v
1. list targets        S1 initializes std::initializer_list    ICS-22
                       and S2 does not: S1 is better
                       arrays of one element type: fewer        ICS-23
                       elements, then a known bound, is better  ICS-24
   |
   v
2. kind                standard > user-defined = ambiguous      ICS-21
                       > ellipsis
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
:tests: foundations/conversion-sequences/standard-top-level-cv.cpp foundations/conversion-sequences/standard-three-steps.cpp

A {term}`standard conversion sequence` is the identity, or one to three
standard conversions. The sequence has at most one conversion from each
group of ICS-P2, in the order of the groups. Different top-level
cv-qualifiers on the argument and a non-reference parameter add no
conversion.
:::

:::{rule} ICS-2
:kind: required
:tests: foundations/conversion-sequences/rank-worst-step.cpp foundations/conversion-sequences/rank-class-value.cpp foundations/conversion-sequences/bit-field-promotion.cpp

Each conversion has a {term}`sequence rank`:

- exact match: the identity, an lvalue transformation, a qualification
  adjustment,
- promotion: an integer promotion ({rule}`CONV-1`), the promotions of
  ICS-39, and `float` to `double`,
- conversion: every other conversion of ICS-P2.

The worst conversion gives the rank of the sequence. A class argument
for a parameter of the same class has exact-match rank. For a parameter
of a base class, the rank is conversion.
:::

:::{rule} ICS-39
:kind: required
:tests: foundations/conversion-sequences/fixed-enumeration-promotion.cpp

An unscoped enumeration with a fixed underlying type `U` has a
promotion to `U` and to the type that `U` promotes to ({rule}`CONV-1`).
:::

:::{rule} ICS-3
:kind: required
:tests: foundations/conversion-sequences/user-defined-first-part.cpp foundations/conversion-sequences/user-defined-second-part.cpp

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
:tests: foundations/conversion-sequences/user-defined-candidates.cpp foundations/conversion-sequences/reference-conversion-value-result.cpp

These functions can be the second part of ICS-3:

- for a class parameter, the {term}`converting constructors <converting
  constructor>` of the class, and the conversion functions to the class
  or a class derived from it,
- for another parameter, the conversion functions whose result reaches
  the parameter by a standard sequence,
- for a reference that binds directly to the result, the conversion
  functions that {rule}`OVL-15` gives for the kind of reference.

ICS-45 adds `explicit` conversion functions in direct initialization.
{rule}`CPY-7` and {rule}`OVL-17` remove some constructors.
:::

:::{rule} ICS-45
:kind: required
:tests: foundations/conversion-sequences/user-defined-explicit.cpp foundations/conversion-sequences/user-defined-explicit-reference-value.cpp foundations/conversion-sequences/user-defined-candidates.cpp

Direct initialization can also use an `explicit` conversion function
({rule}`CVF-3`) for these targets:

- a class `C`, when the result type is `C` or a reference to `C`, apart
  from cv-qualifiers. The result goes to the parameter of the
  constructor that copies or moves `C`.
- a target of another type, other than a reference, when the result
  reaches it by a qualification adjustment alone,
- a reference that binds directly to an lvalue result, when the
  result reaches the referenced type by a qualification adjustment
  alone ({question}`Q-0156`).

A function parameter is copy-initialized, so these functions never
serve it.
:::

:::{rule} ICS-5
:kind: required
:tests: foundations/conversion-sequences/template-conversion-exact.cpp

A conversion function template serves a user-defined conversion only if
the third part of ICS-3 has exact-match rank.
:::

:::{rule} ICS-6
:kind: required
:tests: foundations/conversion-sequences/ellipsis-sequence.cpp

An argument that matches the `...` of a function has an
{term}`ellipsis conversion sequence`.
:::

:::{rule} ICS-7
:kind: required
:tests: foundations/conversion-sequences/ambiguous-loses-to-standard.cpp foundations/conversion-sequences/ambiguous-ties-user-defined.cpp foundations/conversion-sequences/chosen-ambiguous.cpp foundations/conversion-sequences/chosen-ambiguous-tie.cpp foundations/conversion-sequences/user-defined-choice-object-parameter.cpp foundations/conversion-sequences/chosen-ambiguous-constructor-conversion.cpp

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
:tests: foundations/conversion-sequences/chosen-ambiguous.cpp foundations/conversion-sequences/chosen-ambiguous-tie.cpp foundations/conversion-sequences/chosen-ambiguous-constructor-conversion.cpp foundations/conversion-sequences/chosen-list-ambiguous.cpp foundations/conversion-sequences/chosen-deleted-constructor.cpp foundations/conversion-sequences/chosen-deleted-conversion-function.cpp foundations/conversion-sequences/chosen-bit-field.cpp foundations/conversion-sequences/chosen-private-base.cpp foundations/conversion-sequences/chosen-ambiguous-base.cpp foundations/conversion-sequences/chosen-narrowing.cpp foundations/conversion-sequences/chosen-designator-order.cpp foundations/conversion-sequences/chosen-explicit-copy-list.cpp

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
:tests: foundations/conversion-sequences/two-user-defined-none.cpp

A sequence holds at most one user-defined conversion. A conversion that
needs two has no sequence, and {rule}`CTOR-22` and {rule}`CVF-10` reject
it. The element conversions of ICS-16 to ICS-19 are separate
conversions.
:::

:::{rule} ICS-10
:kind: required
:tests: foundations/conversion-sequences/user-defined-step-standard-only.cpp foundations/conversion-sequences/implicit-object-standard-only.cpp foundations/conversion-sequences/assignment-left-standard-only.cpp foundations/conversion-sequences/double-brace-standard-only.cpp foundations/conversion-sequences/object-explicit-assignment.cpp

Only a standard sequence can reach these parameters:

- the implicit object parameter of a member function,
- the left operand of a built-in assignment,
- the first parameter of a constructor, and the object parameter of a
  conversion function, that ICS-4 considers,
- a parameter of type `C` or reference to `C` of a constructor of class
  `C`, for an inner braced list, when a list with only that inner list
  initializes `C`.
:::

### References and object parameters

:::{rule} ICS-11
:kind: required
:tests: foundations/conversion-sequences/reference-direct-derived.cpp foundations/conversion-sequences/reference-temporary.cpp foundations/conversion-sequences/reference-conversion-result.cpp foundations/conversion-sequences/reference-conversion-value-result.cpp

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
:::

:::{rule} ICS-12
:kind: required
:tests: foundations/conversion-sequences/reference-no-sequence.cpp foundations/conversion-sequences/reference-function-rvalue.cpp foundations/conversion-sequences/reference-rvalue-other-type.cpp

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
:tests: foundations/conversion-sequences/object-ref-qualifier.cpp foundations/conversion-sequences/object-conversion-function-class.cpp foundations/conversion-sequences/object-using-declaration.cpp foundations/conversion-sequences/object-rvalue-non-const.cpp foundations/conversion-sequences/object-explicit-user-defined.cpp

A non-static member function has an {term}`implicit object parameter`:

- Its type is an lvalue reference to the class, with the function's
  cv-qualifiers. With the ref-qualifier `&&`, it is an rvalue reference.
- For a conversion function, the class is the class of the object.
- For a function that a using-declaration names, the class is the class
  with the using-declaration.
:::

:::{rule} ICS-40
:kind: required
:tests: foundations/conversion-sequences/object-rvalue-non-const.cpp foundations/conversion-sequences/rvalue-reference-object-exception.cpp

If a member function has no ref-qualifier, its implicit object
parameter also binds to an rvalue, in spite of ICS-12. Two such
bindings compare as other lvalue reference bindings do, so ICS-30
applies. ICS-28 does not apply to such a binding.
:::

:::{rule} ICS-41
:kind: required
:tests: foundations/conversion-sequences/object-explicit-user-defined.cpp foundations/conversion-sequences/object-explicit-assignment.cpp

An explicit object parameter is an ordinary parameter. A user-defined
sequence can reach it, also when the function is an assignment
operator.
:::

:::{rule} ICS-14
:kind: required
:tests: foundations/conversion-sequences/static-object-slot.cpp

A static member function that is a candidate in a member call has an
object slot. Any object matches the slot. The slot's sequence ties with
every standard sequence.
:::

### Sequences from a braced list

:::{rule} ICS-15
:kind: required
:tests: foundations/conversion-sequences/list-scalar.cpp

For a parameter that is not a class or a reference, a braced list has
this sequence:

- for an empty list, the identity,
- for one element that is not a braced list, the sequence of the
  element,
- for any other list, no sequence.
:::

:::{rule} ICS-16
:kind: required
:tests: foundations/conversion-sequences/list-initializer-list.cpp foundations/conversion-sequences/list-initializer-list-designated.cpp

For a parameter of type `std::initializer_list<E>` ({rule}`LSUP-1`), a
braced list has a sequence if each element converts to `E`. An element
can use a user-defined conversion. The sequence is the worst element
conversion, or the identity for an empty list. A designated list has no
sequence.
:::

:::{rule} ICS-17
:kind: required
:tests: foundations/conversion-sequences/list-array.cpp foundations/conversion-sequences/list-array-against-scalar.cpp

For a reference to an array of `E` that binds a temporary (ICS-20), a
braced list has a sequence if each element converts to `E` and these
conditions hold:

- A known bound is not less than the number of elements.
- Each element past the list converts from empty braces.

The sequence is the worst element conversion. ICS-38 gives one string
literal for an array of characters.
:::

:::{rule} ICS-38
:kind: required
:tests: foundations/conversion-sequences/list-string-literal.cpp foundations/conversion-sequences/list-string-literal-utf8.cpp foundations/conversion-sequences/list-string-literal-too-short.cpp foundations/conversion-sequences/list-string-literal-bound.cpp foundations/conversion-sequences/list-array.cpp

For a reference to an array of a character type, one string literal in
braces has the identity if the literal can initialize the array
({rule}`LIST-6`, {rule}`LIST-15`). A literal that cannot initialize the
array gives no sequence ({question}`Q-0155`).
:::

:::{rule} ICS-18
:kind: required
:tests: foundations/conversion-sequences/list-aggregate.cpp foundations/conversion-sequences/list-designated.cpp foundations/conversion-sequences/list-aggregate-nearest-base.cpp

For a parameter of an aggregate class, a braced list that can initialize
the aggregate ({rule}`LIST-3`) has a user-defined sequence. A designated
list needs the named members, in any order ({rule}`LIST-5`). A list with
one element of the class or a derived class has the element's sequence.
:::

:::{rule} ICS-19
:kind: required
:tests: foundations/conversion-sequences/list-class-constructor.cpp foundations/conversion-sequences/list-class-tie.cpp foundations/conversion-sequences/list-class-same-class.cpp foundations/conversion-sequences/list-class-list-constructor.cpp foundations/conversion-sequences/list-class-nearest-base.cpp

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
:tests: foundations/conversion-sequences/list-reference.cpp foundations/conversion-sequences/list-reference-direct.cpp foundations/conversion-sequences/list-reference-temporary.cpp foundations/conversion-sequences/list-reference-class-element-scalar.cpp foundations/conversion-sequences/list-reference-class-element-class.cpp

For a reference parameter, a braced list binds the reference as
{rule}`REF-6` and {rule}`REF-14` give ({question}`Q-0157`):

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
:tests: foundations/conversion-sequences/kind-order.cpp

A standard sequence is better than a user-defined sequence or an
ellipsis sequence. A user-defined sequence is better than an ellipsis
sequence. ICS-22 to ICS-24 come first.
:::

:::{rule} ICS-22
:kind: required
:tests: foundations/conversion-sequences/initializer-list-target.cpp foundations/conversion-sequences/initializer-list-over-rank.cpp foundations/conversion-sequences/initializer-list-over-kind.cpp foundations/conversion-sequences/initializer-list-reference.cpp

A list sequence to `std::initializer_list<E>`, or to a reference to it,
is better than a list sequence to any other type. The rule comes before
kind ({question}`Q-0133`) and before rank.
:::

:::{rule} ICS-23
:kind: required
:tests: foundations/conversion-sequences/array-fewer-elements.cpp

If two list sequences go to references to arrays of the same element
type, the sequence to the array with fewer elements is better.
:::

:::{rule} ICS-24
:kind: required
:tests: foundations/conversion-sequences/array-known-bound.cpp foundations/conversion-sequences/list-string-literal-bound.cpp

If the two arrays of ICS-23 have the same number of elements, the array
of known bound is better than the array of unknown bound.
:::

:::{rule} ICS-25
:kind: required
:tests: foundations/conversion-sequences/user-defined-same-function.cpp foundations/conversion-sequences/user-defined-same-aggregate.cpp foundations/conversion-sequences/user-defined-different-functions.cpp

Two user-defined sequences compare by their third parts, as standard
sequences, in these cases:

- Both use the same constructor or conversion function.
- Both initialize the same aggregate class.

Other user-defined sequences tie. The ambiguous sequence ties with every
user-defined sequence.
:::

:::{rule} ICS-26
:kind: required
:tests: foundations/conversion-sequences/subsequence-function-pointer.cpp foundations/conversion-sequences/subsequence-qualification.cpp

Two standard sequences are compared without their lvalue
transformations. If one is a proper subsequence of the other, the
shorter one is better. The identity is a subsequence of every other
sequence.
:::

:::{rule} ICS-27
:kind: required
:tests: foundations/conversion-sequences/rank-comparison.cpp foundations/conversion-sequences/rank-before-reference.cpp

Of two standard sequences, the one of better rank is better. Exact
match is better than promotion, and promotion is better than
conversion. ICS-28 to ICS-36 apply only to sequences of equal rank.
:::

:::{rule} ICS-32
:kind: required
:tests: foundations/conversion-sequences/pointer-to-bool.cpp foundations/conversion-sequences/tie-breaker-before-rvalue-reference.cpp

A sequence that converts a pointer or a member pointer to `bool` is
worse than a sequence of equal rank that does not. ICS-26 and ICS-27
come first.
:::

:::{rule} ICS-33
:kind: required
:tests: foundations/conversion-sequences/base-pointer-over-void.cpp foundations/conversion-sequences/array-pointer-tie-breaker.cpp foundations/conversion-sequences/tie-breaker-before-rvalue-reference.cpp

A conversion of a pointer to a class into a pointer to its base is
better than a conversion of the same pointer into a pointer to `void`.
ICS-26 and ICS-27 come first.
:::

:::{rule} ICS-34
:kind: required
:tests: foundations/conversion-sequences/fixed-enumeration.cpp foundations/conversion-sequences/fixed-enumeration-before-rvalue-reference.cpp

If two sequences promote an unscoped enumeration with a fixed
underlying type to different types (ICS-39), the promotion to the
underlying type is better. ICS-26 and ICS-27 come first.
:::

:::{rule} ICS-35
:kind: required
:tests: foundations/conversion-sequences/floating-equal-rank.cpp foundations/conversion-sequences/floating-before-rvalue-reference.cpp

A conversion from a floating-point type to a floating-point type of
equal rank is better than a conversion to an integer type or to a
floating-point type of other rank. Only the extended floating-point
types *(extended floating-point types referenced)* have equal ranks.
ICS-26 and ICS-27 come first.
:::

:::{rule} ICS-36
:kind: required
:tests: foundations/conversion-sequences/nearest-base.cpp foundations/conversion-sequences/array-pointer-tie-breaker.cpp foundations/conversion-sequences/tie-breaker-before-rvalue-reference.cpp foundations/conversion-sequences/list-class-nearest-base.cpp foundations/conversion-sequences/list-aggregate-nearest-base.cpp

Let `B` be a base of `M`, and `M` a base of `D`. These conversions are
better than the alternative in parentheses:

- a pointer, reference, or class value of `D` to `M` (to `B`),
- a member pointer of `B` to a member pointer of `M` (of `D`).

ICS-26 and ICS-27 come first.
:::

:::{rule} ICS-28
:kind: required
:tests: foundations/conversion-sequences/rvalue-reference-binding.cpp foundations/conversion-sequences/rvalue-reference-before-cv.cpp foundations/conversion-sequences/rvalue-reference-object-exception.cpp foundations/conversion-sequences/rank-before-reference.cpp foundations/conversion-sequences/tie-breaker-before-rvalue-reference.cpp foundations/conversion-sequences/fixed-enumeration-before-rvalue-reference.cpp foundations/conversion-sequences/floating-before-rvalue-reference.cpp foundations/conversion-sequences/rvalue-reference-before-qualification.cpp

If both sequences bind references to an rvalue, binding an rvalue
reference is better than binding an lvalue reference. The rule does not
apply if either sequence binds the implicit object parameter of a
function without a ref-qualifier (ICS-40). ICS-32 to ICS-36 come first
({question}`Q-0152`).
:::

:::{rule} ICS-29
:kind: required
:tests: foundations/conversion-sequences/function-lvalue-reference.cpp

If both sequences bind references to a function lvalue, binding an
lvalue reference is better than binding an rvalue reference.
:::

:::{rule} ICS-31
:kind: required
:tests: foundations/conversion-sequences/qualification-fewer.cpp foundations/conversion-sequences/subsequence-qualification.cpp foundations/conversion-sequences/rvalue-reference-before-qualification.cpp foundations/conversion-sequences/qualification-array-bound.cpp foundations/conversion-sequences/qualification-reference-temporary.cpp

If two sequences differ only in a qualification adjustment, and the
result of one converts to the result of the other by adding
cv-qualifiers, the sequence that adds fewer is better. Only
cv-qualifiers count, and an array bound does not ({question}`Q-0153`).
ICS-28 comes first ({question}`Q-0152`).
:::

:::{rule} ICS-30
:kind: required
:tests: foundations/conversion-sequences/reference-less-cv.cpp foundations/conversion-sequences/rvalue-reference-before-cv.cpp

If both sequences bind references to the same type apart from top-level
cv-qualifiers, the reference with fewer cv-qualifiers is better. Its
cv-qualifiers are a subset of the other's. ICS-28 comes first.
:::

### Choosing a conversion function

:::{rule} ICS-37
:kind: required
:tests: foundations/conversion-sequences/conversion-choice-second-part.cpp

When a user-defined conversion chooses between two conversion functions
whose object parameters tie (ICS-7), the function whose result has the
better third part of ICS-3 wins ({rule}`OVL-26`). The third parts
compare by step 4 of ICS-P5, with ICS-42 to ICS-44 for results of
different types. For a reference to a function, {rule}`OVL-29` decides
between results that still tie ({question}`Q-0138`).
:::

:::{rule} ICS-42
:kind: required
:tests: foundations/conversion-sequences/conversion-choice-nearest-source.cpp foundations/conversion-sequences/conversion-choice-class-value.cpp

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
:tests: foundations/conversion-sequences/conversion-choice-void.cpp

When two results convert to `void*`, a conversion from a pointer to a
class is better than a conversion from a pointer to a class derived
from it.
:::

:::{rule} ICS-44
:kind: required
:tests: foundations/conversion-sequences/conversion-choice-equal-rank-source.cpp

When two results convert to a floating-point type `A`, a conversion from
a floating-point type of the same rank as `A` is better. The other
result is an integer type or a floating-point type of other rank
*(extended floating-point types referenced)*.
:::

## Examples

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
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 85
of 106 tests.

- ICS-8: GCC keeps an `explicit` constructor in the sequence of a
  copy-list-initialized parameter, chooses the function, and rejects the
  call. Clang and EDG form no sequence and choose the other function
  ({question}`Q-0132`).
- ICS-22: GCC and EDG prefer a user-defined list sequence to
  `std::initializer_list` over a standard sequence to `int`. Clang
  compares the kinds first and chooses `int` ({question}`Q-0133`).
- ICS-19: Clang 18 gives a one-element list of a class with an
  initializer-list constructor an exact match. The other compilers use
  the initializer-list constructor ({question}`Q-0081`).
- ICS-35, ICS-44: Clang has no extended floating-point suffixes
  ({rule}`PRIM-22`). GCC and EDG follow the rules. No matrix target has
  three floating-point types of one rank, so no rule orders two targets
  that both have the rank of the source.
- ICS-2: Clang gives a `long long` bit-field of width 3 a promotion to
  `int`. GCC and EDG give it a conversion, so `int` and `long` tie
  ({question}`Q-0134`).
- ICS-17: GCC prefers a scalar parameter to a reference to an array when
  both sequences are the identity. Clang and EDG find a tie
  ({question}`Q-0135`).
- ICS-17, ICS-18: for empty braces and a reference to an array of
  unknown bound, and for a designated list and a reference to an array,
  the compilers differ in the lists grid ({question}`Q-0136`).
- ICS-26, ICS-33, ICS-36: after an array-to-pointer step, GCC applies
  neither ICS-33 nor ICS-36. Clang and EDG apply both
  ({question}`Q-0137`).
- ICS-28, ICS-31 to ICS-36: Clang applies every same-rank tie-breaker
  before ICS-28. GCC does so except for ICS-34, and EDG applies ICS-28
  first. Clang and EDG apply ICS-28 before ICS-31, and GCC applies
  ICS-31 first ({question}`Q-0152`, the order grid).
- ICS-31: Clang prefers a known array bound. EDG does not compare a
  reference that binds a temporary with a value parameter
  ({question}`Q-0153`).
- ICS-19, ICS-36: EDG finds a tie between `Middle` and `Base` for a
  one-element list of `Derived` when the classes are not aggregates
  ({question}`Q-0154`).
- ICS-38: GCC gives a braced UTF-8 literal no sequence to a `char`
  array. Clang gives a literal a sequence to an array that is too short,
  and finds a tie between a known and an unknown bound
  ({question}`Q-0155`).
- ICS-45: Clang lets direct initialization of a reference use an
  `explicit` conversion function that returns a value. GCC and EDG do
  not ({question}`Q-0156`).
- ICS-20: for a list with one class element that converts to the
  referenced type, Clang and EDG make a temporary unless the element
  has the referenced type or a derived class. GCC binds through the
  conversion function. ICS-20 follows REF-6 and REF-14
  ({question}`Q-0157`). Every compiler binds a reference to an array
  directly to a one-element list of an array lvalue of that type.
- ICS-37: Clang finds two conversion functions to `F&` and `F&&`
  ambiguous for a reference `F&&` to a function type. GCC and EDG apply
  {rule}`OVL-29` ({question}`Q-0138`).
- ICS-10, ICS-41: every compiler accepts a user-defined sequence to the
  explicit object parameter of an assignment operator.
- ICS-8 is a `reject` rule because each fault is rejected once its
  function is chosen. Every test has a fallback function that the call
  would use if the fault removed the sequence.

The probe grids are in `tests/foundations/conversion-sequences/grids/`,
and `tools/probe_grid.py` prints these tables from them. Each column of a
grid declares two overloads, `One f(a)` and `Two f(b)`. The row
"X → first" checks that the call with X chooses `f(a)`, and the row
"X → second" checks that it chooses `f(b)`. An `accept` in one of the two
rows names the better sequence. A `reject` in both rows means that the
sequences tie or that no function is viable. In the labels, lref stands
for `&` and rref for `&&`. In the object-parameter grid, each column
gives the qualifiers of a member `operator~` and the parameter of a
non-member `operator~`. In the order grid, each column gives the
parameters of `f(a)` and `f(b)`, and each row gives the argument.

Arithmetic arguments (ICS-2, ICS-27, ICS-32, ICS-34):

| | bool / short | bool / int | bool / long | bool / float | bool / double | short / int | short / long | short / float | short / double | int / long | int / float | int / double | long / float | long / double | float / double |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| bool → first | accept | accept | accept | accept | accept | reject | reject | reject | reject | accept | accept | accept | reject | reject | reject |
| bool → second | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| char → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | reject | reject | reject |
| char → second | reject | accept | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| short → first | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| short → second | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | reject | reject | reject |
| int → second | reject | accept | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| unsigned → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| unsigned → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| long → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | reject |
| long → second | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept | reject | reject | reject | reject | reject |
| float → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| float → second | reject | reject | reject | accept | accept | reject | reject | accept | accept | reject | accept | accept | accept | accept | reject |
| double → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| double → second | reject | reject | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept | reject | accept | accept |
| long double → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| long double → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| enum → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | reject | reject | reject |
| enum → second | reject | accept | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| enum : short → first | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| enum : short → second | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| enum class → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| enum class → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| long long : 3 bit-field → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept: clang-host, clang-22, clang-18; reject: gcc-host, edg-7.0, gcc-14, gcc-16, edg-7.0-ilp32 | accept: clang-host, clang-22, clang-18; reject: edg-7.0, gcc-16, edg-7.0-ilp32, gcc-host, gcc-14 | accept: clang-22, clang-18, clang-host; reject: edg-7.0, gcc-host, edg-7.0-ilp32, gcc-16, gcc-14 | reject | reject | reject |
| long long : 3 bit-field → second | reject | accept: clang-host, clang-22, clang-18; reject: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-host, gcc-16 | reject | reject | reject | accept: clang-host, clang-18, clang-22; reject: edg-7.0, gcc-16, gcc-host, edg-7.0-ilp32, gcc-14 | reject | reject | reject | reject | reject | reject | reject | reject | reject |

Pointer arguments (ICS-26, ICS-31 to ICS-33, ICS-36):

| | void* / const void* | void* / Base* | void* / Middle* | void* / const Middle* | void* / bool | const void* / Base* | const void* / Middle* | const void* / const Middle* | const void* / bool | Base* / Middle* | Base* / const Middle* | Base* / bool | Middle* / const Middle* | Middle* / bool | const Middle* / bool |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Derived* → first | accept | reject | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept | accept | accept | accept |
| Derived* → second | reject | accept | accept | accept | reject | accept | accept | accept | reject | accept | accept | reject | reject | reject | reject |
| const Derived* → first | reject | reject | reject | reject | reject | accept | accept | reject | accept | reject | reject | reject | reject | reject | accept |
| const Derived* → second | accept | reject | reject | accept | accept | reject | reject | accept | reject | reject | accept | accept | accept | accept | reject |
| Middle* → first | accept | reject | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept | accept | accept | accept |
| Middle* → second | reject | accept | accept | accept | reject | accept | accept | accept | reject | accept | accept | reject | reject | reject | reject |
| Derived[2] → first | accept | reject | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept | accept | accept | accept |
| Derived[2] → second | reject | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 | accept: edg-7.0-ilp32, edg-7.0, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 | reject | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-22, clang-18; reject: gcc-host, gcc-14, gcc-16 | accept: edg-7.0-ilp32, edg-7.0, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 | accept: edg-7.0-ilp32, edg-7.0, clang-host, clang-18, clang-22; reject: gcc-14, gcc-host, gcc-16 | reject | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-14, gcc-host, gcc-16 | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 | reject | reject | reject | reject |
| 0 → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| 0 → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| nullptr → first | reject | reject | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept | reject | accept | accept |
| nullptr → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |

Member pointer arguments (ICS-32, ICS-36):

| | int Base::* / int Middle::* | int Base::* / int Derived::* | int Base::* / bool | int Middle::* / int Derived::* | int Middle::* / bool | int Derived::* / bool |
|---|---|---|---|---|---|---|
| &Base::x → first | accept | accept | accept | accept | accept | accept |
| &Base::x → second | reject | reject | reject | reject | reject | reject |
| &Middle::y → first | reject | reject | reject | accept | accept | accept |
| &Middle::y → second | accept | accept | accept | reject | reject | reject |
| 0 → first | reject | reject | reject | reject | reject | reject |
| 0 → second | reject | reject | reject | reject | reject | reject |
| nullptr → first | reject | reject | accept | reject | accept | accept |
| nullptr → second | reject | reject | reject | reject | reject | reject |

Function arguments (ICS-26, ICS-29, ICS-32):

| | pointer / noexcept pointer | pointer / lref | pointer / rref | pointer / bool | noexcept pointer / lref | noexcept pointer / rref | noexcept pointer / bool | lref / rref | lref / bool | rref / bool |
|---|---|---|---|---|---|---|---|---|---|---|
| run → first | accept | reject | reject | accept | reject | reject | reject | accept | accept | accept |
| run → second | reject | reject | reject | reject | accept | accept | accept | reject | reject | reject |
| stop (noexcept) → first | reject | reject | reject | accept | reject | reject | accept | accept | accept | accept |
| stop (noexcept) → second | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject |

Reference parameters (ICS-11, ICS-12, ICS-28, ICS-30):

| | int / int lref | int / const int lref | int / const volatile int lref | int / int rref | int / const int rref | int / const long lref | int lref / const int lref | int lref / const volatile int lref | int lref / int rref | int lref / const int rref | int lref / const long lref | const int lref / const volatile int lref | const int lref / int rref | const int lref / const int rref | const int lref / const long lref | const volatile int lref / int rref | const volatile int lref / const int rref | const volatile int lref / const long lref | int rref / const int rref | int rref / const long lref | const int rref / const long lref |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| int lvalue → first | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| int lvalue → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept |
| const int lvalue → first | accept | reject | reject | accept | accept | accept | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| const int lvalue → second | reject | reject | reject | reject | reject | reject | accept | accept | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept |
| int xvalue → first | accept | reject | accept | reject | reject | accept | reject | reject | reject | reject | reject | accept | reject | reject | accept | reject | reject | reject | accept | accept | accept |
| int xvalue → second | reject | reject | reject | reject | reject | reject | accept | reject | accept | accept | accept | reject | accept | accept | reject | accept | accept | accept | reject | reject | reject |
| int prvalue → first | accept | reject | accept | reject | reject | accept | reject | reject | reject | reject | reject | accept | reject | reject | accept | reject | reject | reject | accept | accept | accept |
| int prvalue → second | reject | reject | reject | reject | reject | reject | accept | reject | accept | accept | accept | reject | accept | accept | reject | accept | accept | accept | reject | reject | reject |
| short lvalue → first | accept | reject | accept | reject | reject | accept | reject | reject | reject | reject | reject | accept | reject | reject | accept | reject | reject | reject | accept | accept | accept |
| short lvalue → second | reject | reject | reject | reject | reject | reject | accept | reject | accept | accept | accept | reject | accept | accept | reject | accept | accept | accept | reject | reject | reject |
| int : 3 bit-field → first | reject | reject | reject | accept | accept | accept | reject | reject | reject | reject | reject | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| int : 3 bit-field → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept |

Class arguments and parameters (ICS-2, ICS-11, ICS-18, ICS-21, ICS-36):

| | Base / Middle | Base / const Middle lref | Base / Middle rref | Base / long | Base / Box | Base / ellipsis | Middle / const Middle lref | Middle / Middle rref | Middle / long | Middle / Box | Middle / ellipsis | const Middle lref / Middle rref | const Middle lref / long | const Middle lref / Box | const Middle lref / ellipsis | Middle rref / long | Middle rref / Box | Middle rref / ellipsis | long / Box | long / ellipsis | Box / ellipsis |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Derived lvalue → first | reject | reject | accept | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| Derived lvalue → second | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | accept | accept |
| Derived prvalue → first | reject | reject | reject | accept | accept | accept | reject | reject | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| Derived prvalue → second | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | accept | accept |
| Middle prvalue → first | reject | reject | reject | accept | accept | accept | reject | reject | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| Middle prvalue → second | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | accept | accept |
| int → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept |
| int → second | reject | reject | reject | accept | accept | accept | reject | reject | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| ToMiddle → first | reject | reject | reject | accept | accept | accept | reject | reject | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| ToMiddle → second | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | accept | accept |
| ToInt → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | reject |
| ToInt → second | reject | reject | reject | accept | reject | accept | reject | reject | accept | reject | accept | reject | accept | reject | accept | accept | reject | accept | reject | reject | accept |
| braced {d} → first | reject | reject | accept | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| braced {d} → second | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |

User-defined sequences (ICS-3, ICS-7, ICS-21, ICS-25):

| | int / long | int / double | int / Box | int / Wide | int / ellipsis | long / double | long / Box | long / Wide | long / ellipsis | double / Box | double / Wide | double / ellipsis | Box / Wide | Box / ellipsis | Wide / ellipsis |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| int → first | accept | accept | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | reject | accept | accept |
| int → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| ToInt → first | accept | accept | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| ToInt → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept |
| ToShort → first | accept | accept | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| ToShort → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept |
| ToBoth → first | reject | reject | accept | accept | accept | reject | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| ToBoth → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept |

Braced-list arguments (ICS-15 to ICS-19, ICS-22 to ICS-24):

| | int / initializer_list<int> | int / initializer_list<long> | int / int[2] rref | int / int[] rref | int / Point | int / Pair | initializer_list<int> / initializer_list<long> | initializer_list<int> / int[2] rref | initializer_list<int> / int[] rref | initializer_list<int> / Point | initializer_list<int> / Pair | initializer_list<long> / int[2] rref | initializer_list<long> / int[] rref | initializer_list<long> / Point | initializer_list<long> / Pair | int[2] rref / int[] rref | int[2] rref / Point | int[2] rref / Pair | int[] rref / Point | int[] rref / Pair | Point / Pair |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| empty {} → first | reject | reject | accept: gcc-14, gcc-16, gcc-host; reject: edg-7.0-ilp32, clang-18, clang-host, edg-7.0, clang-22 | accept: clang-host, clang-18, gcc-16, gcc-14, gcc-host, clang-22; reject: edg-7.0-ilp32, edg-7.0 | accept | accept | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept: clang-host, clang-18, clang-22; reject: edg-7.0-ilp32, gcc-16, gcc-14, edg-7.0, gcc-host | accept | accept | accept: gcc-host, gcc-14, gcc-16; reject: clang-host, edg-7.0-ilp32, edg-7.0, clang-18, clang-22 | accept: gcc-16, gcc-host, gcc-14; reject: edg-7.0-ilp32, edg-7.0, clang-host, clang-18, clang-22 | accept |
| empty {} → second | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept: gcc-14, gcc-host, gcc-16; reject: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22 | reject | reject | accept: clang-host, clang-18, clang-22; reject: edg-7.0, edg-7.0-ilp32, gcc-host, gcc-14, gcc-16 | reject | reject |
| one {1} → first | reject | reject | accept: gcc-host, gcc-14, gcc-16; reject: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22 | accept: gcc-host, gcc-14, gcc-16; reject: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22 | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | accept | accept | accept | accept | accept |
| one {1} → second | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject |
| two {1, 2} → first | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject |
| two {1, 2} → second | accept | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| three {1, 2, 3} → first | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | accept | accept | reject |
| three {1, 2, 3} → second | accept | accept | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject |
| nested {{1}} → first | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | accept | accept | accept | accept | accept |
| nested {{1}} → second | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject |
| designated {.x = 1} → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| designated {.x = 1} → second | reject | reject | reject | reject | accept | reject | reject | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept: clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, edg-7.0-ilp32, edg-7.0, gcc-16 | reject | accept: clang-host, clang-18, clang-22; reject: gcc-host, edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16 | reject | reject |

Object parameters (ICS-13, ICS-28):

| | no qualifier / Item lref | no qualifier / Item rref | const / const Item lref | const / const Item rref | const lref / const Item rref | lref / Item rref | rref / Item lref | rref / const Item lref |
|---|---|---|---|---|---|---|---|---|
| Item lvalue → member | reject | accept | reject | accept | accept | accept | reject | reject |
| Item lvalue → non-member | reject | reject | reject | reject | reject | reject | accept | accept |
| const Item lvalue → member | reject | reject | reject | accept | accept | reject | reject | reject |
| const Item lvalue → non-member | reject | reject | reject | reject | reject | reject | reject | accept |
| Item prvalue → member | accept | reject | reject | reject | reject | reject | accept | accept |
| Item prvalue → non-member | reject | reject | reject | reject | accept | accept | reject | reject |

Comparison order (ICS-28, ICS-31 to ICS-36):

| | const short lref / int rref | void* const lref / bool rref | const Middle lref / Base rref | Base* const lref / void* rref | Middle* const lref / Base* rref | int Middle::* const lref / int Derived::* rref | const double lref / long double rref | const int* const lref / const volatile int* rref |
|---|---|---|---|---|---|---|---|---|
| low (enum : short) → first | accept: clang-18, clang-host, clang-22; reject: gcc-host, gcc-16, gcc-14, edg-7.0-ilp32, edg-7.0 | reject | reject | reject | reject | reject | reject | reject |
| low (enum : short) → second | accept: gcc-14, edg-7.0-ilp32, edg-7.0, gcc-host, gcc-16; reject: clang-18, clang-host, clang-22 | accept | reject | reject | reject | reject | accept | reject |
| &n (int*) → first | reject | accept: gcc-host, gcc-16, clang-host, gcc-14, clang-18, clang-22; reject: edg-7.0-ilp32, edg-7.0 | reject | reject | reject | reject | reject | accept: gcc-14, gcc-16, gcc-host; reject: edg-7.0-ilp32, edg-7.0, clang-host, clang-18, clang-22 |
| &n (int*) → second | reject | accept: edg-7.0-ilp32, edg-7.0; reject: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22 | reject | accept | reject | reject | reject | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 |
| Derived{} → first | reject | reject | accept: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22; reject: edg-7.0, edg-7.0-ilp32 | reject | reject | reject | reject | reject |
| Derived{} → second | reject | reject | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22 | reject | reject | reject | reject | reject |
| &d (Derived*) → first | reject | accept: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22; reject: edg-7.0, edg-7.0-ilp32 | reject | accept: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22; reject: edg-7.0, edg-7.0-ilp32 | accept: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22; reject: edg-7.0-ilp32, edg-7.0 | reject | reject | reject |
| &d (Derived*) → second | reject | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22 | reject | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22 | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22 | reject | reject | reject |
| &Base::x → first | reject | reject | reject | reject | reject | accept: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22; reject: edg-7.0, edg-7.0-ilp32 | reject | reject |
| &Base::x → second | reject | accept | reject | reject | reject | accept: edg-7.0-ilp32, edg-7.0; reject: gcc-14, gcc-host, gcc-16, clang-host, clang-18, clang-22 | reject | reject |
| 1.0f64 → first | reject | reject | reject | reject | reject | reject | accept: gcc-14, gcc-host, gcc-16; reject: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22 | reject |
| 1.0f64 → second | accept: edg-7.0, edg-7.0-ilp32, gcc-host, gcc-14, gcc-16; reject: clang-host, clang-18, clang-22 | accept: edg-7.0-ilp32, edg-7.0, gcc-14, gcc-host, gcc-16; reject: clang-host, clang-18, clang-22 | reject | reject | reject | reject | accept: edg-7.0-ilp32, edg-7.0; reject: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22 | reject |

Braced lists for references (ICS-12, ICS-20, ICS-30):

| | int lref / const int lref | int lref / int rref | int lref / const int rref | int lref / const long lref | int lref / long | const int lref / int rref | const int lref / const int rref | const int lref / const long lref | const int lref / long | int rref / const int rref | int rref / const long lref | int rref / long | const int rref / const long lref | const int rref / long | const long lref / long |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| {i} int lvalue → first | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| {i} int lvalue → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | reject |
| {ci} const int lvalue → first | reject | reject | reject | reject | reject | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| {ci} const int lvalue → second | accept | reject | reject | accept | accept | reject | reject | reject | reject | reject | accept | accept | accept | accept | reject |
| {1} prvalue → first | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | reject |
| {1} prvalue → second | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject |
| {s} short lvalue → first | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | reject |
| {s} short lvalue → second | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject |

The source of each rule:

| Rule | Source |
|---|---|
| ICS-1, ICS-2 | grids arithmetic, pointers, classes; inventory A5, A8 to A13, B1, B3 to B5 |
| ICS-3 | grid user-defined; inventory A14 to A17 |
| ICS-4 | diagnostics for explicit conversion functions; probes of conversion functions that return a value; inventory E1 to E5; gap review E5, M6 |
| ICS-5 | probe of a defaulted conversion template; inventory A18 |
| ICS-6 | grids classes, user-defined; inventory A4 |
| ICS-7 | grid user-defined (ToBoth); probes of a constructor against a conversion function; inventory A1 to A3; gap review E2 |
| ICS-8 | diagnostics in the `chosen-*` tests; inventory A7, C1, F6, F9, F19; gap review M1; front-end finding G43 |
| ICS-9, ICS-10 | probes; inventory D1, D3 to D7, G1, G3; gap review M9 |
| ICS-11, ICS-12 | grids references, list-references; inventory B2, C2 to C6; front-end finding G38 |
| ICS-13, ICS-14, ICS-40, ICS-41 | grid object-parameter; inventory B6, C7 to C10, D2; gap review S4, S5, M9 |
| ICS-15 to ICS-20 | grids lists, list-references, classes; inventory D8, D9, F1 to F18; gap review E1, M5, M7, M8; front-end finding G41 |
| ICS-21 | grids classes, user-defined; inventory H1 to H3 |
| ICS-22 to ICS-24 | grid lists; edg-overload (the list checks before the level comparison); inventory I1 to I3; gap review O3, M5 |
| ICS-25 | grid user-defined; inventory L1 to L3 |
| ICS-26, ICS-31 | grids pointers, functions, order; edg-overload (identity against qualification); inventory J3, J7, J8; gap review E3 |
| ICS-27 | grid arithmetic; inventory J4 |
| ICS-28 to ICS-30 | grids references, functions, object-parameter, order; edg-overload (reference comparison); inventory J1, J2, J5, J6; gap review O1, O2, S4 |
| ICS-32 to ICS-36 | grids arithmetic, pointers, member-pointers, classes, order; edg-overload (standard conversion comparison); inventory K1, K2, K4 to K9; gap review O1, O2, M4, M8, S1 |
| ICS-37 | probes of conversion functions; edg-overload (the source-class check after a user-defined conversion); inventory K3, K10, K11, L4; gap review E2, M2, S2 |
| ICS-38 | probes of braced string literals; gap review S3 |
| ICS-39 | grid arithmetic (enum : short); gap review S1 |
| ICS-42, ICS-43 | probes of conversion functions; edg-overload (the source-class check after a user-defined conversion); inventory K10, K11; gap review S2 |
| ICS-44 | probes of conversion functions to extended floating-point types; gap review M3 |
| ICS-45 | probes of `static_cast` with explicit conversion functions; gap review E4 |

Inputs: edg-overload.
