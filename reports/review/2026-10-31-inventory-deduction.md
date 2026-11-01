<!--
Behaviour inventory, 2026-10-31: deduction, function templates, partial
ordering (roadmap item O, decision 0044).
Written by: a source reader under AGENTS.md that read the C++ working
draft sources and Tidal's own pages. It did not write or change any file
in this repository; the integrator copied this summary here.
Checked before use: the source reader ran tools/similarity.py on its
summary outside the repository and reworded one flagged passage; the
integrator's own check gives 0 flagged. No code, no quotes, no section
names. Each item is a behaviour to probe, not a rule.
-->

Role: source reader

# Inventory for item O: deduction, function templates, partial ordering

## 1. Categories for a probe grid

Axis 1, where deduction runs:
- a call by name or by operator,
- a target type for the name of a function template, with or without the address operator,
- a conversion function template looked up for a target type,
- an explicit specialization, explicit instantiation, or friend declaration that names a specialization,
- matching a class or variable template partial specialization,
- partial ordering between two templates,
- a placeholder in a variable, a return type, or a constant template parameter.

Axis 2, the shape of the parameter type:
- a bare type parameter, with or without top-level qualifiers,
- a lvalue reference to a parameter, with or without added qualifiers,
- an rvalue reference to an unqualified parameter (a forwarding reference),
- an rvalue reference to a qualified parameter,
- a pointer, a pointer to member, an array with a deducible or fixed bound,
- a function type or function pointer type, including its exception flag,
- a class template specialization whose arguments hold parameters,
- a template template parameter applied to arguments,
- a qualified name whose qualifier holds a parameter,
- a decltype operand or a constant argument that is an expression of a parameter,
- a function parameter pack, last or not last,
- an initializer list class or an array reference, for a braced argument.

Axis 3, the kind of argument:
- an lvalue, an xvalue, a prvalue,
- an array, a function, a string literal,
- a value of a qualified type,
- a value of a class derived from a specialization, singly or with two candidate bases,
- a braced list, empty, uniform, mixed, or nested,
- the name of an overload set, with or without templates in it,
- an argument for a parameter that has a default and is left out.

Axis 4, what explicit arguments fix: none, the leading ones, all of them, an empty list, more than the non-pack parameters.

Axis 5, for ordering: the two templates differ by pointer, reference kind, qualifiers, class pattern, a pack, a default argument, an ellipsis, a member against a non-member, constraints.

## 2. Behaviours

### A. Adjustments before matching in a call

- A1 Argument decay. When the parameter is not a reference, an array argument counts as a pointer to its first element and a function argument counts as a pointer to that function.
- A2 Argument qualifiers. When the parameter is not a reference, top-level const and volatile on the argument type are dropped before matching.
- A3 Parameter qualifiers. Top-level const and volatile on the parameter type are ignored for matching.
- A4 Parameter reference. When the parameter is a reference, matching uses the referred type.
- A5 Qualifier on the deduced reference. With a lvalue reference parameter to a bare type parameter, a const argument makes the deduced type const.
- A6 Value category ignored. Apart from forwarding references, whether the argument is an lvalue or an rvalue does not change the deduced type.

### B. Allowed differences after matching

- B1 Base of a derived class. A parameter that is a class template pattern, or a pointer to one, accepts an argument of a class derived from a matching specialization.
- B2 Closest base wins. If one candidate base is itself derived from another candidate base, the more derived candidate is chosen.
- B3 Fallback only. The three tolerances of this group apply only when an exact match fails.
- B4 More qualifiers through a reference. With a reference parameter, the deduced referred type can carry more const or volatile than the argument.
- B5 Pointer conversions. A pointer or member pointer argument can differ from the deduced type by an added qualification or by dropping a no-throw flag.
- B6 Two answers fail. If a tolerance leads to two different results, for example two unrelated bases that both match, deduction fails.

### C. Braced list arguments

- C1 Array bound from list length. With a reference to an array whose bound is a constant parameter, the bound comes from the number of elements.
- C2 Array element type. The element type of such an array is deduced from each element, and a fixed bound is then not checked against the list length during deduction.
- C3 Element by element. A parameter that is the initializer list class of some pattern, after removing reference and qualifiers, deduces the pattern from each element on its own.
- C4 Elements must agree. Elements that give different types for the same parameter make deduction fail.
- C5 Empty list. An empty braced list deduces nothing from such a parameter.
- C6 Nested lists. Nested braced lists can deduce both bounds of a two-level array, and an inner list can initialize a class element.
- C7 Other parameter shapes. For any other parameter type, a braced list argument contributes nothing, so a bare type parameter cannot be deduced from a list.
- C8 Overload set element. An element that names an overload set can contribute nothing while other elements decide.

### D. Combining results

- D1 Agreement across pairs. Each parameter and argument pair deduces on its own, and every pair that gives a value for one template parameter must give the same value.
- D2 Array bound as last resort. A type parameter used for a bound gets a value from the bound only when nothing else gives one.
- D3 Default template argument. A template parameter left without a value takes its default, which may use earlier parameters.
- D4 Default function argument. The type of a default function argument never supplies a deduced type.
- D5 Missing value. If any template parameter has no value from deduction, explicit arguments, or a default, deduction fails.
- D6 Several values in one pair. One pair can fix several template parameters at once, for example from the parts of a function pointer type, and a conflict among them fails.

### E. Deducible shapes

- E1 Array bound through reference or pointer. A bound of an array reached through a reference or pointer is deducible.
- E2 Bound type. A constant parameter of dependent type deduced from an array bound gets the size type.
- E3 Class template arguments. Each argument of a class template pattern is compared with the matching argument of the argument type.
- E4 Constant type mismatch. A constant parameter deduced from a class template argument fails if its type differs from the type that class template declares for that position, with top-level qualifiers ignored.
- E5 Exception flag. The no-throw flag of a function type can deduce a bool constant parameter.
- E6 Function parameters and result. Parameter types and the result type of a function or function pointer type are all deducible positions.
- E7 Major array bound. The outermost bound of an array parameter written by value is lost by adjustment and cannot be deduced.
- E8 Member pointer parts. Both the class and the member type of a pointer to member are deducible.
- E9 Template template parameter. When a class template argument of the argument type names a template, that template becomes the value of a template template parameter.
- E10 Type of a constant from a value. When a constant parameter of dependent type is deduced, the type parameter in its type is deduced from the value's type.
- E11 Unrelated constructs. Anything outside the listed shapes gives no deduction.

### F. Explicit template arguments

- F1 Conversions on fixed parameters. A parameter whose type no longer holds a deducible parameter after explicit arguments accepts any implicit conversion.
- F2 Constructor templates. No syntax supplies explicit arguments to a constructor template.
- F3 Empty list selects templates. An empty explicit list makes the call consider only templates, so a plain function with the same name is skipped.
- F4 Invalid explicit list. If the explicit list does not form a valid template-id for the template, the template drops out.
- F5 Leading order. Explicit arguments go to template parameters in declaration order, and the rest come from deduction or defaults.
- F6 Pack extension. Deduction can add more elements to a pack after explicit elements.
- F7 Qualified explicit type. An explicit type with top-level const changes the parameter variable but not the function type.
- F8 Trailing pack empty. A trailing pack that gets nothing from the explicit list or deduction is empty.

### G. Forwarding references

- G1 Class template parameter. A rvalue reference to a parameter of the enclosing class template is not a forwarding reference, except in a deduction guide.
- G2 Lvalue argument. A forwarding reference with a lvalue argument deduces a lvalue reference type, which collapses to a lvalue reference parameter.
- G3 Qualified pattern. A rvalue reference to a const parameter is not a forwarding reference, so a lvalue argument fails to bind.
- G4 Rvalue argument. A forwarding reference with an rvalue argument deduces the plain type.
- G5 Through function types. When matching whole function types from a target or a declaration, a top-level forwarding reference parameter against a lvalue reference deduces a lvalue reference type.

### H. Non-deduced positions

- H1 Constant expression of a parameter. A constant template argument or array bound that is an expression involving a parameter, other than the parameter alone, is non-deduced.
- H2 Decltype operand. The operand of decltype is non-deduced.
- H3 Default argument in use. A function parameter whose default argument the call uses is non-deduced.
- H4 Mixed composite. A compound type can have one part non-deduced while another part of the same parameter list deduces the same parameter.
- H5 Non-deduced pack. A function parameter pack that is not last is non-deduced, so its types must come from explicit arguments.
- H6 Pack expansion not last in a class pattern. If a class template pattern has a pack expansion before its last argument, the whole argument list is non-deduced.
- H7 Qualifier part. The part before the last name in a qualified type is non-deduced, and every parameter inside it is too.
- H8 Unused in deduced positions. A parameter that appears only in non-deduced positions must be given explicitly or by default.

### I. Overload sets as arguments

- I1 Ambiguous set. If more than one member of a set of plain functions matches, the parameter is non-deduced.
- I2 Set with templates. A set that includes any function template makes the parameter non-deduced.
- I3 Single match. If exactly one member of a set of plain functions matches the parameter, deduction uses that member.
- I4 Rest of the call. A parameter left non-deduced by an overload set can still be resolved later from types that other arguments fixed.

### J. Packs

- J1 Class pattern pack. A pack expansion at the end of a class template pattern matches each remaining argument in turn.
- J2 Empty deduction. A pack can be deduced as empty.
- J3 Length mismatch. Expanding two packs of different lengths together fails deduction.
- J4 Trailing function pack. A function parameter pack at the end takes one argument per element, each matched against its pattern.
- J5 Pattern per element. Each element of a pack is deduced with the same rules as a single parameter, so decay and forwarding apply per element.

### K. Other deduction contexts

- K1 Address without a target. Without a target type, a template name deduces from empty type lists and needs explicit arguments for every parameter.
- K2 Address with a target. With a target type, the function type of the template is matched against the target's function type.
- K3 Conversion function result. A conversion function template deduces its result type from the type the context needs.
- K4 Conversion function adjustments. For a conversion function, a reference result counts as its referred type, an array or function result decays when the needed type is not a reference, and top-level qualifiers on both sides drop.
- K5 Conversion function tolerances. When an exact match fails, a conversion function can still match if the needed type differs only by qualifiers that a qualification step can add, by a dropped no-throw flag, or by qualifiers under a needed reference.
- K6 Declaration of a specialization. An explicit specialization, explicit instantiation, or friend declaration finds its template arguments by matching the declared function type against each template, then ordering.
- K7 Deduced return type at address. A placeholder return type is not used for deduction, and the body is instantiated to find it.
- K8 No unique match. If no template or more than one template fits such a declaration after ordering, the declaration is not valid.
- K9 Partial specialization match. A class or variable partial specialization matches when its parameters can be deduced from the given argument list and its constraints hold.

### L. Partial ordering, the comparison

- L1 Call context types. In a call, only the parameters with explicit call arguments take part, so packs not used, defaulted parameters, and an ellipsis are ignored.
- L2 Conversion context types. For conversion functions, only the result types take part.
- L3 Each direction. Each template is turned into a version with invented unique types, values, and templates, and its types are matched against the other template's original types.
- L4 Member functions. A non-static member template gets an extra first parameter, a reference to its class with its qualifiers, lvalue or rvalue by its ref-qualifier or by the other template's matching parameter.
- L5 Other contexts. For an address or a declaration of a specialization, the whole function type takes part.
- L6 Reference and qualifier stripping. Before matching, references and then top-level qualifiers are removed from both types, after noting which side was the more qualified reference.
- L7 Reversed candidate. If only one template was found by a rewritten operator with swapped operands, its parameter order is reversed for ordering.
- L8 Unused parameters may stay open. A template parameter that appears in none of the compared types need not get a value.

### M. Partial ordering, the verdict and tie-breakers

- M1 At least as specialized. One template is at least as specialized as another if every compared pair deduces in that direction.
- M2 Constraints decide a tie. If both directions succeed, the more constrained template wins, but only when both have equivalent template parameter lists and the same function parameter types position by position.
- M3 Conversion result types differ. In a conversion context, if the result types are not the same, constraints do not decide.
- M4 Lengths differ. If the template parameter lists or function parameter lists differ in length, constraints do not decide.
- M5 Lvalue over rvalue reference. When both compared types were references and are otherwise equal, a lvalue reference parameter is more specialized than a rvalue reference parameter.
- M6 More qualified reference. When both compared types were references and equal after stripping, the side with more qualifiers is more specialized.
- M7 Non-variadic over variadic. When each is at least as specialized as the other and only one has a trailing function pack with no matching parameter on the other side, the one without the pack wins.
- M8 Pack against non-pack. A pack on the argument side matched against a single parameter fails, and a pack with nothing opposite it is ignored.
- M9 Pack count. Ordering does not depend on how many elements a pack received in the call.
- M10 Reversed candidate with constraints. For a swapped rewritten candidate, constraints decide only if every template parameter is deducible and exactly one reordering of the parameters lines both lists up.
- M11 Strict order. One template is more specialized when it is at least as specialized and the other is not.

### N. Partial specializations, the order

- N1 Function rewrite. Two class or variable partial specializations compare by turning each into a function template with its own parameters and constraints and one parameter of the class type with its argument pattern.
- N2 Placeholder constant parameters. Partial specializations whose constant parameters use placeholders of different pointer depth order like the matching function templates.
- N3 Primary as a rival. A partial specialization must be more specialized than the primary template.
- N4 Repeated parameter. A pattern that repeats one parameter in two places is more specialized than one using two separate parameters there.

### P. Placeholders that use deduction

- P1 Copy list initialization. With an equals sign and a braced list, the placeholder becomes the initializer list class of a deduced element type.
- P2 Decltype placeholder. The decltype form does not use template argument deduction.
- P3 Invented parameter. The placeholder is replaced by an invented type parameter, and deduction runs as for a call with the initializer as the argument.
- P4 Return statements. Each return statement deduces separately, and all must agree, and a braced list in a return statement is not valid.

### Q. Substitution and its failures

- Q1 Access checks. Access checking is part of substitution, so an inaccessible member in the declaration removes the template.
- Q2 Constraints after deduction. Constraints are checked after all arguments are known, and unsatisfied constraints remove the template.
- Q3 Early check of plain parameters. After the template parameter list is substituted, each parameter whose type was never dependent is checked against its argument, and a failed conversion removes the template before the function type is substituted.
- Q4 Explicit then deduced. Explicit arguments are substituted first, then the function type is adjusted as for any function, and at the end the deduced and default values are substituted.
- Q5 Immediate context only. An error that arises inside the instantiation of another class or function, or in a generated special member, is a hard error.
- Q6 Lambda bodies. A lambda in the signature or in a template parameter is outside the immediate context, so an error in its body or captures is a hard error.
- Q7 Lexical order. Substitution goes in source order and stops at the first failure, so a later invalid part is never reached.
- Q8 Narrowing constant. An explicit constant argument that narrows for one template removes that template.
- Q9 No-throw clause. The no-throw clause is not substituted during deduction.
- Q10 Removed candidate. A template whose deduction or substitution fails adds no candidate and causes no error.
- Q11 Typical failures. Failures include a qualified name on a non-class, a missing or wrong-kind member, an array of void or of references or of size zero, a pointer to a reference, a reference to void, a member pointer into a non-class, a parameter of type void, a function returning an array or function, a constant parameter given an invalid type or value.
- Q12 Where substitution applies. Substitution applies to the function type apart from the no-throw clause, the explicit condition, and the template parameter declarations.

## 3. Gaps and doubts

- Array bound of a fixed array reference with a longer braced list. The bound is not used in deduction, so the list size mismatch shows only at initialization. Compilers may differ on whether the template is removed or the call is an error.
- Base selection. The rule that prefers the most derived candidate base is recent. Older compilers may report ambiguity.
- Class pattern pack not last. Compilers may differ on whether such a pattern is silently non-deduced or rejected.
- Default argument order. Compilers may differ on when a default template argument that fails substitution removes the template versus errors.
- Different declarations order. Two declarations of one template whose substitution order differs make the program invalid without a diagnostic, so probes should avoid this.
- Lambda in signature. Generic lambda call inside a decltype in the signature can still fail softly in some cases. Compiler support varies.
- Member against non-member ordering. The choice of lvalue or rvalue reference for the invented object parameter has a recursion clause. Compilers may differ.
- Overload set inside a braced list. Compiler support for skipping one element is uneven.
- Partial ordering with defaults. Templates that differ only in a defaulted trailing parameter can be ambiguous. Compilers have differed here historically.
- Partial ordering with explicit arguments. Explicit arguments do not enter partial ordering. Some compilers have let them.
- Pack with explicit prefix and a non-trailing function pack. Compilers differ on how many elements explicit arguments fix.
- Placeholder constant parameters in partial specializations. Ordering by pointer depth is newer and may be missing in some compilers.
- Reversed rewritten candidates. The reordering rule for constraints is new and support is likely uneven.
- Substitution in the no-throw clause. Probes should check that an invalid no-throw clause is an error only when instantiated.

## 4. Already covered in Tidal

- AUTO-1, AUTO-2: adjustments for a plain placeholder, matching A1 and A2.
- AUTO-3, AUTO-4: reference placeholders, the variable side of A5 and G2, G4.
- AUTO-7, AUTO-21: copy list initialization with a placeholder, P1 and C4.
- AUTO-10: the decltype placeholder, P2.
- LAM-15: a parameter with a placeholder makes a call operator template that deduces per call.
- OVL-1: a template whose deduction fails adds no candidate, Q10 in part.
- OVL-16: deduction comes before checking a dependent explicit condition.
- OVL-19, OVL-20: address of an overload set with templates, K1 and K2 in outline.
- OVL-22: viable function count for a template with a pack.
- OVL-27: a plain function wins a tie over a specialization.
- OVL-28: the more specialized template wins a tie, without the steps of L and M.
- OVL-39: an overload set as a call argument, in outline only.
- OVL-41: constraints decide only between templates with the same function parameters, M2 in part.
- TARG-6, TARG-20, TARG-21: explicit arguments for leading parameters, too many arguments, a pack takes the rest, F5 and F8 in part.
- TPL-11: a placeholder for a constant parameter.
- TPL-22: function templates and plain functions share a name.
- TSP-2, TSP-4: arguments of an explicit specialization can be left out when deduction finds them, K6 in outline.
- TSP-11: a partial specialization must deduce each parameter and be more specialized than the primary template, N3 in part.
- TSP-14, TSP-15, TSP-16: matching and selecting partial specializations, K9 and the selection side of N.
