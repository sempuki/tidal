<!--
Gap review summary, 2026-10-31: third review, earlier declaration pages.
Written by: a reviewer agent (a source reader under AGENTS.md) that read the
C++23 working draft N4950 and cppreference. It did not write or change any
file in this repository; the writer copied this summary here.
Checked before use: tools/similarity.py against cplusplus/draft n4950
(0 flagged; the reviewer reworded one flagged passage first), no code, no
section names. Each item is a suggestion to probe, not a rule.
-->

# Tidal declarations review, round 3 (part C)

Written by a reviewer agent acting as a source reader under AGENTS.md. It compared the C++23 reference material with these Tidal pages: auto, storage-linkage, declarators, structured-bindings, enumerations, namespaces, attributes, and constexpr under spec/declarations; forms, list, references, and static under spec/declarations/initialization; definitions, default-arguments, deleted, and return-deduction under spec/declarations/functions. Items from the first gap review are not repeated unless the current rule still looks wrong. Labels: M is missing behavior, E is a possible error in an existing rule (the Tidal rule ID is named), S is structure. Each item gives the result a probe should expect. Clang 22 and GCC 16 were run in C++23 mode with pedantic errors for the items that say so.

## declarations/auto

- E1 (AUTO-3). The rule says the initializer for a plain lvalue-reference placeholder must be an lvalue unless the declaration adds const. An xvalue of a const-qualified type also works, because deduction picks up the const from the initializer. Probe a reference placeholder initialized with a cast of a const int to an rvalue reference to const int. Expect accept. Clang and GCC both accept.
- E2 (AUTO-14). A static data member whose declaration is const (and neither inline nor constexpr) can also use a placeholder when it has an initializer in the class. Probe a static const placeholder member initialized with 1 and read in main. Expect accept. Clang and GCC both accept.
- E3 (AUTO-2). The string literal case gives a pointer to const of the literal's own character type. A UTF-8 literal gives a pointer to const char8_t, and the wide and UTF-16/32 literals give their own types. The rule names only plain char. Probe a placeholder initialized from a UTF-8 literal and compare its type with pointer to const char8_t. Expect accept.
- M1. With decltype(auto), the name of an array gives the array type itself. Copy-initializing an array from another array is not allowed, so the declaration fails. Probe decltype(auto) initialized from the name of an int array. Expect reject. Both compilers reject.
- M2. AUTO-12 lists if conditions, if initializers, and for initializers. Placeholders are also allowed in while conditions, switch conditions and switch initializers. Probe a while loop whose condition declares a placeholder variable. Expect accept.

## declarations/storage-linkage

- E1 (LNK-9). The rule says a block-scope extern declaration takes the linkage of an earlier namespace-scope declaration that is visible. In C++23 it redeclares that earlier entity even when a local variable of the same name in an outer block hides it, and so it gets internal linkage. Probe a namespace-scope static int holding 5, and a function that declares a local int of the same name holding 1, then opens an inner block with an extern declaration of that name and returns it. Expect run: 5. Clang and GCC both give 5.
- M1. STO-6 forbids mutable only on namespace-scope variables. A block-scope variable, a function, a static member, a const member and a reference member cannot be mutable either. Probe a block-scope mutable int. Expect reject.
- M2. During program termination, if a block static has already been destroyed and control later passes through its declaration again (for example, from the destructor of another static object), or the program uses that object, the behavior is undefined. Probe a namespace-scope object whose destructor calls a function that has a block static of class type which was initialized after the namespace object. Expect undefined.

## declarations/declarators

- E1 (CV-8). A const variable that is also volatile cannot be read in a constant expression, even with a constant initializer. The rule says any const integer variable with a constant initializer can. Probe a const volatile int initialized with 1 and used as an array bound. Expect reject. Both compilers reject.
- M1. A function type that carries a cv-qualifier or a ref-qualifier (only valid for a non-static member function) can be named through an alias. It can be used to declare a member function or as the pointee of a pointer to member. A pointer to it, a reference to it, or a parameter or variable of that type is ill-formed. Probe an alias for a function type with a trailing const qualifier and a pointer declared to it. Expect reject. Both compilers reject.
- M2. An array of unknown bound whose initializer is an empty braced list would have zero elements, which DCL-5 forbids. Probe an int array with no bound and an empty braced initializer. Expect reject. GCC rejects. Clang accepts even with pedantic errors.

## declarations/structured-bindings

- E1 (SB-1). The rule says the hidden variable has the written auto. For an array initializer, the hidden variable is an array of the same element type and bound, and its elements are copied one by one. Ordinary auto deduction would give a pointer. The rule should not suggest the decay. Probe a plain decomposition of a two-element int array into two names. Expect accept (with decay it would have one element and fail).
- M1. In C++23 the only specifiers allowed besides auto are static, thread_local, const and volatile. Probe a structured binding at namespace scope marked inline, then one marked extern, then one marked constinit. Expect reject for each. Clang and GCC reject all three. GCC gives a different reason for constinit.
- M2. Each binding name is an lvalue, whatever the kind of the hidden variable. Probe a decomposition of an array prvalue through a forwarding reference, then assign to one name. Expect accept.

## declarations/enumerations

- E1 (ENUM-1). The rule says the first enumerator without a value gets 0. Only the first enumerator of the list gets 0 when it has no value. A later enumerator without a value gets the previous value plus one, even when it is the first one without a value. Probe an unscoped enum whose first enumerator is 5 and whose second has no value, then check the second against 6. Expect accept. Both compilers agree.
- M1. A scoped enumeration value does not convert to bool contextually, so it cannot be an if condition, a loop condition, or an operand of the logical operators. Probe an if whose condition is a scoped enumerator. Expect reject. Both compilers reject.
- M2. An enumerator's own name is in scope only after its definition, including its value. Inside the value expression, the same name still refers to an outer entity. Probe a function with a local unnamed enum whose single enumerator has the same name as a namespace-scope const int holding 12 and uses that name as its value, then return the enumerator. Expect run: 12. Both compilers give 12.
- M3. The enumeration named in a using enum declaration can be written through a type alias. Probe an alias for a scoped enum, then a using enum declaration on the alias, then an unqualified enumerator. Expect accept. Both compilers accept.

## declarations/namespaces

- M1. A namespace alias cannot be used to reopen a namespace in a namespace definition. Probe an alias for an existing namespace followed by a namespace definition that uses the alias name. Expect reject. Both compilers reject.
- M2. A using directive is not a snapshot. Members that are added to the namespace after the directive are also found. Probe a directive for a namespace, then a reopening of that namespace that adds a variable, then an unqualified use of that variable. Expect accept. Both compilers accept.
- M3. A using directive cannot appear in a class. Probe a directive in a struct body. Expect reject.
- M4. One using declaration can list several qualified names separated by commas. Probe one using declaration that names a variable from each of two namespaces. Expect accept.
- M5. A namespace-scope using declaration must not name a class member. Probe a using declaration at global scope that names a static data member of a struct. Expect reject. This belongs partly to classes.

## declarations/attributes

- M1. A using prefix in an attribute specifier forbids namespace-qualified attributes in the same specifier. Probe a specifier with a prefix for one vendor namespace that also lists an attribute qualified with that namespace. Expect reject. Both compilers reject.
- M2. Two adjacent left square brackets may only start an attribute specifier. A subscript whose index starts with a lambda therefore fails. Probe an array subscript whose index is a lambda called immediately, written without extra parentheses. Expect reject. Both compilers reject.
- M3. The argument of assume is a single expression without a top-level comma operator. Probe an assumption that lists two variables separated by a comma. Expect reject. Both compilers reject, for different stated reasons.
- M4. no_unique_address may not be applied to a bit-field. Probe it on a three-bit int member. Expect reject. Clang rejects. GCC accepts.
- M5. When fallthrough is inside a loop body that is inside a switch, the next statement executed must be a case or default label of that switch within the same loop iteration. A fallthrough at the end of a loop body breaks this. Probe a fallthrough as the last statement of a for loop body in a case, followed by another case label after the loop. Expect reject. Clang and GCC both accept. GCC warns that the attribute does not precede a label.
- M6. carries_dependency follows the same first-declaration rule as noreturn. Probe a function declared without it and then redeclared with it. Expect reject. Clang rejects. GCC accepts (it ignores the attribute).
- M7. A program can add deprecated on a later declaration of an entity that was first declared without it. Uses after that point should warn. Probe a function declared, then redeclared deprecated, then called. Expect accept with a recommended warning. Both compilers warn.
- M8. The nodiscard warning applies only when the call returns the marked class or enumeration by value. A call that returns a reference to such a type is not covered. Probe a function returning a reference to a nodiscard struct, with the result discarded. Expect accept with no warning. Both compilers are silent.

## declarations/constexpr

- E1 (CEV-1). The escalation rule names only constexpr function templates. In C++23 a lambda's function call operator escalates as well, unless the lambda is itself marked consteval. A defaulted special member function escalates too. Probe a lambda that passes its parameter to a consteval function, called once with a constant to initialize a constexpr variable and once with a runtime value. Expect accept for the constant call and reject for the runtime call. Both compilers agree.
- M1. A constexpr reference at namespace scope can bind to a temporary, because the extended temporary has static storage. At block scope without static, it is rejected. Probe a namespace-scope constexpr reference to const int bound to the literal 1, then the same declaration inside main. Expect accept for the first and reject for the second. Both compilers agree.
- M2. The result of a consteval call must be a permitted result of a constant expression. A pointer to memory allocated during the call and not freed in it fails. Probe a consteval function that returns a newly allocated int pointer, called at run time with the pointer deleted afterwards. Expect reject. Both compilers reject.
- M3. constinit requires only constant initialization. It does not require constant destruction. Probe a constinit global of a class with a constexpr default constructor and a destructor that is not constexpr. Expect accept. Both compilers accept.

## declarations/initialization/forms

- M1. A null pointer constant of the nullptr type converts to bool only in direct-initialization. Probe a bool copy-initialized from nullptr, then a bool direct-initialized from it with parentheses and with braces. Expect reject for the first and accept for the others, with the value false. Both compilers agree.
- M2. Parenthesized initialization of an array with no bound takes the bound from the number of values. Probe an int array with no bound and three values in parentheses, then return its element count. Expect run: 3. Both compilers give 3.
- M3. The declarators of one declaration are initialized in order, left to right. Probe one declaration with two int variables whose initializers call two printing functions. Expect run: the first declarator's function prints first. Both compilers agree.

## declarations/initialization/list

- M1. An aggregate member that is an empty aggregate (for example, an empty base class) cannot have its initializer omitted by brace elision while later elements are given. Probe a struct with an empty public base and one int member, brace-initialized with one integer. Expect reject. Both compilers reject.
- M2. Brace elision is not used when the value can initialize the member directly. Probe an outer aggregate whose first member is an inner aggregate, initialized from a variable of the inner type and then an integer. Expect run: the inner member is a copy and the integer goes to the next member. Both compilers agree.
- M3. A scalar cannot be list-initialized from a list whose only element is itself a braced list. Probe an int initialized with doubled braces around 1. Expect reject. GCC rejects. Clang accepts with pedantic errors on.
- M4. A member omitted from the list is copy-initialized from an empty list, so an explicit default constructor is not usable for it. Probe an aggregate with an int and a member of a class whose default constructor is explicit, initialized with only the int. Expect reject. Both compilers reject.
- M5. When a character array is longer than the string literal, the remaining elements are zero. Probe a five-element char array initialized from a two-character literal, then read the last two elements. Expect run: both zero.

## declarations/initialization/references

- E1 (REF-1). The rule requires the same type, with fewer cv-qualifiers allowed. A non-const lvalue reference to a class also binds directly to an lvalue of a derived class through an unambiguous accessible base. Probe a reference to the base initialized from a derived object, then a read of a base member. Expect accept and run with the derived object's value. Both compilers agree.
- E2 (REF-2). The rule says a const lvalue reference that binds to an rvalue binds to a temporary copy. An xvalue of the same type (or a class rvalue of a compatible type) is bound directly, with no copy. Probe a const int reference initialized with a cast of a global int to an rvalue reference, then change the global and read through the reference. Expect run: the reference shows the new value. Both compilers give the new value.
- E3 (REF-3). An rvalue reference must not drop cv-qualifiers either. The rule does not say this. Probe an rvalue reference to int initialized from an xvalue of const int. Expect reject. Both compilers reject.
- E4 (REF-5). REF-5 says only the bindings in REF-4 extend a temporary's life. Extension also passes through a cast with no user-defined conversion (static_cast, const_cast and the others) and through a subscript on an array member of a temporary. Probe a const reference to a class bound to a static_cast of a class prvalue whose destructor prints, and a second case that binds an rvalue reference to an element of an array member of a class prvalue. Expect run: in both cases the destructor runs at the end of the reference's block. Both compilers agree.
- M1. A reference to an array of unknown bound can bind to an array of known bound (C++20). Probe such a reference bound to a three-element int array. Expect accept. Both compilers accept.
- M2. Binding a reference to the result of dereferencing a null pointer is undefined. Probe an int reference initialized through a null int pointer, inside a constant evaluation. Expect undefined (rejected during constant evaluation).

## declarations/initialization/static

- E1 (SINIT-2). The rule says the order of definitions applies to inline variables too. An inline variable has only partial ordering. It is ordered after an earlier variable only when that earlier variable is itself inline and comes first in every unit that defines the later one. An ordinary variable defined before an inline variable in the same unit is not guaranteed to be initialized first, because another unit may initialize the inline variable earlier. Probe two units that both define an inline int as one plus an extern int; the first unit also defines that extern int from a volatile holding 7 just before the inline variable, and main returns the inline variable. Expect unspecified: 8 or 1. Clang gives 1 for both link orders. GCC gives 8 or 1 depending on link order.
- M1. Dynamic initialization of a namespace-scope thread_local variable can happen before the thread's first statement or be deferred until first use. The choice is implementation-defined. Probe a namespace-scope thread_local int initialized by a printing function, with main printing first and then reading the variable. Expect implementation-defined order. Clang and GCC both defer, so main prints first.

## declarations/functions/definitions

- E1 (FN-5). The string held by the predefined function-name variable is implementation-defined. The rule says it holds the name of the function, and the size test relies on that. Clang and GCC give the plain name for a simple function. Expect implementation-defined. Whether this variable has an address distinct from other objects is unspecified.
- M1. In a function definition, each parameter type and the return type must be complete (other than void), unless the definition is deleted. A declaration that is not a definition may use incomplete types. Probe a definition taking an incomplete struct by value. Expect reject. Probe a plain declaration and a deleted definition with the same parameter. Expect accept. Both compilers agree.
- M2. A function definition must use a function declarator. A function type introduced by a typedef cannot be used to define the function. Probe a typedef for a function type, then a definition of a function written with only that typedef name and a body. Expect reject. Both compilers reject.
- M3. Any entity named main that has C language linkage is ill-formed, in any namespace. Probe a namespace that declares a function main with C linkage. Expect reject. Clang and GCC both accept.
- M4. If main is given argc and argv and argc is not zero, argv[0] points to the name used to start the program or to an empty string. Probe main returning whether argc is at least one and argv[0] is not null. Expect run: 0 on a hosted system that passes the program name.

## declarations/functions/default-arguments

- M1. A default argument is checked against the parameter type at the declaration, even when no call uses it. Probe a pointer parameter whose default is a floating constant, with no call. Expect reject. Both compilers reject.
- M2. When a using declaration brings in a namespace function, default arguments that a later redeclaration in the namespace adds are also usable through the using declaration. Probe a namespace function with one int parameter, a using declaration for it, a later redeclaration in the namespace that gives the parameter a default, and an unqualified call with no arguments. Expect accept. GCC accepts. Clang rejects.
- M3. If an inline function is declared in several units, the accumulated default arguments must be the same in each unit. Expect no diagnostic required.

## declarations/functions/deleted

No new items. DEL-1 to DEL-5 match the reference material for the cases outside classes.

## declarations/functions/return-deduction

- E1 (RET-6). In C++23, a parenthesized name of a local variable in a return statement is an xvalue, because of the implicit move rule (jumps page). decltype(auto) therefore deduces an rvalue reference to the local. The rule says an lvalue reference. For a global variable, the result is still an lvalue reference. Probe a decltype(auto) function that returns a parenthesized local int, and compare the call's type with an rvalue reference to int using a small type-equality template. Expect accept. Both compilers agree. The undefined use of the result does not change.
- M1. The same implicit move rule makes a plain lvalue-reference placeholder return of a local ill-formed. The return operand is an xvalue and cannot bind. Probe a function with a deduced lvalue-reference return that returns a local int by name. Expect reject. Both compilers reject.
- M2. A function with a reference or pointer placeholder return and no return statement cannot deduce from void. Probe a function with a deduced lvalue-reference return and an empty body. Expect reject. Both compilers reject.
- M3. Return statements in a discarded branch of if constexpr do not take part in deduction, also outside templates. Probe a deduced function whose discarded branch returns a double and whose other return gives an int, then call it through a pointer to a function returning int. Expect accept. Both compilers accept.
