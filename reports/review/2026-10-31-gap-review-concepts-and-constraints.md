<!--
Gap review summary, 2026-10-31: concepts and constraints (item P,
decision 0044).
Written by: a source reader under AGENTS.md that read the C++ working
draft sources and Tidal's pages concepts.md and constraints.md. It did
not write or change any file in this repository; the integrator copied
this summary here.
Checked before use: the source reader ran tools/similarity.py on its
summary outside the repository; the integrator's own check gives 0
flagged. No code, no quotes, no section names. Each item ends with the
outcome a probe should show; it is a suggestion to probe, not a rule.
Labels: M = missing, E = possible error, O = order or condition,
S = structure.
-->

Role: source reader

1. E (CPT-21, CPT-22): A constrained placeholder on a constant template parameter adds a constraint to the template itself, in the same way a type constraint on a type parameter does. For a function template called with an explicit constant argument whose type fails the concept, the candidate drops out quietly and another overload can win. CPT-22 treats the failure as an error in every case. Probe: a function template with a constant parameter constrained to wide types, plus an unconstrained fallback overload; call with a character argument. Expected: the fallback is chosen, no error.

2. M (CNS-23, ORD-13): Because the placeholder constraint of item 1 belongs to the template, a function template with a constrained constant parameter is more constrained than an otherwise identical template whose constant parameter is unconstrained. Probe: two such templates, call with an int argument that both accept. Expected: the constrained one is chosen, no ambiguity.

3. O (CNS-1): CNS-1 does not say where the constraint of a constrained constant parameter sits in the conjunction. My reading puts it among the template parameter constraints, in the order the parameters appear, before the requires clause after the head. Probe: a constant parameter whose constraint fails, followed by a type parameter whose constraint instantiates a class with a failing static assertion, and a fallback overload. Expected: the fallback is chosen and the trap is never instantiated.

4. E (CNS-25): The second sentence of CNS-25 says that an argument with constraints the parameter lacks is not valid. When the template template parameter has no constraints at all, the argument's constraints are ignored and the argument is accepted. The test with the plain parameter already shows this and contradicts the rule text. Probe: the existing accept test. Expected: accept on every compiler.

5. M (CNS-13, CNS-14): A template template parameter can carry its own constraints. Inside the template, a use of that parameter with non-dependent arguments that fail the parameter's constraints makes the program invalid with no diagnostic required. After substitution, the argument template's own constraints also apply to each use. Probe: a template template parameter constrained to wide types, used with a character argument in the body, never instantiated. Expected: compilers differ, most reject at definition.

6. M (CNS-14): CNS-14 covers only dependent template-ids. A template-id inside a template whose arguments do not depend on any parameter and fail the constraints makes the program invalid, with no diagnostic required, even when the enclosing template is never instantiated. Probe: a class template member of a constrained alias type with a fixed failing argument, never instantiated. Expected: reject on most compilers at definition.

7. O (CNS-11): The deduced and default arguments are substituted into the template parameter list itself (later defaults, dependent types of constant parameters) before the constraints are checked. A hard error in a default template argument therefore appears even when the constraints would fail. Probe: a constrained function template whose second template parameter defaults to a member of a class with a failing static assertion, a call whose argument fails the constraint, and a fallback. Expected: rejected, the static assertion fires.

8. E (CNS-7): CNS-7 says an invalid type or expression makes the atomic constraint not satisfied. Only failures that substitution forms directly behave this way. An error inside an instantiation that substitution triggers, such as a class body with a failing static assertion or the body of a function whose return type must be deduced, is a hard error. Several tests rely on this, but no rule states it. Probe: an atomic constraint that reads a member of a class whose body has a failing static assertion, with a fallback overload. Expected: rejected, not quietly not satisfied.

9. M (CNS-7): The check of an atomic constraint counts as a compile-time evaluation context. The library query that asks whether the current evaluation happens at compile time returns true inside it. Probe: a concept whose atomic constraint calls a constexpr function that returns the answer of that query. Expected: satisfied.

10. M (CNS-2, CNS-4): The mapping of an atomic constraint holds only the template parameters that occur in its expression. Two concept-ids that differ only in an argument for a parameter the atom never uses give identical atomic constraints. Probe: a two-parameter concept whose body tests only its first parameter; one overload uses it with a second argument of int, another with long plus one extra conjunct. Expected: the overload with the extra conjunct is chosen, no ambiguity.

11. M (CNS-4): Mappings compare by the arguments they produce after substitution, so two different wrapper concepts that pass the same computed argument to one inner concept give identical atoms. Probe: two wrapper concepts that each forward the parameter plus one to a common inner concept; one overload uses the first wrapper, the other uses the second wrapper and a further conjunct. Expected: the second overload is chosen.

12. O (CNS-23, ORD-13): Neither page says how the template parameters of two function templates correspond when their atomic mappings are compared. Correspondence is by position in the template parameter lists, and the function parameters at each position must have the same type. Probe: two templates with two type parameters each, declared in swapped order, with the same function parameter list, one with an extra conjunct. Expected: ambiguous call.

13. M (CPT-19, CNS-3): A type constraint on a type parameter pack becomes one fold expression, and therefore one atomic constraint, per declaration. Two templates whose packs are constrained by a concept and by a concept that subsumes it are not ordered. Probe: two variadic overloads constrained by those two concepts, called with two int arguments. Expected: ambiguous in the C++20 mode. Doubt: compilers in later language modes may order them, so the result can depend on the language mode.

14. M (CPT-19, CPT-20): A concept whose first parameter is itself a type pack can be used as a type constraint. On a pack, it is checked once per element, each time with a single element. On a single parameter, it receives just that type. Probe: a variadic concept satisfied only when its own pack has exactly one element, used on a pack of three types. Expected: satisfied.

15. M (CPT-17): A constrained type parameter can have a default argument. The default is not checked when the template is declared. It is checked when a use relies on it. Probe: a class template whose constrained parameter defaults to a type that fails, declared and then used with an empty argument list. Expected: the declaration is accepted, and the use is rejected.

16. M (CPT-20): A constrained placeholder of the decltype form cannot be the type of a function parameter. Probe: an abbreviated function template with such a parameter. Expected: reject.

17. M (CPT-22, AUTO-16): The pages do not say whether a constrained placeholder can appear in a structured binding declaration or in a functional cast. Both allow only the plain unconstrained placeholder. Probe: each form with a satisfied concept. Expected: both rejected.

18. M (CPT-22): A constrained placeholder return type on a function with no return statement, or one that returns a void expression, deduces void, and void must satisfy the concept. Probe: such a function with a concept that rejects void. Expected: reject.

19. O (CPT-33): A compound requirement is checked in a fixed order. The expression is substituted first. Next comes the no-throw test, if written. Then the extra arguments of the type constraint are substituted, and the concept check runs last. A failure at one step stops the later steps. Probe: an invalid expression, followed by a type constraint whose extra argument names a member of a class with a failing static assertion. Expected: the requirement is false and the trap is not instantiated. Repeat with a valid expression that can throw and the no-throw test. Expected: the same.

20. O (CPT-26, CPT-27): The source ties the hard error to a requires expression that is not inside the declaration of a template or templated entity. Tidal ties it to whether the expression depends on template parameters. The two differ for a non-dependent requirement inside a template. Probe: inside a function template body, a requires expression whose only requirement is an invalid expression that uses no parameter, with the template never instantiated. Expected: I expect compilers to differ. Record each compiler's result.

21. M (CPT-26): Access checking is part of substitution. A requirement that names a private member, checked from a concept, makes the requirement fail with no error. The concept is checked in its own context, so friendship granted to some other function does not help. A requires expression written inside a member of the class can use the private member. Probe: both cases. Expected: false from the concept, true from inside the member.

22. E (CPT-35): The expression of a nested requirement becomes atomic constraints, and each must be a constant. A parameter of the requires expression has no value, so it can appear only where it is not evaluated, such as in an operand of the size or type queries. CPT-35 allows use of the parameters with no limit. Probe: a nested requirement that compares a member read through a parameter with zero. Expected: the program is rejected for a non-constant constraint. The requirement is not quietly false.

23. M (CPT-28): The parameter list of a requires expression follows the ordinary rules for function parameter lists. An array or function type adjusts to a pointer. A list made of the single keyword void means no parameters. A placeholder type as a parameter type is not valid. Probe: each case. Expected: the adjusted array parameter has pointer type, the void list is accepted, and the placeholder parameter is rejected.

24. M (CPT-28): A parameter of a requires expression must not be named like any template parameter that encloses it. Probe: a concept whose requires expression names its parameter the same as the concept's type parameter. Expected: reject.

25. M (CPT-32): A type requirement takes a type name, so a fundamental type keyword cannot follow the typename keyword there. A requirement that names a constrained class template specialization fails when the arguments fail that template's constraints or the substitution into its defaults. Probe: the keyword form, and a requirement naming a constrained class template with a failing argument. Expected: a syntax error for the first, false for the second.

26. M (CPT-8): The list in CPT-8 leaves out two places where a requires clause can follow a template parameter list. These are the head of a template template parameter and a deduction guide. Probe: a deduction guide whose clause fails for one argument type, and a template template parameter with a clause. Expected: both are accepted, and the failing guide is not used.

27. M (CPT-11, CPT-14): The criterion is whether the function is templated. The closed list in CPT-11 misses some templated functions: a lambda in a default member initializer of a class template, a generic lambda outside any template, and a function declared at block scope inside a function template. Probe: a trailing clause on each. Expected: the first two are accepted. Compilers may differ on the block-scope declaration, so record the results.

28. M (CPT-1, CPT-18): A concept-id whose arguments do not match the concept's parameters in kind or number is an error, and its value is not false. Probe: a type concept given a constant argument, inside a static assertion that expects false. Expected: reject.

29. O (CNS-21): CNS-21 does not say whether a more constrained special member whose constraints are not satisfied stops a less constrained, satisfied member of the same kind from being eligible. A literal reading says it does. Implementations count only satisfied members. Probe: a class template with an unconstrained defaulted default constructor and a more constrained user-provided one whose constraint fails for int, placed in a union. Expected: on Clang and GCC the defaulted constructor is eligible and trivial, and the union is valid.

30. E (CNS-21): "Same kind" is narrower than the "same parameter types" that CNS-21 states. Two copy or move constructors are of one kind when their first parameter types match. Two assignment operators also need the same cv-qualifiers and the same reference qualifier. Probe: two copy assignment operators that differ only in reference qualifier, the more constrained one user-provided. Expected: the other one stays eligible, and the triviality result follows from it.

31. S (TSP-11, CNS-17): TSP-11 forbids a partial specialization whose argument list repeats the primary template's parameters. A partial specialization with that list is valid when its constraints make it more specialized than the primary. The CNS-17 test does exactly that. Probe: the existing CNS-17 test. Expected: accept. TSP-11 needs an exception or a cross-reference.

32. M (CNS-28, CNS-29): Take an explicit specialization of a member template of a class template specialization, given outside the class. The member it specializes is found by substituting the class arguments into each member's constraint. A member whose constraint becomes invalid after that substitution does not match. Probe: two member templates that differ only in trailing clauses, one naming a nested type that int lacks; an explicit specialization for int whose clause matches the other member after substitution. Expected: accepted, specializing the other member.

33. M (CNS-27): A constraint that contains a lambda expression is never equivalent to another, even to identical text. Probe: a class template declared twice with the same requires clause that contains a lambda; and two function templates declared that way, then called. Expected: the class redeclaration is rejected, and the call is ambiguous.

34. M (CNS-31, CNS-32): A constrained friend of CNS-31 is a different function from any declaration in another scope, even one with the same signature and constraints. Probe: a namespace-scope declaration of the same name and form, then a call that finds both through argument-dependent lookup. Expected: ambiguous call.

35. O (CNS-27, Q-0269): Two template parameters that are constant parameters compare equal when their types match, apart from any type constraint on the placeholder. The comparison of template heads does not settle whether a constrained and an unconstrained constant placeholder parameter declare the same template. Probe: a class template declared once with a constrained constant parameter and once with an unconstrained one. Expected: compilers reject, and they may give different reasons. Record each.

36. M (CNS-15): CNS-15 covers explicit instantiation of the whole class. It does not cover an explicit instantiation of a single member function whose constraints fail for the class arguments. Probe: an explicit instantiation of only that member. Expected: reject.

37. M (CNS-11, CNS-P3): Deduction guides that come from constructors keep the constructor's constraints together with the class template's constraints. Class template argument deduction then skips a guide whose constraints fail. Probe: a class template with two constructors that have exclusive constraints, deduced from arguments that satisfy only one. Expected: the deduced specialization comes from the satisfied constructor. With no constructor satisfied, deduction fails.

38. M (CNS-7): Neither page covers a check that depends on itself. This happens when checking a constraint for some arguments needs the same constraint for the same arguments, for example through overload resolution inside a requirement. Probe: a concept whose requirement calls a function constrained by that same concept. Expected: Clang and GCC reject and report a satisfaction that depends on itself. Record EDG.
