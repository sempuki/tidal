# Gap review: open questions on overloading, references, and list initialization

Date: 2026-11-03
Role: source reader

Reference: the C++23 working draft in the local reference tree (the
final C++23 draft). The review compares each answer with that draft
only. The cluster holds the 26 open or provisional questions whose
first Affects rule is on the two overloading pages, the references
page, or the list initialization page, and whose Opinions section has
no gap review. The draft already contains the exception for a narrow
bit-field in the narrowing rule (Q-0042), the rule for a call through
the address of an overload set (Q-0143), and the rule that a braced
list for a reference makes its temporary by copy list initialization
(Q-0150). Later changes are mentioned only where they matter, and they
are not in the reference.

## Result

Two answers contradict the reference, and two more contradict it in
part.

- Q-0140 contradicts the reference. The draft makes the call invalid
  whenever two declarations in different scopes both supply the default
  argument that the call uses, even when the two values are equal. The
  draft supports option 2. No compiler in the table follows the draft
  for equal values.
- Q-0142 contradicts the reference in part. The draft removes the less
  constrained of two plain functions from the address of an overload
  set with no condition on their parameter lists. The answer agrees for
  the cases with equal parameter lists. For the test with different
  parameter lists, the draft keeps only the more constrained function,
  which is Clang 18's result.
- Q-0155 contradicts the reference in part, with doubt. The draft's
  sequence for one braced string literal and a character array checks
  only the literal's character kind, not the bound. A literal too long
  for the array still gets the identity, which gives Clang's tie. The
  other two cases agree.
- Q-0165 contradicts the reference in part, with doubt. The draft binds
  a reference to the one braced element only when the referenced type
  is related to the element's type. A function that cannot throw has a
  type that is not related in this sense to the plain function type, so
  the draft makes a temporary, which a function type cannot have. This
  supports option 3 (EDG) for that case. The overload set case is
  unclear.

Four answers have a doubt or an unclear part.

- Q-0134 agrees, with doubt. The draft lists the narrow bit-field
  conversion to the plain signed integer type among the promotions, but
  a note on conversion sequences says that whether an argument is a
  bit-field is ignored.
- Q-0136 agrees for the designated list. For empty braces the draft
  is unclear, and a literal reading leans to option 2.
- Q-0154 is unclear. The draft gives the sequence a rank only and does
  not say whether the nearest-base tie-breaker applies.
- Q-0180 is a matter of quality of implementation. The draft is silent
  on warnings for dangling references.

The other eighteen answers agree with the reference. Q-0294, the
parent model, agrees in its central condition, and its children carry
the exceptions above.

## Questions in the cluster

| Question | Answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0040 | Only braces extend the lifetime (option 1). | agrees | option 1 | The draft states that a temporary bound to a reference member of an aggregate initialized from a parenthesized list lasts only to the end of the full expression. With braces the ordinary extension applies. EDG's acceptance is a defect. |
| Q-0042 | Not narrowing when every value of the width fits (option 1). | agrees | option 1 | The draft has an exception for a bit-field narrower than its declared type. The test is whether the target holds every value of an integer type with that width and the declared type's signedness. For an enumeration the width is compared with the underlying type's. Clang 22 and GCC 16 follow the draft. Clang 18, GCC 14, and EDG apply the older rule. |
| Q-0043 | No brace elision in parentheses (option 1). | agrees | option 1 | The draft initializes each element from exactly one value of the parenthesized list and says in a note that braces are never elided there. An inner aggregate cannot be initialized from one integer, so the program is not valid. |
| Q-0109 | The binding is valid and extends the temporary (option 1). | agrees | option 1 | Member access on an expiring object and subscripting of an expiring array both give an expiring result, so an rvalue reference may bind. The draft's list of forms that carry lifetime extension includes subscripting of an array operand that itself carries it, and member access with a dot. So the whole temporary lives as long as the reference. EDG's rejection is a defect. |
| Q-0110 | The binding is undefined, so constant evaluation rejects it (option 1). | agrees | option 1 | The draft demands that every reference be bound to an existing object or function, and a note says that indirection through a null pointer is undefined. Constant evaluation rejects any undefined operation. A change after C++23 states the undefined indirection outright. GCC and Clang 22 follow that reading. EDG and Clang 18 do not. |
| Q-0132 | The sequence exists, and the call fails only if its function is chosen (option 1). | agrees | option 1 | The draft lets a constructor marked explicit take part when a braced list chooses a constructor, and makes copy list initialization invalid only when that constructor is chosen. A note adds that the ban matters only for the function that overload resolution finally picks. GCC follows the draft. |
| Q-0134 | The conversion is a promotion with promotion rank (option 1). | agrees, with doubt | option 1 | See below. |
| Q-0135 | The two sequences tie (option 1). | agrees | option 1 | The tie-breaker between list sequences compares two array targets or an initializer list target with another. A scalar target is neither, and no other rule separates two identity sequences when only one side binds a reference. GCC's choice is a defect. |
| Q-0136 | No sequence for empty braces or a designated list (option 1). | agrees for the designated list; unclear for empty braces | option 1 for the designated list | See below. |
| Q-0140 | Not valid if the defaults differ (option 1). | contradicts | option 2 | See below. |
| Q-0141 | The class operand converts by its conversion function only (option 1). | agrees | option 1 | When a built-in candidate wins, the draft converts a class operand with the user-defined part of its sequence and leaves out the standard conversion that would follow. The built-in operator then sees a pointer and a floating type, which is not valid. EDG's acceptance is a defect. |
| Q-0142 | A more constrained function removes another when they meet the conditions of ORD-13 (option 1). | contradicts, in part | option 1 without the ORD-13 conditions | See below. |
| Q-0143 | The static member wins (option 1). | agrees | option 1 | The draft runs overload resolution on the set with no implied object argument, and makes the program invalid only when the chosen function is an implicit object member function. Clang 22 follows the draft. |
| Q-0149 | The reference keeps its declared type, and empty braces are rejected (option 1). | agrees | option 1 | The draft gives the temporary the type that a variable of array type with unknown bound would get from the same list, so the temporary has one element per value. Nothing changes the reference's own declared type. A variable of unknown bound may not be initialized from empty braces, so the temporary for empty braces is not valid either. |
| Q-0150 | The temporary is copy initialized, so an explicit constructor is not valid (option 1). | agrees | option 1 | The draft makes the temporary for a braced list bound to a reference by copy list initialization, with or without the equals sign. An example in the draft rejects this case. Clang does not follow the draft. |
| Q-0151 | A parenthesized initializer of a reference holds one expression (option 1). | agrees | option 1 | The draft requires a parenthesized initializer for an entity that is not of class type to be a single expression. A reference is not of class type. The same holds for a reference to an array. GCC's acceptance is a defect. |
| Q-0154 | The element's sequence applies, with the nearest-base tie-breaker (option 1). | unclear | none clearly | See below. |
| Q-0155 | A sequence exactly when the literal can initialize the array (option 1). | agrees for the UTF-8 and bound cases; contradicts for the literal that does not fit, with doubt | option 1 except the size condition | See below. |
| Q-0157 | A temporary unless the element has the referenced type or a derived class (option 2). | agrees | option 2 | The draft binds through the single element only when the referenced type is related to the element's type (the same type up to qualifiers, a similar array type, or a base). Otherwise a temporary is made by copy list initialization, and a non-const lvalue reference cannot bind to it. Clang and EDG follow the draft. |
| Q-0164 | Bind directly, as after the equals sign (option 1). | agrees | option 1 | The draft treats an array of known bound and an array of unknown bound with the same element type as similar, so the referenced type is related to the element's type and the reference binds directly. |
| Q-0165 | Bind as the equals sign does in each case (option 1). | contradicts, in part, with doubt | option 3 for the function that cannot throw; unclear for the overload set | See below. |
| Q-0166 | The literal initializes the temporary as for an array variable (option 1). | agrees | option 1 | An array of a different character type or a different bound is not related to the literal's type, so the draft makes a temporary by copy list initialization. A single braced string literal may then initialize a character array of a fitting kind, including an ordinary character array from a UTF-8 literal. GCC's rejection is a defect. |
| Q-0167 | Ambiguous (option 1). | agrees | option 1 | The draft gives both candidates the identity. The reference binding tie-breakers compare rvalue against lvalue references and qualifiers, and none prefers a direct binding over a binding to a temporary. |
| Q-0180 | A general recommended rule, tested on one case (option 1). | silent | none | The draft has no recommendation on warnings for dangling references. Notes in the draft point out that a parenthesized aggregate with a reference member and a new expression can leave a dangling reference. |
| Q-0294 | The model of REF-P3 (option 1). | agrees, with the exceptions of its children | option 1 | The draft's condition for a direct binding is that the referenced type is related to the element's type, which matches the model's condition for object types. The children Q-0136, Q-0154, Q-0155, and Q-0165 carry the exceptions. I did not review Q-0081 and Q-0188, which belong to other clusters. |
| Q-0331 | The values that ENUM-22 gives (option 1). | agrees | option 1 | The narrowing rule asks whether the target holds all the values of the source type. For an enumeration without a fixed type, the draft defines its values as those of the smallest bit width that holds every enumerator. GCC follows this. |

### Q-0134 in more detail

- The draft lists among the integral promotions the conversion of a
  bit-field to the plain signed integer type when that type can hold
  every value of the bit-field. The list has no condition on the
  bit-field's declared type, so a narrow bit-field declared with a
  wider type promotes. A promotion has promotion rank in a sequence.
  This is Clang's result.
- The doubt is a note in the general part on conversion sequences. It
  says that sequences ignore whether the argument is a bit-field.
  Read strictly, the note would make the sequence follow the declared
  type, which is GCC's and EDG's result. A note is not binding, and the
  promotion rule is, so I lean to option 1.

### Q-0136 in more detail

- A designated list has a sequence only to an aggregate that aggregate
  initialization can fill from that list. Designators name members of
  a class, and an array has none, so no sequence exists. This agrees
  with option 1.
- For empty braces and a reference to an array of unknown bound, the
  draft's array rule asks for a sequence from each element of the list
  to the element type. An empty list meets that condition with nothing
  to check. The rule then takes the worst of the element sequences,
  which is undefined for no elements. Read literally, a sequence
  exists, and the tie-breaker that prefers fewer initialized elements
  makes it beat the two-element array. That is GCC's result.
- The initialization that would follow is not valid. The temporary
  would be an array of unknown bound initialized from empty braces,
  which the draft forbids. A sequence can exist for a call that is
  later invalid, as Q-0132 shows, so the invalid initialization does
  not decide the matter.
- I mark empty braces as unclear. The literal reading leans to
  option 2.

### Q-0140 in more detail

- The draft has a rule for a best function found through several
  declarations. If two of those declarations are in different scopes
  and each supplies a default argument that the call needed to be
  viable, the program is not valid. The rule does not compare the two
  values. An example in the draft uses equal values and marks the call
  that relies on them as an error, while a call that passes the
  argument is valid.
- So the call is not valid for equal values and for different values.
  This is option 2. The rule needs a diagnostic.
- GCC rejects only different values, and Clang and EDG accept both
  cases. No compiler in the table follows the draft for equal values.

### Q-0142 in more detail

- The draft's rule for the address of an overload set removes the
  functions whose constraints are not satisfied. Among the remaining
  plain functions, the draft removes any function when another one is
  more constrained. The test for more constrained compares only the
  associated constraints. The rule has no condition on parameter lists
  or on the target type. The same steps run with and without a target.
- With a target and with equal parameter lists, the more constrained
  member wins, which agrees with the answer and with Clang 22 and
  GCC 16.
- Without a target and with different parameter lists, the draft still
  removes the less constrained function, and one function remains. So
  the test that expects a rejection does not follow the draft. Clang 18
  gives the draft's result.
- The conditions of ORD-13 match the draft's rule for calls between two
  plain functions, which does require equal parameter lists. The draft
  does not carry them into the address rule. I have not checked
  whether a later change adds them.

### Q-0154 in more detail

- When the parameter is a class that is not an aggregate and the list
  has one element of a class derived from it, the draft gives the
  sequence conversion rank. It does not call the sequence a change
  of a derived object into its base.
- The nearest-base tie-breaker is stated for standard sequences that
  convert one class to two different bases. The draft does not say
  whether a list sequence described only by its rank counts. So the
  draft does not decide between Clang and GCC on one side and EDG on
  the other.
- For an aggregate parameter, the draft takes the element's own
  sequence, which is a conversion to a base, so the tie-breaker
  applies there. The answer's uniform treatment is a reasonable
  reading, but the draft does not state it.

### Q-0155 in more detail

- The draft gives one braced string literal the identity to a character
  array when the literal's kind suits the array. A UTF-8 literal suits
  an ordinary character array, so the first case agrees with Clang and
  EDG.
- For two arrays of the same element type, one of known and one of
  unknown bound, the list tie-breaker compares the number of elements
  initialized, then prefers the known bound when the counts are equal.
  So the known bound wins, which agrees with the answer.
- The sequence rule does not mention the bound. A literal too long for
  the array still has a suitable kind, so the identity exists. The
  pointer candidate needs only an array-to-pointer conversion, which
  ranking ignores, so the two candidates tie. This is Clang's result
  and does not agree with the answer.
- The doubt is whether a literal that does not fit counts as suitable.
  The initialization of such an array is not valid, but a sequence can
  exist for a call that is later invalid.

### Q-0165 in more detail

- The draft binds a reference to the one braced element as the equals
  sign would only when the referenced type is related to the element's
  type. For function types, related means the same type, since a
  function type is similar only to itself.
- A function that cannot throw has a type that differs from the plain
  function type. The equals sign still binds, because the binding
  rule after the equals sign allows the function pointer conversion.
  With braces the draft goes on to make a temporary of the referenced
  type, which cannot exist for a function type. So the draft rejects
  the braced form, as EDG does. I have not checked whether a later
  change aligns the braced form with the equals sign.
- For an overload set the element has no type until the target picks a
  function. The draft lists the initialized reference as a target that
  resolves an overload set, which supports Clang's and EDG's binding.
  But the list rule chooses its branch from the element's type, which
  an overload set does not have before resolution. GCC's rejection is
  a possible reading. I mark this case as unclear.
