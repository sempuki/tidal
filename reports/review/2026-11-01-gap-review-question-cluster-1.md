# Gap review: question cluster 1

Date: 2026-11-01
Role: source reader

Reference: the C++23 working draft in the local reference tree (the
final C++23 draft). The review compares each answer with that draft
only. Later defect resolutions are mentioned only where they matter,
and they are not in the reference.

## Result

Two of the 19 settled questions conflict with the reference.

- Q-0300 contradicts the reference. The draft has the reverse step
  that the answer denies. All five compilers share the same behavior,
  so this is the shared-defect case that the review looks for.
- Q-0181 contradicts the text of the C++23 draft. A later core defect
  resolution, which compilers apply to C++23 mode, gives the settled
  answer. The question should name that dependency.

Of the other 17 settled answers, 16 agree with the reference. For
Q-0271 the reference is silent.

## Settled questions

| Question | Settled answer (Tidal) | Verdict | Note |
|---|---|---|---|
| Q-0010 | Valid, deducing from the single element as if the braces were absent. | supports | For a braced initializer with one element and no equals sign, the deduction uses that element as the operand of decltype. A list of several elements is an error. |
| Q-0015 | Valid; the Clang 18 rejection is fixed in later versions. | supports | A structured binding with a braced initializer and an array initializes each element from the matching array element. Braces and parentheses act the same way. |
| Q-0023 | Rejected; the GCC 14 acceptance is fixed in GCC 16. | supports | The draft makes a program ill-formed when it defines the main function as deleted. |
| Q-0026 | Rejected; the GCC 14 acceptance is fixed in GCC 16. | supports | In the declaration of a range-based for loop, only type specifiers and constexpr may appear. Neither static nor thread_local is allowed. |
| Q-0072 | alignas must not occur on a function, a type alias, or a bit-field. | supports | The draft lets an alignment specifier apply to variables, data members, and class declarations. It forbids bit-fields by name, and functions and aliases are outside the allowed list. |
| Q-0101 | The operand is valid, as contextual conversion to bool permits. | supports | Contextual conversion uses direct-initialization of a bool. That initialization gives false for a null pointer constant of the null pointer type. |
| Q-0125 | Keep NEW-19; Clang 22 declares the sized forms. | supports | The draft lists the sized array and non-array deallocation functions among the functions that every translation unit declares implicitly. |
| Q-0169 | The argument is valid when every argument list that the parameter accepts is valid for the argument. | supports | The draft matches a template template argument when the parameter is at least as specialized as the argument. Extra parameters with defaults pass that test. |
| Q-0177 | Both positions are valid; GCC 14 has a defect. | supports | The grammar puts an optional attribute list at the start of each base specifier. The grammar also puts one after each bound in a new declarator. |
| Q-0181 | The prvalue initializes the parameter directly, as any prvalue of the same type does. | contradicts | In the C++23 draft, the dot operator needs a glvalue on its left, so a prvalue object is first materialized as a temporary. The by-value parameter is then initialized from that temporary as an xvalue, which needs the move constructor, and the call is ill-formed when the move constructor is deleted. A later core defect resolution removes the materialization for this case, and the newest compilers follow it. |
| Q-0206 | The partial specializations are not used for the enclosing specialization. | supports | When a program gives an explicit specialization of the main member template for one specialization of the outer class, the draft drops the member's partial specializations for that outer specialization. |
| Q-0261 | The constraints come first, and a failure stops the substitution. | supports | During deduction, the draft checks constraints after the template parameter list is substituted. It checks them before it substitutes into the function type. A failed constraint ends deduction, so the return type is never formed. |
| Q-0270 | Valid. | supports | The grammar of a concept definition has an optional attribute list right after the concept name. |
| Q-0271 | The expansion is not valid for an alias template, as every compiler rejects it. | open | The draft has no rule that forbids a pack expansion in the arguments of an alias template when the matching parameter is not a pack. The draft also has no rule that allows the expansion. The alias text says only that a use is equal to the substituted type, and that does not settle when substitution happens. |
| Q-0288 | Access is checked from the place of the check, so the member can use the private member. | supports | The requires expression sits inside a member of the class, so normal access rules let it name a private member. The requirement holds, so the result is true. |
| Q-0289 | An error, as for any atomic constraint that is not a constant expression. | supports | Local parameters of a requires expression have no storage or lifetime. Reading a member of one is not a constant expression, and an atomic constraint must be a constant expression of type bool, so the program is ill-formed. The C++23 draft no longer has a separate rule that limits local parameters to unevaluated use. |
| Q-0291 | Not valid. | supports | A note in the explicit instantiation text says the instantiated entity must satisfy its constraints. The normative basis is thin, since the matching of the declaration to a constrained member is only implied. |
| Q-0300 | No reverse step; the program is rejected. | contradicts | The draft has two fallback steps for a new expression. If a type with extended alignment finds no function, the alignment argument is dropped and lookup runs again. If a type with ordinary alignment finds no function, an alignment argument is added after the size and lookup runs again. The class in the test should therefore get its aligned allocation function, and the program is valid. |
| Q-0308 | The program is invalid, and an implementation rejects it (STG-1). | supports | The draft makes it ill-formed to form any type whose object representation is larger than the largest size_t value. Classes are included, so a compiler should diagnose the class definition itself, even if the program never takes its size. |

### Q-0300 in more detail

- The draft describes the fallback in both directions in one list. The
  reverse step applies only to a type of ordinary alignment, after the
  first lookup finds nothing.
- Clang 22, GCC 16, and EDG 7.0 all reject the program. They share a
  defect, or the test has a confound.
- The confound to check: the test declares its own alignment tag type
  in namespace std. A compiler may not treat that type as the real tag
  type. The error messages say that no function matched, which points
  to a missing retry, but a writer can probe the case with the real
  header to rule this out.
- Q-0300 suggests that the inventory may describe a later version of
  the language. The reverse step is already in the C++23 draft, so it
  belongs to C++23.

### Q-0181 in more detail

- The C++23 draft has three relevant points. Member access with a dot
  requires a glvalue object. A prvalue in a glvalue context is
  materialized. The implied object argument of an explicit object
  member function initializes its first parameter like any other
  argument.
- Together these give a copy or move from a temporary into the
  parameter, which is option 2 of the question.
- The newest compilers follow a later defect resolution, so the
  settled answer matches current practice. The question should record
  that the answer relies on that resolution, and that the C++23 text
  alone gives option 2.

## Questions that stay open

| Question | Provisional answer (Tidal) | Option the reference supports | Note |
|---|---|---|---|
| Q-0044 | After the iteration expression (option 1). | option 1 | The draft defines the for loop by rewriting it as a while loop whose body holds the statement and then the iteration expression. The condition variable lives through one pass of that body, so it is destroyed after the iteration expression. The EDG order during constant evaluation, with destruction before the body, conflicts with the reference. |
| Q-0297 | One object for each specialization in the program (option 1). | option 1 | The internal linkage rule for const variables covers only variables that are not templates. A note adds that a const instance of a variable template can have external linkage. |
| Q-0319 | The copy into self takes an unspecified place among the argument evaluations (option 1). | option 1 | The draft treats the implied object argument as the first argument of an explicit object member function. The draft sequences each parameter's initialization indeterminately with the others. Only the evaluation of the object expression itself must come first. |
| Q-0243 | The qualifier can be dependent (option 1). | option 2 | The draft says that a nested name specifier before a deduced class template name must not be dependent. This applies with or without the template keyword. |
| Q-0257 | The program is not valid, and the implementation rejects it (option 1). | option 2 | The draft adds the end of the translation unit as a point of instantiation. It makes the program ill-formed with no diagnostic required when two points give different meanings. A related rule covers dependent calls whose result would change if lookup saw more declarations. No rule requires a diagnostic. A rejection is allowed, but the draft does not require one. |
| Q-0275 | Deduction uses the class template that the parameter names after substitution (option 1). | option 2, weakly | The draft allows class template argument deduction only for a class template or for an alias template whose type names such a template in turn. A template template parameter is neither of these inside the template definition, and the alias form has the same problem. The draft does not mention template template parameters directly, so this reading is a strict one. |
| Q-0305 | Only arrays of unsigned char and std::byte host objects (option 1). | option 1 | The draft names only arrays of unsigned char and arrays of std::byte as arrays that provide storage for a nested object. Plain char is not in the list. |
