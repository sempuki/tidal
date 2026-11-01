<!--
Gap review summary, 2026-10-31: the core of templates (item N1).
Written by: a source reader under AGENTS.md that read the C++ working
draft sources and Tidal's chapter spec/templates/. It did not write or
change any file in this repository; the integrator copied this summary
here.
Checked before use: the source reader ran tools/similarity.py on its
summary outside the repository and reworded four passages; the
integrator's own check gives 0 flagged. No code, no quotes, no section
names. Each item ends with the outcome a probe should show; it is a
suggestion to probe, not a rule.
Labels: M = missing, E = possible error, S = structure.
-->

# Gap review of item N1

Role: source reader

## Template declarations and parameters

E1. TPL-6. The list of structural types leaves out a pointer to void, and a pointer to void is a valid type for a constant parameter. Any scalar type counts, with or without cv-qualifiers. Probe: a class template whose constant parameter is a pointer to void, used with the address of a global int. Expected: accepted.

M2. Nearest TPL-6. Top-level const or volatile on the type of a constant parameter is dropped when the parameter type is formed. So one declaration with a const int parameter and a second declaration with a plain int parameter declare the same template. Probe: declare a class template twice that way, then define the class. Expected: accepted.

M3. Nearest TPL-6 and TPL-7. A constant parameter must not carry a storage class keyword such as static, and the parameter declaration must not define a new type (for example a new enumeration written inside the head). Probe: each form in turn. Expected: both rejected.

M4. Nearest TPL-11. The type of a constant parameter can also be the bare name of a class template, which is then deduced from the argument the way a variable with that type gets its class arguments. A placeholder that deduces to a type not allowed for a constant parameter makes the program invalid, for example when the argument is a cast of zero to void, or a class with a private data member. Probe: a parameter of placeholder type given a void expression. Expected: rejected. Probe: a parameter whose type is a class template name with no arguments, given a braced value. Expected: accepted, with the class arguments deduced.

E5. TPL-13. A default template argument is not allowed on a pack of any kind. The rule says any parameter can have a default. TPL-14 says a pack needs no default but does not forbid one. Probe: a class template whose type pack has a default. Expected: rejected.

E6. TPL-30. The parameter after a pack in a function template is allowed only when deduction can find it from the function parameter types or when it has a default. Two packs in a row with nothing to deduce the second, or a plain type parameter after a pack that appears in no function parameter, make the declaration invalid. Probe: a function template with two type packs and no function parameters. Expected: rejected.

E7. TPL-30. The pack-last condition applies to the primary declaration of a class or variable template and to alias templates. A partial specialization can declare a pack in the middle of its own head, since its parameters are found by deduction. Probe: a partial specialization whose head lists a type pack and then a plain type parameter, with an argument list that ends in the expanded pack. Expected: accepted.

M8. Nearest TPL-30. A parameter that is itself a pack expansion (a constant pack whose type is an earlier type pack) must not expand a pack declared in the same head. Inside a nested member template the same form is fine. Probe: one head with a type pack followed by a constant pack typed by that type pack. Expected: rejected. Probe: the same constant pack in a member template of a class whose head declares the type pack. Expected: accepted.

M9. Nearest TPL-17. A friend declaration of a class template must not give a default template argument. A friend function template declaration with a default must be a definition, and no other declaration of that function template may be reachable from it in either direction. Probe: a friend class template declaration with a default. Expected: rejected. Probe: a friend function template defined in a class with a default, and declared nowhere else. Expected: accepted.

M10. Nearest TPL-13 and TARG-3. In a head, a default for a constant parameter ends at the first greater-than sign that is not nested, so a comparison there needs parentheses. Probe: a default of the form three greater-than four with no parentheses. Expected: rejected. With parentheses. Expected: accepted.

M11. Nearest TPL-12. The parameters inside the head of a template template parameter can have defaults. Those defaults belong to that declaration only. An out-of-class member definition that repeats the class head without the inner default cannot rely on the default, and one that writes a different inner default uses the new one. Probe: a member definition that uses the template template parameter with an empty list, once with no inner default and once with one. Expected: rejected, then accepted.

E12. TPL-20. Equivalent heads also need the two parameters at each position to agree on whether they are packs, and template template parameters to have equivalent heads of their own, checked recursively. Probe: redeclare a class template with a pack in place of a plain type parameter. Expected: rejected. Probe: redeclare with a template template parameter whose inner head has a constant parameter in place of a type parameter. Expected: rejected.

E13. TPL-20 and TPL-22. For function templates, a second declaration with the same name and function type but a different head declares a second overload, not an invalid redeclaration. The two can be told apart by an explicit argument list. Probe: one function template with a type parameter and one with an int parameter, both with no function parameters and the same return type, each called with a matching explicit argument. Expected: accepted, each call picks its own template.

E14. TPL-19. In a member definition outside a class template, lookup searches the class and its bases that do not depend on the parameters before the template parameter scope. So a member or a base class with the same name as a parameter of the out-of-class definition wins over that parameter. A name from the namespace that contains the class does not win. Probe: a class template with a non-dependent base named B, and an out-of-class member definition whose head names its parameter B. Inside, B names the base class. Expected: accepted with the base meaning, checked by its size.

M15. Nearest TPL-18. A friend function declaration inside a class template that uses the name of a template parameter is allowed, because the friend binds no name in that scope. A class template whose own name equals the name of its parameter is not allowed. Probe: each case. Expected: accepted, then rejected.

E16. TPL-4. A declaration that declares a class template allows no declarator at all, so a class template definition followed by a variable name is invalid, even with one declarator. Probe: a class template definition that also declares one variable. Expected: rejected.

M17. Nearest TPL-23. Member classes and member enumerations of a class template can also be defined after the class body. An enumeration with a fixed underlying type that uses the parameter can be declared in the class and completed outside. Probe: each kind of out-of-class definition, then a use. Expected: accepted.

E18. TPL-23. The argument list after the class name in an out-of-class member definition must be the head's parameters in their declared order, and a pack must appear expanded. Parameter names can differ. Probe: a two-parameter class template with a member defined outside where the head lists the parameters in one order and the argument list swaps them. Expected: rejected. Probe: a pack named without expansion in that list. Expected: rejected.

M19. Nearest TPL-24 and TPL-25. A destructor must not be a member template. A class can declare both a plain member function and a member function template that could produce a function of the same signature. A call with no explicit arguments picks the plain function, and an empty explicit list picks the template. A specialization of a member function template never overrides a virtual function of a base. Probe: a destructor template. Expected: rejected. Probe: the plain and template members, with one call each way. Expected: accepted, each call reaches the expected function.

M20. Nearest TPL-22. A plain function with the same name and type as a possible specialization of a function template is a separate function. The template is never used to define it. Probe: declare such a plain function without a definition and call it. Expected: link failure, with no fallback to the template body.

M21. Nearest TPL-3. Explicit specializations and partial specializations must not have C language linkage either. Probe: an explicit specialization inside a C linkage block. Expected: rejected.

M22. Nearest TPL-2. A friend template must not be declared in a local class. Probe: a local class with a friend class template declaration. Expected: rejected.

M23. Nearest TPL-28 and TPL-31. The type produced by a specialization of an alias template must not use that same specialization through another class. Each specialization of an alias template that contains a lambda expression gives a different closure type, even when the lambda does not use the parameters. Probe: an alias that goes through a member type of a class template, which itself uses the alias. Expected: rejected on use. Probe: an alias of the type of a lambda, compared for two arguments. Expected: different types.

M24. Nearest TPL-26 and TPL-23. Each specialization of a class template has its own copies of the static data members. A static data member template of a plain class is declared in the class and defined in the enclosing namespace. Probe: write a static member through one specialization and read it through another. Expected: the values differ.

M25. Nearest TPL-9. The object that a constant parameter of class type names is shared across the whole program for the same type and value, also between two different templates. Probe: compare the addresses of the parameter objects in two templates given equal arguments. Expected: equal.

## Template-ids and arguments

E26. TARG-2. When an unqualified name is followed by a less-than sign and ordinary lookup finds only plain functions, or finds nothing, the less-than sign opens a template argument list. The call can then find a function template by argument-dependent lookup. Probe: a function template in a namespace, called with an explicit argument list from outside, with an argument of a class type from that namespace, where ordinary lookup finds nothing. Expected: accepted. A variable name followed by a less-than sign stays the operator.

E27. TARG-3. The greater-than signs that close the type of a named cast, or that close a nested template argument list, count as nested, so they do not end the outer list. When a double greater-than token is split, the second half can close something other than a template argument list, such as the angle brackets of a cast or of a head. Probe: a template argument that contains a static cast. Expected: accepted. Probe: a named cast whose target type is a template-id ending right before the cast's closing sign, written as one double token. Expected: accepted.

M28. Nearest TARG-6. A template-id is also invalid when putting its arguments into later parameters fails, for example when a later constant parameter has a type taken from a member of an earlier type argument that the argument lacks. Probe: a class template whose second parameter's type is a nested type of the first, used with int. Expected: rejected.

M29. Nearest TARG-7 and TARG-5. A local class, an unnamed class, an unnamed enumeration, and an incomplete class are all valid type arguments. Probe: each as an argument of a class template, with the incomplete one used only through a pointer. Expected: accepted.

M30. Nearest TARG-7. Names in a template argument are access-checked where the argument appears. The template gets no special access to members of a class or template it receives. A class can name its own private nested type as an argument from inside itself. Probe: a private nested type used as an argument outside the class. Expected: rejected. From inside the class. Expected: accepted.

E31. TARG-12. The allowed conversions for a constant argument are wider than promotions, integral conversions, and user-defined conversions. Array-to-pointer, function-to-pointer, qualification conversions, the conversion of the null pointer constant from nullptr, and the function pointer conversion that drops a no-throw guarantee are allowed too. An overloaded function name given to a function pointer parameter picks the matching overload. Probe: an array of int given to a parameter of pointer to const int. Expected: accepted. Probe: an overloaded name given to a function pointer parameter. Expected: accepted, with the overload matching the parameter type.

M32. Nearest TARG-15. A pointer or reference argument also must not point to the result of a typeid expression or to the predefined function-name variable, or to a subobject of either. The same limits apply to a pointer or reference member inside an argument of class type, so a class argument whose reference member is bound to a member of a temporary is invalid. A string literal can still serve as an argument for a class-type parameter whose constructor takes a pointer to char. Probe: each case. Expected: the first two rejected, the last accepted.

M33. Nearest TARG-17. Only primary templates are compared when a template template argument is checked. Partial specializations of the argument play no role in that check, but they are used later when the parameter is used with arguments. Probe: a template template argument with a pointer partial specialization, used inside the receiving template with a pointer type and a plain type. Expected: the pointer use picks the partial specialization.

E34. TARG-17 and TARG-18. Constant parameters of the two heads must have equivalent types, but a parameter of placeholder type in the argument accepts a parameter of int type in the head of the template template parameter. The reverse does not hold. Probe: a template template parameter whose head has an int parameter, given a template whose parameter is a placeholder. Expected: accepted. Swap the two. Expected: rejected.

M35. Nearest TARG-11. In an explicit or partial specialization, the injected class name used alone as a type means that specialization's own argument list. When the injected class name of the same class template is found in two bases, use as a template is not ambiguous, while use as a type is. A qualified name that reaches the template from namespace scope never finds the injected name, so it needs an argument list even inside the class. Probe: each case. Expected: accepted, accepted as template and rejected as type, rejected.

M36. Nearest TARG-P4 and TARG-19. Sameness also covers arguments of union type (same active member with the same value, or no active member in both) and array members inside class arguments, compared element by element. Two different enumerators with equal values are the same argument. Probe: a class template with an enumeration parameter, given two enumerators of one value. Expected: same type.

## Explicit specialization

E37. TSP-8. The ban on default function arguments also covers an explicit specialization of a member function template, and an explicit specialization of a member function for a class specialization that comes from the primary template. A member function of a class specialization that is itself explicitly specialized may have defaults. Probe: each case. Expected: rejected, rejected, accepted.

M38. Nearest TSP-4. An explicit specialization must not use a storage class keyword other than thread_local. Probe: an explicit specialization of a function template marked static. Expected: rejected.

M39. Nearest TSP-2. The inline and constexpr properties of an explicit specialization come from its own declaration. A function template or variable template that has them passes neither to its explicit specializations. Attributes of the template do not carry over. Probe: an inline function template whose explicit specialization is defined without inline in a header seen by two translation units. Expected: a multiple definition error at link time.

M40. Nearest TSP-3 and TSP-1. When a whole class specialization is given explicitly, its members get out-of-class definitions in the same form as members of an ordinary class, with no template-empty prefix. Nested classes behave the same way: once a nested class of one specialization is given explicitly, its own members are defined without the prefix. A member of a member class template that is explicitly specialized as a class template still needs the prefix. Probe: each form with and without the prefix. Expected: the plain-class forms reject the prefix, and the class template form requires it.

E41. TSP-3. To specialize a member template of a class template at namespace scope, every enclosing class template must be explicitly specialized as well. A declaration that specializes the inner member while an enclosing class template keeps its parameters is invalid. One template-empty prefix is needed for each enclosing class template that is specialized. Probe: specialize a member function of a nested class template for a fixed inner argument while the outer template keeps its parameter. Expected: rejected.

M42. Nearest TSP-3. Explicitly specializing an implicitly declared special member function is invalid. Probe: an explicit specialization of the implicit copy constructor of one class specialization. Expected: rejected.

M43. Nearest TSP-12. An explicit specialization must not be a friend declaration. Probe: a friend declaration that starts with the template-empty prefix. Expected: rejected.

M44. Nearest TSP-2. When a program explicitly specializes a static member variable, or a static member variable template, the declaration defines the variable only if it has an initializer. Without one, the variable is declared and left undefined. An empty pair of parentheses makes a function declaration. Probe: the declaration form used and never defined. Expected: link failure.

M45. Nearest TSP-4. Names in the declaration of an explicit specialization are not access-checked, except inside the body, default arguments, base list, member list, enumerator list, and initializer. The same holds for the argument list of a partial specialization. Probe: an explicit specialization whose argument is a private nested type of another class. Expected: accepted.

M46. Nearest TSP-6. A member enumeration of a class template that is completed in the class template definition is created with the class specialization, so an explicit specialization of that enumeration after the class specialization is used is invalid. A scoped one that is only declared there is fine. Probe: both kinds after a use of the class specialization. Expected: rejected, then accepted.

## Partial specialization

M47. Nearest TSP-11. A constant parameter of the primary template whose type depends on an earlier parameter must not get a fixed value in a partial specialization while that earlier parameter stays a parameter of the partial specialization. Probe: a primary with a type parameter and a constant of that type, and a partial specialization that fixes the constant to one and keeps the type. Expected: rejected.

M48. Nearest TSP-11. An argument of a partial specialization must not contain a pack that is not expanded, and a pack expansion must be the last argument. Probe: an expansion in the first of two arguments. Expected: rejected.

M49. Nearest TSP-9. The primary template must be declared before any partial specialization. A partial specialization can be declared again, and two declarations are the same when their heads and argument lists are equivalent. A partial specialization of a member template can be declared in the class, or outside the class with the enclosing heads in front. Probe: declare a partial specialization, then define it. Expected: accepted. Probe: a partial specialization with no primary. Expected: rejected.

E50. TSP-1 and TSP-18. The members of a partial specialization are unrelated to the primary's members as well. Using a member that the partial specialization declares but never defines is an error, even when the primary defines a member with that name. The rules state the separation for explicit specializations only. Probe: call a member declared in a partial specialization and defined only for the primary. Expected: link failure.

M51. Nearest TSP-14. Partial specializations are found through the primary template, so a using-declaration that names the primary also brings in partial specializations declared after it. Probe: a using-declaration of a namespace class template, then a partial specialization in the namespace, then a use through the using name. Expected: the partial specialization is selected.

M52. Nearest TSP-13. When the primary member template of a class template is explicitly specialized for one class specialization, its partial specializations are ignored for that class specialization. When one partial specialization is explicitly specialized, the primary and the other partial specializations still count. Probe: a member class template with a pointer partial specialization, the primary member specialized for short, and a pointer use with short. Expected: the explicit specialization is used.

M53. Nearest TSP-9 and TSP-1. The class key of an explicit or partial specialization must agree in kind with the primary. A union and a class do not agree, while struct and class do. Probe: a union explicit specialization of a struct template. Expected: rejected.

S54. The chapter gives explicit specialization of member classes, member enumerations, member class templates, and static data member templates no rule of its own, and TSP-P1 lists only functions and static data members as members that can be specialized. The member specialization rules (prefix count, enclosing templates, definitions of members of specialized classes) are spread across TSP-3 and TSP-19 with no group heading.

S55. Access checking of template arguments (M30 and M45) has no place in the chapter. A short group on access in template-ids and in specialization declarations would hold both items.
