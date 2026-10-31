<!--
Behaviour inventory, 2026-10-31: implicit conversion sequences (roadmap
item L, decision 0044).
Written by: a source reader under AGENTS.md that read the C++ working
draft sources and Tidal's own pages. It did not write or change any file
in this repository; the integrator copied this summary here. The summary
reached the integrator as a subagent message before the similarity
check ran.
Checked before use: tools/similarity.py against the draft sources. Two
passages were flagged and went back to the source reader, who reworded
them; the second check gives 0 flagged. No code, no quotes, no section
names. Each item is a behaviour to probe, not a rule.
-->

# Behaviour inventory: implicit conversion sequences

Role: source reader

Scope read: the draft's text on implicit conversion sequences and their three forms, reference binding inside sequences, list-initialization sequences, the candidate sets used when a user-defined conversion initializes an object or reference, implicit object parameters, and the ranking rules. Tidal pages read: spec/foundations/conversions.md, spec/declarations/initialization/*.md, spec/classes/conversion-functions.md, spec/classes/initializer-lists.md, spec/classes/constructors.md (converting constructor rules). Tidal has no page on overload resolution yet. No Tidal page states any ranking rule.

## 1. Categories (axes for the probe grid)

Arithmetic sources and targets
- bool as source
- a character type (plain, signed, unsigned, and the wide and UTF types)
- an enumeration that is scoped
- an unscoped enumeration with a fixed underlying type narrower than int
- an unscoped enumeration with a fixed underlying type of int or wider
- an unscoped enumeration without a fixed underlying type
- an extended floating-point type whose conversion rank equals a standard floating type's rank
- an extended floating-point type whose rank equals no standard type's rank
- float, double, long double (the narrower-to-wider pair float to double is special)
- an integer bit-field narrower than int, and one wider than int
- an integer type narrower than int (signed and unsigned short, the char types)
- an integer type of int rank or wider, signed and unsigned

Class sources and targets
- a class with a conversion function to a class type, to a reference type, to a scalar type
- a class with a conversion function template
- a class with a converting constructor taking one argument
- a class with a converting constructor taking several arguments (reached only by a braced list)
- a class with an explicit constructor or explicit conversion function
- a class with an initializer-list constructor, with and without a default constructor
- an aggregate class, with members in one order and another aggregate with the same members in another order
- the same class as the target, with and without cv-qualifiers
- a class derived from the target, directly, indirectly, ambiguously, through a private base
- a three-level chain of classes (base, middle, most derived) for the tie-breakers

Pointer and member-pointer kinds
- a null pointer constant (zero literal) and the null pointer value type
- a pointer to a function, with and without the no-throw marker
- a pointer to a member of a base, of a middle class, of a most derived class
- a pointer to an object of derived, middle, base class
- a pointer to void
- a pointer to cv-qualified object, at one level and at several levels
- a pointer to an array of known bound and of unknown bound

Reference targets
- an lvalue reference to const (non-volatile) type
- an lvalue reference to non-const or volatile type
- an rvalue reference to object type
- an lvalue reference and an rvalue reference to function type
- a reference to an array of known bound and to an array of unknown bound
- a reference to a type more cv-qualified than another reference target of the same type

Value categories of the source
- an lvalue, an xvalue, a prvalue
- a function lvalue (a function name)
- an array lvalue
- a bit-field lvalue

Braced-list sources
- an empty braced list
- a braced list with one element, where the element is itself a braced list
- a braced list with one element of the target class or a derived class
- a braced list with one element that is a string literal, for a character-array reference target
- a braced list with several elements of mixed types
- a designated braced list

Special parameter kinds
- an ellipsis parameter
- an explicit object parameter
- the implicit object parameter of a member function: without ref-qualifier, with lvalue ref-qualifier, with rvalue ref-qualifier, cv-qualified
- the object slot of a static member function that is a candidate in a member call
- the left operand of a built-in assignment candidate
- a parameter of initializer-list type for some element type
- a parameter that is an array reference (known or unknown bound)

## 2. Behaviours

A. Kinds of sequence and what each may hold

- A1. Ambiguous sequence: when more than one distinct way exists to convert an argument to a parameter, the parameter gets a single special sequence that counts as user-defined and ties with every other user-defined sequence.
- A2. Ambiguous sequence, consequence: a candidate with an ambiguous sequence stays viable, and the call fails only if that candidate wins.
- A3. Ambiguous sequence, effect on other candidates: an ambiguous sequence can still lose to, or tie with, another candidate's sequence, so a third overload with a better sequence makes the call valid.
- A4. Ellipsis sequence: an argument that matches the ellipsis of a variadic function has an ellipsis sequence.
- A5. Identity sequence: when the argument already has the parameter's type, the sequence is the identity, which is a standard sequence of exact-match rank.
- A6. Kinds: every valid implicit conversion sequence is one of three kinds, standard, user-defined, or ellipsis.
- A7. Properties ignored: forming a sequence looks only at type, cv-qualification and value category; lifetime, storage, alignment, access, bit-field status and deletion of the chosen function do not affect it, so a sequence can exist while the final initialization is still invalid.
- A8. Standard sequence, content: a standard sequence is either the identity alone or one to three steps, at most one from each of three groups (lvalue transformation; promotion or conversion; qualification adjustment), in that fixed order.
- A9. Standard sequence, lvalue transformations: the lvalue-to-rvalue step, array-to-pointer decay and function-to-pointer decay make up the first group.
- A10. Standard sequence, qualification adjustments: adding cv-qualifiers through pointers, and dropping the no-throw marker from a function pointer type, make up the last group.
- A11. Standard sequence, promotions: the integer promotions and the single floating promotion from float to double make up the promotion group.
- A12. Standard sequence, conversions: integer-to-integer, floating-to-floating (other than the promotion), floating-integer either way, pointer conversions, member-pointer conversions and conversions to bool make up the conversion group.
- A13. Top-level cv-qualifiers: a difference in top-level cv-qualification between a non-reference parameter and the argument is not a step in the sequence.
- A14. User-defined sequence, content: a user-defined sequence is a first standard sequence, then exactly one constructor or conversion function, then a second standard sequence.
- A15. User-defined sequence, first part with a constructor: the first standard sequence converts the source to the constructor's first parameter type.
- A16. User-defined sequence, first part with a conversion function: the first standard sequence converts the source to the conversion function's object parameter.
- A17. User-defined sequence, reference binding: when the target is a reference, the binding belongs to the second standard sequence.
- A18. User-defined sequence, template conversion function: when the conversion function is a specialization of a template, the second standard sequence must be of exact-match rank, or that conversion is not usable.

B. Rank of a standard sequence

- B1. Derived-to-base for class values: converting a class value to a non-reference parameter of a base class type is described as a derived-to-base step of conversion rank, though no such standard conversion exists outside this description.
- B2. Derived-to-base for reference binding: a direct binding of a base reference to a derived object counts as a derived-to-base step of conversion rank.
- B3. Rank table: lvalue transformations, qualification adjustments and the identity have exact-match rank; promotions have promotion rank; everything in the conversion group has conversion rank.
- B4. Rank of a whole sequence: the worst rank among its steps and its reference binding decides the rank of the sequence.
- B5. Same-class copy: copying a class value to a parameter of the same class (any cv-qualification) has exact-match rank, even though a constructor runs.
- B6. Static member function object slot: a static member function, as a candidate, has an object slot that accepts any object. The sequence for that slot counts as standard and ties with every other standard sequence.

C. Reference binding inside a sequence

- C1. Bit-field argument: a bit-field lvalue still forms an exact-match sequence for a non-const lvalue reference parameter; the call fails only if that function is chosen.
- C2. Direct binding to the argument: when a reference parameter binds directly to the argument, the sequence is the identity, or a derived-to-base step if the argument's class derives from the referenced class.
- C3. Direct binding to a conversion result: when a reference binds directly to what a conversion function returns, the sequence is user-defined with a second part that is the identity or a derived-to-base step.
- C4. Indirect binding: when a reference binds to a temporary, the sequence is the one that converts the argument to the referenced type, as if copy-initializing a temporary of that type, with top-level cv differences ignored.
- C5. Lvalue reference to rvalue: no sequence exists that binds a non-const or volatile lvalue reference to an rvalue (the implicit object parameter is an exception).
- C6. Rvalue reference to lvalue: no sequence exists that binds an rvalue reference to an object lvalue; an rvalue reference to a function type may bind to a function lvalue.
- C7. Implicit object parameter, rvalue argument: for a member function without ref-qualifier, an rvalue object can bind to the implicit object parameter even when the member function is not const, and that fact does not change the ranking.
- C8. Implicit object parameter, type: for a member function without ref-qualifier or with the lvalue ref-qualifier, the parameter is an lvalue reference to the class with the function's cv-qualifiers; with the rvalue ref-qualifier it is an rvalue reference.
- C9. Implicit object parameter of a conversion function: the class used for the parameter type is the class of the object expression, not the class that declares the function.
- C10. Implicit object parameter of a member brought in by a using-declaration: for non-conversion members, the parameter type uses the derived class that holds the using-declaration.

D. When user-defined conversions are allowed

- D1. Assignment left operand: the left operand of a built-in assignment candidate accepts only standard sequences.
- D2. Explicit object parameter: an explicit object parameter accepts user-defined sequences, unlike the implicit object parameter.
- D3. Implicit object parameter: the implicit object parameter accepts only standard sequences, in every context.
- D4. Nested user-defined conversions in class copy-initialization: in the second step of copy-initializing a class from another type, the copy or move constructor's parameter does not accept a user-defined sequence, so a chain of two user-defined conversions never forms.
- D5. Nested user-defined conversions in conversion-function selection: when choosing a conversion function or a converting constructor for a user-defined conversion, the source-to-parameter step and the object parameter step accept only standard sequences.
- D6. Nested user-defined conversions with double braces: when a class is list-initialized from a list whose only element is itself a braced list, and the constructor's first parameter is that same class or a reference to it, the inner list does not get a user-defined sequence.
- D7. One conversion: outside list-initialization, an implicit conversion sequence holds at most one user-defined conversion.
- D8. List elements: inside a list-initialization sequence, each element can use its own user-defined conversion to reach a constructor parameter, so several user-defined conversions can occur for one argument.
- D9. Initializer-list elements: the conversion of each element to the initializer-list element type can be user-defined, also when the call is to an initializer-list constructor.

E. Candidate sets for a user-defined conversion

- E1. Class target, copy-initialization: the candidates are the converting constructors of the target and, when the source is a class, its non-explicit conversion functions whose result is the target class or a class derived from it.
- E2. Class target, direct-initialization through a copy or move constructor: when a single argument direct-initializes a class and the temporary for the constructor's reference parameter is being made, explicit conversion functions to the class or a derived class are also candidates.
- E3. Non-class target: the candidates are non-explicit conversion functions whose result reaches the target by a standard sequence; in direct-initialization, explicit ones whose result reaches the target by at most a qualification conversion are added.
- E4. Reference target, direct binding: the candidates are conversion functions returning a reference (lvalue reference for an lvalue reference target; prvalue or rvalue reference for an rvalue reference target) whose result type the target is reference-compatible with; in direct-initialization explicit ones that need at most a qualification conversion are added.
- E5. Self, base and void: conversion functions to the class itself, a base of it, or void are never used by an implicit conversion (Tidal already states this; see section 4).

F. List-initialization sequences

- F1. Aggregate from a list: when the parameter is an aggregate that can be aggregate-initialized from the list, the sequence is user-defined with an identity second part.
- F2. Aggregate from one element of its own class: when the parameter is an aggregate class and the list holds one element of that class or a derived class, the sequence is the one that converts that element.
- F3. Array parameter: for a reference to an array of known bound N or of unknown bound, the sequence exists if every element converts to the element type (and, for known bound larger than the list, an empty braced list also converts), and the sequence is the worst of those.
- F4. Character array from a string literal: for a reference to a character array and a list holding one suitable string literal, the sequence is the identity.
- F5. Designated list: a designated list converts only to an aggregate that it can initialize, giving a user-defined sequence with an identity second part.
- F6. Designated list, member order: forming the sequence ignores whether the designators follow declaration order, so a mismatched order makes the chosen call fail rather than removing the candidate.
- F7. Designated list, two aggregates: two aggregate parameters that both accept the same designated list give indistinguishable sequences.
- F8. Initializer-list parameter: when the parameter is an initializer list of some element type and every element converts, the sequence is the worst element conversion, or the identity for an empty list.
- F9. Narrowing: a narrowing element conversion does not stop the sequence from forming; it makes the call invalid once that candidate is chosen.
- F10. No other case: a braced list that matches none of the listed cases has no sequence.
- F11. Non-aggregate class, constructor chosen: when constructor selection for the list picks one best constructor, the sequence is user-defined with an identity second part.
- F12. Non-aggregate class, constructor tie: when constructor selection for the list finds several viable constructors with no best one, the sequence is the ambiguous sequence.
- F13. Non-aggregate class, one element of the same class: when a constructor that is not an initializer-list constructor is chosen and the list holds one element of the target class (or a derived class), the sequence is exact match (or conversion rank for a derived class) instead of user-defined.
- F14. Non-class, empty list: an empty list for a non-class parameter has the identity sequence.
- F15. Non-class, one element: a list with one non-list element for a non-class parameter has the sequence of that element.
- F16. Non-class, one element that is a list: a list whose one element is itself a braced list has no sequence to a non-class parameter.
- F17. Reference parameter: for a reference parameter not covered above, the list initializes a temporary of the referenced type, and the sequence follows the rules for that temporary.
- F18. Two-phase constructor selection: the list sequence for a non-aggregate class uses the two-phase selection (initializer-list constructors first unless the list is empty and a default constructor exists, then all constructors with the elements as arguments).
- F19. Explicit constructor in copy-list context: an explicit constructor stays a candidate in copy-list-initialization, and choosing it makes the program invalid only if the choice is part of the final result.

G. When no sequence exists

- G1. General: if no chain of allowed steps converts the argument to the parameter, no sequence exists and the candidate is not viable.
- G2. Reference value-category mismatch: see C5 and C6.
- G3. Second user-defined conversion: a sequence that would need two user-defined conversions does not exist (except the list cases of D8 and D9).
- G4. Template conversion function with non-exact second part: see A18.

H. Comparing sequences of different kinds

- H1. Standard beats user-defined and ellipsis: a standard sequence is better than any user-defined or ellipsis sequence.
- H2. User-defined beats ellipsis: a user-defined sequence is better than an ellipsis sequence.
- H3. Indistinguishable default: two sequences of the same kind tie unless one of the rules in groups I to L applies.

I. Comparing list-initialization sequences (these win even when later rules would decide otherwise)

- I1. Array bound count: between two list sequences to arrays of the same element type, the one that initializes fewer elements is better.
- I2. Array known bound over unknown bound: between two list sequences to arrays of the same element type that initialize the same number of elements, the one to a known bound is better.
- I3. Initializer-list target first: a list sequence to an initializer-list type beats a list sequence to any other type, even when the other one has a better element conversion.

J. Comparing two standard sequences

- J1. Function lvalue: when both sequences bind a reference to a function lvalue, binding an lvalue reference is better than binding an rvalue reference.
- J2. More cv-qualified referenced type loses: when both sequences bind references to the same type except for top-level cv-qualifiers, the one whose referenced type has fewer cv-qualifiers is better.
- J3. Qualification conversion: when the two sequences differ only in their qualification conversion and the result of one can still be qualification-converted to the result of the other, the one that adds fewer qualifiers is better.
- J4. Rank: a sequence of better rank wins; exact match beats promotion, promotion beats conversion.
- J5. Rvalue reference to rvalue: when both sequences bind references, the one binding an rvalue reference to an rvalue beats the one binding an lvalue reference.
- J6. Rvalue reference exception: J5 does not apply when either side is the implicit object parameter of a member function declared without a ref-qualifier.
- J7. Subsequence: a sequence that is a proper subsequence of the other wins, comparing after removing lvalue transformations; the identity counts as a subsequence of any non-identity sequence.
- J8. Subsequence order: the subsequence rule is checked before rank.

K. Tie-breakers among standard sequences of equal rank

- K1. Bool conversion of pointers: a conversion that does not turn a pointer or a member pointer into bool beats one that does.
- K2. Derived pointer to base pointer over void pointer: converting a pointer to a derived class into a pointer to its base beats converting it into a pointer to void.
- K3. Base pointer to void over derived pointer to void: converting a base pointer to void beats converting a derived pointer to void (for comparing second parts of user-defined sequences).
- K4. Enumeration with fixed underlying type: promoting such an enumeration to its underlying type beats promoting it to the promoted form of that type, when the two differ.
- K5. Floating types of equal rank: a conversion between two floating types of equal conversion rank beats a conversion from the same source to an integer type, to a floating type of different rank, or to a floating type of equal rank but lower subrank.
- K6. Nearest base, class value: converting a most derived class value to the middle class beats converting it to the base.
- K7. Nearest base, member pointer: converting a member pointer of the base to one of the middle class beats converting it to one of the most derived class.
- K8. Nearest base, pointer: converting a most derived pointer to a middle-class pointer beats converting it to a base pointer.
- K9. Nearest base, reference: binding a most derived object to a middle-class reference beats binding it to a base reference.
- K10. Nearest source (second parts only): converting a middle-class pointer, value, or reference binding to the base beats doing so from the most derived class; converting a middle-class member pointer to a most-derived member pointer beats doing so from the base.
- K11. Source types differ only in one context: two compared sequences have different source types only when comparing the second parts of user-defined conversions during a user-defined-conversion initialization.

L. Comparing two user-defined sequences

- L1. Different functions tie: two user-defined sequences that use different constructors or conversion functions tie, whatever their second parts.
- L2. Same aggregate class: two user-defined sequences that both aggregate-initialize the same class compare by their second parts.
- L3. Same function: two user-defined sequences that use the same constructor or conversion function compare by their second standard sequences.
- L4. Choosing the conversion function: when selecting which user-defined conversion to use, the second standard sequence from each function's result to the target breaks ties between conversion functions.

## 3. Gaps and doubts

- Ambiguous sequence and viability: some compilers have treated an ambiguous user-defined conversion as making the candidate non-viable, or report ambiguity early, instead of keeping the candidate and failing only if it wins. Probe A2 and A3.
- Array bound list rules (I1, I2, F3 with unknown bound): recent addition; older compilers lack conversion of a braced list to a reference to an array of unknown bound and the related tie-breakers.
- Bit-field to non-const lvalue reference (C1): compilers may drop the candidate instead of choosing it and then failing. Probe with a competing overload that takes the value.
- Designated list ambiguity (F6, F7): GCC and Clang may check designator order during viability, which changes which overload wins.
- Double-brace suppression (D6): compilers have differed on whether the inner braced list may use a conversion function of the element's class.
- Explicit conversion functions in direct-initialization of a class through the copy constructor (E2): recent rule; Clang and GCC differ on whether an explicit conversion function of the source is usable there.
- Extended floating-point tie-breaker (K5): depends on which extended floating types the implementation provides; Clang lacks the fixed-width floating types on many targets. The behaviour with only standard types (float, double, long double) needs separate probes.
- Fixed-underlying enumeration tie-breaker (K4): older compilers report ambiguity between the underlying type and int.
- Implicit object parameter rvalue-reference exception (J6): probe mixed member (no ref-qualifier) and non-member operator candidates; compilers have differed.
- Narrowing during overload resolution (F9): compilers may differ on whether a narrowing element removes a candidate in the list path; the source treats narrowing as a check after the choice.
- Base that is ambiguous or not accessible: the source says access does not affect forming the sequence; whether an ambiguous base removes the candidate or makes the chosen call fail is less clear, and compilers may differ.
- Null pointer value type to bool: this conversion exists only in direct-initialization; its rank and how it compares with the K1 tie-breaker are not stated directly.
- One-element list of the same class (F13, F2): Tidal already has Q-0081 for a related case; compilers differ when the class has an initializer-list constructor that could take the element.
- Qualification tie-breaker (J3) with multi-level pointers and pointers to arrays of unknown bound: the rule uses "similar types", which recent changes widened; older compilers may disagree.
- Static member function object parameter (B6): when a static and a non-static member overload compete, the static one's object sequence never decides; compilers have differed in older versions.
- Template conversion function (A18): probe whether compilers reject the template, or fall back to another conversion, when the second part would need a promotion or conversion.
- User-defined conversion by aggregate initialization compared with one by constructor (L2 with L1): the result when one parameter is an aggregate and another a class with a constructor taking the same list is not spelled out beyond "different functions tie".

## 4. Already covered in Tidal

Existence of individual standard conversions (not their rank):
- CONV-1: integer promotions, bit-field by width, enumeration promotion with and without fixed underlying type (supports A11, K4 inputs).
- CONV-8: null pointer constants.
- CONV-9, CONV-10, CONV-14, CONV-15, CONV-16, CONV-17, CONV-18: which pointer, member-pointer, function-pointer and array-pointer conversions exist (A10, A12 inputs).
- CONV-12: null pointer value to bool only in direct-initialization.
- CONV-13: array and function decay (A9).
- CONV-20: contexts that perform implicit conversion (A6 context only).
- INIT-4: states that initialization uses implicit conversion and marks conversion sequences as "referenced", so A5 to A18 are not stated.

User-defined conversions:
- CTOR-9, CVF-2, CVF-3, CVF-11: explicit and non-explicit constructors and conversion functions, and where explicit ones are usable (partly E1 to E4; E2 not covered).
- CTOR-10: standard conversions may come before a converting constructor (A15, partly A14).
- CTOR-22, CVF-10: at most one user-defined conversion (D7, G3). D4, D5, D6, D8, D9 are not stated.
- CVF-4: conversion functions to self, base or void are not used (E5); cv and ref-qualifier selection on the object.
- CVF-12: a tie between a constructor and a conversion function is ambiguous (L1 in part; A1 to A3 not covered).

Reference binding:
- REF-1, REF-2, REF-3, REF-8, REF-9, REF-10, REF-13: which bindings are valid, direct versus temporary (inputs for C2, C4, C5, C6). C1, C3, C7 to C10 are not covered as sequence facts.

List-initialization:
- ILC-2, ILC-3: two-phase constructor selection and the empty-list default-constructor case (F18).
- ILC-4: narrowing in a chosen initializer-list constructor (F9 for constructors only).
- ILC-6: an initializer-list parameter accepts a braced list (part of F8).
- ILC-8: explicit constructor chosen in copy-list-initialization (F19).
- LIST-1, LIST-11: scalar from a braced list with zero or one element (F14, F15, F16 for initialization, not as sequences).
- LIST-2: narrowing table.
- LIST-6: character array from a string literal (F4 for initialization, not as a sequence).
- LIST-10: aggregate from one element of the same class (F2 for initialization).

Not covered at all: every ranking and comparison fact (B1 to B6, H1 to H3, I1 to I3, J1 to J8, K1 to K11, L1 to L4), the ambiguous sequence (A1 to A3), the ellipsis sequence (A4), the implicit and explicit object parameter rules (C7 to C10, D1 to D3, B6), and the list-sequence cases for arrays, designated lists and non-aggregate classes (F1, F3, F5 to F7, F10 to F13, F17).
