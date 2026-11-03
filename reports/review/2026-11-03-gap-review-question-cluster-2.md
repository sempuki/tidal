# Gap review: question cluster 2

Date: 2026-11-03
Role: source reader

Reference: the C++23 working draft in the local reference tree (the
final C++23 draft). The review compares each provisional answer with
that draft only. Later defect resolutions are mentioned only where they
matter, and they are not in the reference.

## Result

Five of the 17 provisional answers conflict with the reference, in
whole or in part.

- Q-0133 contradicts the reference. The draft supports option 2, the
  change that the question's note asks the review to weigh.
- Q-0153 contradicts the reference for the array bound, where the draft
  supports option 2. For the reference that binds a temporary, the
  answer agrees.
- Q-0188 contradicts the text of the C++23 draft for plain direct
  bindings, which tie there (option 2). For the braced string literal
  the draft agrees with the answer. A later core defect resolution may
  give the answer for every column, and the writer should check it.
- Q-0156 contradicts the reference for the two rvalue reference
  columns, which the draft makes valid (option 2). For the lvalue
  reference to const, the draft text leans to the answer, with doubt.
- Q-0161 agrees for the parenthesized qualified form. For the address
  of the plain unqualified name, the draft leans to option 2.

For Q-0189 the reference is unclear. The other 11 provisional answers
agree with the reference.

## Questions in the cluster

| Question | Provisional answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0133 | The initializer-list preference comes before the kind comparison (option 1). | contradicts | option 2 | The draft compares the basic forms of two sequences first, and a standard sequence beats a user-defined one there. The initializer-list preference sits among the rules for two sequences of the same form. Its override clause reaches only the other rules of that same group. A list sequence whose worst element conversion is user-defined is itself a user-defined sequence, so the identity to the integer parameter wins. |
| Q-0137 | The pointer tie-breakers apply after an array-to-pointer step (option 1). | agrees | option 1 | The array-to-pointer step is an lvalue transformation, and the draft leaves lvalue transformations out when it compares sequences. The pointer conversion that remains starts from a pointer to the derived class, so the base-over-void and nearer-base tie-breakers apply. The draft names pointer sources in these tie-breakers and does not mention array arguments, which is the only doubt. |
| Q-0138 | A function reference takes conversion functions of both kinds, and the matching kind wins (option 1). | agrees | option 1 | The draft lets an lvalue reference to function consider conversion functions that return an rvalue reference, and the reverse. It then has an explicit tie-breaker in the best-function list that prefers the conversion function whose result is the same kind of reference as the target. |
| Q-0139 | The call is ambiguous (option 1). | agrees | option 1 | The draft says that surrogates can have identical declarations that overload resolution cannot tell apart, and that such a call is ambiguous unless another candidate is better. The draft does not tie the conversion of the object for a surrogate to the conversion function that made the surrogate. Either way the two surrogates tie on every argument. |
| Q-0144 | A conversion function gives a surrogate only if its cv-qualifiers include those of the object (option 1). | agrees | option 1 | The draft adds a surrogate only for a conversion function whose cv-qualification is the same as or greater than that of the object. A conversion function without const gives no surrogate for a const object. |
| Q-0145 | Copy-list-initialization takes only conversion functions that are not explicit (option 1). | agrees | option 1 | The list chooses among the constructors of the target, and the copy and move constructors take a reference parameter. That parameter is copy-initialized from the source. Explicit conversion functions enter only when a reference is direct-initialized, or when a temporary for a constructor's reference parameter is made in a direct-initialization of the class. Copy-list-initialization is neither. |
| Q-0147 | The rewritten candidate removes the built-in candidate (option 1). | agrees | option 1 | The draft drops a built-in candidate whose parameter types match those of a non-member candidate or of a rewritten non-member candidate, when that candidate is not a template specialization. The three-way comparison function on the enumeration is such a rewritten candidate. |
| Q-0148 | The kind of reference decides before the template tie-breakers (option 2). | agrees | option 2 | In the draft's list of tie-breakers for the best function, the result sequence comes first, then the kind of function reference, then the plain function over a template specialization, then the more specialized template. |
| Q-0152 | The same-rank tie-breakers come before the rvalue-reference preference, and that preference comes before the qualification step (option 1). | agrees | option 1 | The draft places the same-rank tie-breakers inside the rank step, which comes before the reference steps. The rvalue-reference preference comes before the step that compares qualification conversions. Every row of the table then follows option 1. A small doubt is whether the reference steps apply to bindings that make temporaries. The draft speaks of sequences that include reference bindings, and compilers read it to cover them. |
| Q-0153 | A bound does not decide, and the step compares a reference that binds a temporary (option 1). | contradicts in part | option 2 for the bound; option 1 for the temporary | See below. |
| Q-0156 | Only an explicit conversion function that returns an lvalue reference serves a reference (option 1). | contradicts in part | option 2 for rvalue references; option 1, weakly, for the lvalue reference to const | See below. |
| Q-0159 | The call has no object, the non-static member wins, and the call is not valid (option 1). | agrees | option 1 | The draft uses the current object as the implied object only when the current class is the class of the members found or derives from it. Otherwise it uses an invented object that every member accepts, and the call is ill-formed when a non-static member wins. The exact match then wins and the call is rejected. The implicit member access rewrite inside a member function would also fail here, since the nested class is not related to the outer one. |
| Q-0161 | Only the qualified, unparenthesized form can take the address of a set with a non-static member (option 1). | agrees in part | option 1 for the parenthesized qualified form; option 2, with doubt, for the unqualified form | See below. |
| Q-0163 | The result sequences compare as for arguments, so the identity wins (option 1). | agrees | option 1 | The result tie-breaker compares two standard sequences by the usual rules. The identity counts as a subsequence of any sequence that is not the identity, and a function pointer conversion is not the identity. The result tie-breaker also comes before the plain-over-template tie-breaker, which fits the third column. |
| Q-0187 | The object parameter takes no part (option 1). | agrees | option 1 | In a call with no usable this, the draft invents an object of the class as the implied object. A footnote says the invented object never selects or rejects a function, and the call is ill-formed if a non-static member wins. The normative text gives the invented object no value category, so the support comes mostly from the footnote. |
| Q-0188 | The known bound is better for two direct bindings (option 1). | contradicts in part | option 2 in the C++23 text for plain bindings; option 1 for the braced string literal | See below. |
| Q-0189 | The result tie-breaker compares a constructor with a conversion function (option 1). | unclear | neither clearly; a strict reading gives option 2 | See below. |

### Q-0133 in more detail

- The draft first compares the basic form of the two sequences. A
  standard sequence beats a user-defined sequence at this point, before
  any rule for two sequences of the same form.
- The list sequence to an initializer list of a class takes the form of
  its worst element conversion. A converting constructor makes that
  conversion user-defined.
- The override that lets the initializer-list preference beat other
  rules covers only the rules of the same-form group. It cannot reach
  the form comparison.
- The draft therefore chooses the integer parameter, as Clang 22 does.
  The draft's own examples of the initializer-list preference use two
  standard sequences only.
- I know of no later defect resolution that changes this order.

### Q-0153 in more detail

- Since C++20 the draft treats an array of known bound and an array of
  unknown bound as similar types, and a qualification conversion can
  drop the bound.
- The qualification step prefers the sequence whose result can be
  converted to the other result by a qualification conversion. A
  pointer to an array of three const int converts to a pointer to an
  array of unknown bound of const int, and not the reverse. So the
  known bound wins, as Clang 22 finds. This is option 2.
- For the reference that binds a temporary, the draft takes the
  reference's sequence to be the conversion to the referenced type,
  with top-level const ignored. The two sequences then differ only in
  their qualification conversions, and the less qualified result wins.
  This agrees with option 1 for that case.

### Q-0156 in more detail

- A cast to an rvalue reference, and a direct-initialization of an
  rvalue reference, consider conversion functions that return a value
  or an rvalue reference. In direct-initialization the explicit ones
  are allowed when their result type reaches the target by a
  qualification conversion, which a plain value of the referenced
  type does. The reference binds directly to the materialized result.
  So the second and third columns are valid, as Clang 22 finds.
- For an lvalue reference to a const object type, the draft's candidate
  set for direct binding lists only conversion functions that return
  lvalue references. The fallback through copy-initialization excludes
  explicit conversion functions. Read literally, the first column is
  ill-formed, as GCC 16 and EDG 7.0 find.
- The literal reading for the lvalue reference to const looks like a
  gap. The binding rules for that kind of reference expect a conversion
  to an rvalue, but the candidate set never offers one. The question
  should split the two kinds of reference.

### Q-0161 in more detail

- The draft's own example in the call rules marks the parenthesized
  qualified form as an error. Inside a member function, a parenthesized
  name of a member is rewritten as a member access on the current
  object, and the address of that member access cannot be resolved.
  This agrees with option 1.
- The rewrite does not apply to a name that is the direct, unparenthesized
  operand of the address operator. The address of the plain unqualified
  name is then the address of an overload set. For a call through such
  an address, the draft resolves the set with no implied object and
  makes the call ill-formed only when an implicit object member wins.
  Here the static member wins, so the draft leans to option 2 for this
  form.
- The doubt is a separate rule that limits where a name of an implicit
  object member function can appear. The draft does not say whether
  that limit applies to a name of a mixed set whose chosen member is
  static. Tidal's answer rests on its own rule that only the qualified
  form makes a pointer to member, and the draft reaches the call result
  without that rule.

### Q-0188 in more detail

- In the C++23 draft, a reference that binds directly gets the identity
  sequence. The draft has no step that compares two direct bindings to
  similar array types with different bounds. The last reference step
  needs the referenced types to be the same apart from top-level
  cv-qualifiers, and the bound makes them differ. So plain lvalue and
  xvalue arguments tie, as Clang 22 finds. This is option 2.
- The braced string literal is different. The argument is a braced
  list, and the list rule prefers an array of known bound to one of
  unknown bound with the same element count. That gives the known
  bound, which is option 1, and Clang 22 conflicts with the draft
  there.
- For a braced list that holds the array lvalue, the list rule speaks
  of elements that the list initializes. Whether that counts a single
  array element bound by reference is unclear.
- The C++23 text then treats pointers to arrays (Q-0153) and direct
  references to arrays differently. As far as I recall, a later core
  defect resolution on reference binding between similar types adds a
  preference of this kind for references, and GCC 16 and EDG 7.0 may
  follow it. The writer should confirm that resolution by probe before
  the answer depends on it, as for Q-0181 in cluster 1.

### Q-0189 in more detail

- Copy-initialization of a class from another class is a context where
  the result tie-breaker applies. The tie-breaker compares the
  conversions from each candidate's return type to the destination.
- A constructor has no return type. The draft does say that the chosen
  constructor's call gives a value of the destination type, which
  suggests the identity, but the tie-breaker does not say so.
- If the constructor's result counts as the destination type, the
  identity beats the derived-to-base conversion and the constructor
  wins (option 1). If the tie-breaker needs a return type, it does not
  apply, and the two candidates tie (option 2). The draft gives no
  example and no rule that settles the point.
