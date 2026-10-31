<!--
Gap review summary, 2026-10-31: expression and declaration pages added or skipped since the first review.
Written by: a reviewer agent (a source reader under AGENTS.md) that read the
C++23 working draft N4950 and cppreference. It did not write or change any
file in this repository; the writer copied this summary here.
Checked before use: tools/similarity.py against cplusplus/draft n4950
(0 flagged; the reviewer reworded flagged passages before handing it over),
no code, no section names. Each item is a suggestion to probe, not a rule.
-->

# Review of selected expression and declaration pages

Role of the author: source reader. This review covers ten expression pages (primary, decltype, member-pointers, value-categories for VC-8 to VC-10 only, unary, postfix, casts, constant-expressions, new-delete, lambdas) and seven declaration pages (type-names, static-assert, inline, alignment, linkage-specifications, exception-specifications, volatile). Each item has a label. M marks a behavior that no rule covers. E marks a possible error in an existing rule and names the rule. S marks a problem of structure. Each item ends with the result that a probe should expect. Where Clang 22 and GCC 16 disagree in C++23 mode with pedantic errors, the item says so.

## expressions/primary.md

- E (PRIM-15). A floating literal whose value lies outside the range of its type makes the program ill-formed in C++23. The rule calls the value implementation-defined. Expected: reject.
- M. A floating literal value that lies inside the range but has no exact representation becomes one of the two nearest representable values. The implementation picks which one. Expected: implementation-defined.
- M. C++23 adds floating suffixes for the optional extended floating types (16, 32, 64 and 128 bit, and the brain-float 16 format), in lower and upper case. Their support is conditional. Expected: implementation-defined (accept on an implementation that has the type, reject otherwise).
- E (PRIM-18). The last bullet does not separate prefixed from unprefixed literals. An unprefixed character literal that names one character with no single-unit encoding in the ordinary literal encoding is conditionally supported. Its type is int and its value comes from the implementation. Only the prefixed forms are ill-formed. Expected: implementation-defined for the unprefixed form, reject for a prefixed form.
- M. C++23 adds named universal character escapes, which give a character by its Unicode name in braces. An unknown name is an error. Expected: accept for a valid name, reject for an unknown name.
- M. The delimiter of a raw string must not contain parentheses, a tab, a newline, or other control characters. PRIM-19 lists only the space and the backslash. Expected: reject.
- M. Two evaluations of the same string literal, for example two calls of a function that returns the address of one literal, can give the same object or different objects. Expected: unspecified.
- M. A digit separator directly after the hexadecimal or binary prefix is invalid because the prefix is not a digit. A binary or hexadecimal prefix with no digits after it is also invalid. Expected: reject.
- M. The keyword this cannot be used in a member function that has an explicit object parameter. Expected: reject.
- M. The keyword this can appear in the trailing return type of a member function declaration, after the qualifiers. Expected: accept.
- M. In C++23, a name in a return statement that denotes a local object or a by-value parameter that can be moved is an xvalue. One result is that a function returning an lvalue reference to its own local variable no longer compiles. Both Clang and GCC reject it. Expected: reject.
- M. A name that denotes a non-static data member, used inside a member function, has the cv-qualifiers of the object that this points to, unless the member is mutable or a reference. Expected: reject an assignment to an ordinary member in a const member function, accept one to a mutable member.

## expressions/decltype.md

- M. When the operand of decltype is a function call (or a comma expression whose right side is a call) that returns a class prvalue, the destructor of the class is not checked. A deleted or inaccessible destructor is fine there. In the operand of sizeof the same call is rejected by Clang and GCC. Expected: accept for decltype, reject for sizeof.
- M. The incomplete-type allowance in UNEV-1 holds only for a call that is the whole operand. Member access on the result of such a call needs a complete type. Expected: reject.
- M. decltype can name the class in an explicit destructor call through a pointer or object. Expected: accept.
- M. A discarded-value expression that is a volatile glvalue of certain simple forms (a name, a member access, a subscript, an indirection, a pointer-to-member access, or a conditional or comma whose result has one of these forms) is read. UNEV-2 says only that a glvalue is not copied. Expected: run, with one volatile read performed.

## expressions/member-pointers.md

- E (MPTR-2). When the named member is declared in a base class, the type of the pointer to member names the base class, not the class named in the qualifier. Clang and GCC agree. Expected: accept a check that the type uses the base class.
- M. A pointer to a data member that is declared mutable does not let a program change that member of a const object through the dot-star or arrow-star operator. The const of the object applies. Expected: reject.
- E (MPTR-7). The rule makes every comparison that involves a pointer to a virtual member function unspecified. A comparison with a null member pointer is exact: a pointer to a virtual function compares unequal to null. Expected: run, unequal.
- M. Two pointers to members of the same type that refer to different members of one union compare equal. Expected: run, equal.
- M. For an explicit object member function, taking its address with the qualified name gives an ordinary function pointer whose first parameter is the object parameter. It does not give a pointer to member. Expected: accept the ordinary pointer, reject the member pointer.
- M. reinterpret_cast can convert one pointer-to-member-function type to another. A conversion back to the original type gives the original value. A call through the intermediate type is undefined. Expected: accept, run with the round trip giving the original, undefined for the call.
- M. The implicit base-to-derived conversion and the reverse static_cast both need an accessible base. A private base blocks them. MPTR-11 names only virtual and ambiguous bases. Expected: reject.
- M. A qualification conversion can add const or volatile to the member type of a pointer to member, also at deeper levels. Expected: accept.
- M. An integer zero literal is a null pointer constant and converts to a null pointer to member. A pointer to member with static storage and no initializer is null. Expected: run, equal to null.

## expressions/value-categories.md (VC-8 to VC-10)

- M (VC-8). Applying the dot-star operator to a class prvalue also creates a temporary. Expected: run, one temporary object is created, and it goes away once the whole statement finishes.
- M (VC-9). C++23 extends the life of every temporary that the range expression of a range-based for statement creates, up to the end of the loop. Clang and GCC both destroy such a temporary after the last iteration. Expected: run, destructor output after the loop body output.
- M (VC-9). When an array is initialized by calling a constructor for each element, and that constructor has a default argument that makes a temporary, each such temporary is destroyed before the next element is initialized. Expected: run, construction and destruction alternate per element.

## expressions/unary.md

- M. The unary address operator cannot be applied to a bit-field. Expected: reject.
- M. sizeof cannot be applied to a bit-field. Expected: reject.
- M. sizeof of any complete object type is at least one, including an empty class. The size of a class includes padding. Expected: run, size of an empty class is one or more.
- M. alignof of an incomplete class or of void is invalid. alignof of an array of unknown bound gives the alignment of the element type. Expected: reject for the first, accept for the second.
- M. Prefix increment and decrement need a pointer to a complete object type. A pointer to an incomplete class or a pointer to a function is invalid. Expected: reject.
- M. Prefix increment of a signed integer at its maximum, and prefix decrement at its minimum, are undefined. Moving a pointer outside the range from the first element to one past the last is also undefined. Expected: undefined.
- E (UN-18). The rule lists only calls to potentially-throwing functions and throw expressions. The result is also false for a new expression whose allocation function can throw (the default one), for a dynamic_cast to a reference type that can fail, for a typeid on a dereferenced polymorphic pointer, and for any implicit call that is potentially throwing, such as the constructor of a temporary or a conversion function. A new expression with the plain global allocation function gives false on Clang and GCC. Expected: run, false.
- M. Indirection through a pointer that does not point to an object or function, such as a dangling or past-the-end pointer, and then using the result to access a value, is undefined. UN-7 names only the null pointer. Expected: undefined.

## expressions/postfix.md

- E (PFX-4). Writes outside the array are undefined too, not only reads. Forming a pointer more than one past the end by subscripting is also undefined. Expected: undefined.
- E (PFX-13). The rule does not require an arithmetic or pointer-to-object operand. Postfix increment of an enumeration, of a pointer to an incomplete class, or of a pointer to a function is invalid. Expected: reject.
- M. In a subscript, the left operand is sequenced before the index operand. Expected: run, side effects of the left operand happen first.
- M. An argument that matches the ellipsis gets the lvalue-to-rvalue, array-to-pointer and function-to-pointer conversions. A float is promoted to double and small integers go through integer promotion. A null pointer literal becomes a void pointer. Expected: run, a float argument read back as double gives the same value.
- M. Passing a class object with a non-trivial copy constructor, move constructor or destructor through the ellipsis is conditionally supported. Expected: implementation-defined.
- M. Calling a function through a pointer whose function type differs from the type of the function it points to is undefined. Expected: undefined.
- M. The name of a non-static member function, reached through member access, can only be called. Using it as a value is invalid. Expected: reject.
- M. A function call whose return type is an incomplete class is invalid outside the operand of decltype. A parameter of incomplete class type is invalid at the call too. Expected: reject.
- M. When member access names a static member, the object expression on the left is still evaluated. Clang and GCC both run its side effect. Expected: run, the side effect happens.
- M. A mutable member of a const object can be changed through member access, and a reference member of a const object gives a non-const lvalue of the referenced type. PFX-10 does not give the cv-qualifiers of the result. Expected: accept.
- M. A call of a destructor through member access on an object of scalar type, using a type alias, is valid and ends the life of the object. Expected: accept.

## expressions/casts.md

- E (CAST-1). static_cast to a type is valid whenever a direct initialization of that type from the operand would be valid. That includes explicit constructors and explicit conversion functions, which an implicit conversion cannot use. Clang and GCC accept both. Expected: accept.
- M. C++23 adds the decay-copy forms that use the auto placeholder as the type of a functional cast, with parentheses or braces. The result is a prvalue copy, and an array operand decays to a pointer. Expected: accept, run with the original unchanged after the copy is modified.
- M. static_cast can turn a floating value into an enumeration. Expected: accept.
- M. static_cast of an integer to an enumeration without a fixed underlying type, where the value is outside the range of the enumeration, is undefined. Expected: undefined.
- M. static_cast from a pointer or reference to a base class down to a derived class is valid when the base is accessible, non-virtual and unambiguous. If the object is not really a subobject of the derived class, the behavior is undefined. Expected: accept, undefined for the bad object.
- M. static_cast cannot convert an integer to a pointer, except for a null pointer constant, and cannot convert between function pointers and object pointers. Expected: reject.
- M. const_cast also works on pointers to data members and on multi-level pointers. It cannot change a function pointer. Expected: accept for the first two, reject for the last.
- M. The C-style and functional casts can convert to a private or otherwise inaccessible base class, which static_cast cannot. Expected: accept.
- M. A functional cast with an empty pair of parentheses value-initializes. For a scalar type it gives zero, and for void it gives a void prvalue. Expected: accept, run with zero.
- M. A braced functional cast to an array of unknown bound gives an array whose length is the number of elements in the braces. Expected: accept, size of three elements for three values.
- M. reinterpret_cast to a reference type gives an lvalue for the same object. reinterpret_cast from the null pointer type to an integer is valid. Expected: accept.
- M. Converting an object pointer to another object pointer type with stricter alignment and back gives an unspecified pointer when the original was not suitably aligned. The mapping between pointers and integers is implementation-defined. Expected: unspecified for the first, implementation-defined for the second.

## expressions/constant-expressions.md

- M. A pointer or reference that is the result of a constant expression must not point to an object with automatic storage or to a temporary. The address of a static object is fine. Expected: reject for a local, accept for a static.
- M. A relational comparison of pointers to unrelated objects has an unspecified result, so constant evaluation rejects it. Expected: reject.
- M. Constant evaluation of a call to a constexpr function that is declared but not yet defined at that point is invalid. Expected: reject.
- M. In C++23, a constexpr function can contain a static or thread-local variable, a goto, or a variable of non-literal type. Constant evaluation is rejected only if it reaches them. Expected: accept when the path is not taken, reject when it is taken.
- M. In C++23, a reference parameter whose referent is not known during constant evaluation can still be used when the evaluation does not read through it, for example to call a constexpr member function that does not touch the object. Clang and GCC accept. Expected: accept.
- E (CE-3). A const variable of enumeration type with a constant initializer is also usable in constant expressions. A const volatile variable is not usable. Expected: accept for the first, reject for the second.
- M. The operand of the noexcept specifier, and the condition of explicit, need a constant of type bool with no narrowing, so an integer two is rejected. CE-11 lists only array bounds, enumerators and case labels. Expected: reject.

## expressions/new-delete.md

- S. The syntax has no placement arguments in a new expression, and no form where the type is in parentheses. Both are part of the grammar. A user can declare an allocation function with extra parameters and pass them through placement arguments. Expected: accept, run with the user function called.
- M. In a new expression for an array, only the first dimension can be a run-time value. Every later dimension must be a positive constant. Expected: reject.
- M. A new expression for a multi-dimensional array gives a pointer to the first element, which is a pointer to an array. Expected: accept.
- M. A new expression for an array of zero length gives a non-null pointer that is distinct from other live allocations. delete for arrays frees it. Expected: run, non-null.
- M. A new expression for a const object of scalar type needs an initializer. Expected: reject.
- M. A new expression for an array of a class with no default constructor and no initializer is invalid. Expected: reject.
- M. The call of the allocation function is sequenced before the evaluation of the initializer arguments. Expected: run, allocation first.
- M. Class-specific allocation and deallocation functions take precedence, and the global-scope prefix bypasses them. NEW-1 does not cover this. Expected: run, the class function for the plain form and the global function for the prefixed form.
- M. An implementation can omit or merge calls to the replaceable global allocation functions. Expected: unspecified.
- E (NEW-8). C++23 allows delete of a pointer to an incomplete class. The behavior is undefined only when the complete class has a non-trivial destructor or its own deallocation function. Clang and GCC accept. Expected: accept, undefined in the bad case.
- M. delete through a pointer to a base class whose destructor is not virtual, when the object is of a derived class, is undefined. delete for arrays through a base pointer to an array of derived objects is undefined too. Expected: undefined.
- M. delete accepts a pointer to a const object. Expected: accept.
- M. The operand of delete can be a class object with one conversion function to an object pointer type. Expected: accept.

## expressions/lambdas.md

- E (LAM-6). The ban on an explicit capture in the same mode as the default does not cover this. With a reference default, an explicit capture of this is valid. With a copy default, both an explicit capture of this and a copy capture of the current object are valid. Clang and GCC accept. Expected: accept.
- E (LAM-8 and LAM-6). A lambda in a default member initializer can capture this and can have a capture default, though it is not inside a member function. Expected: accept.
- M. A lambda at namespace scope can have an init capture. Expected: accept.
- M. A lambda that appears in a default argument must not capture anything. GCC rejects it. Clang accepts it. Expected: reject.
- M. A parameter of a lambda must not have the same name as one of its explicit captures. Expected: reject.
- M. A non-static data member cannot be captured by name in a simple capture. It needs this or an init capture. Expected: reject.
- M. A use of a member of the current object inside a lambda that has no capture default and does not capture this is invalid. Expected: reject.
- M. A constant variable needs capture once the body uses it in a way that needs its address, such as taking its address or binding a reference to it. Expected: reject without capture.
- M. The initializer of an init capture is looked up in the enclosing scope, so it can use an outer variable with the same name as the capture. Expected: run, the outer value is used.
- M. C++23 makes the captures visible in the trailing return type and in the parameter list. In a non-mutable lambda, the type that decltype gives for a parenthesized copy capture is a reference to const. Expected: accept.
- M. Inside a non-mutable lambda, an object captured by reference can still be changed. Expected: accept.
- M. A closure type with captures has a deleted copy assignment operator. Expected: reject an assignment.
- M. The function pointer conversion of a lambda declared noexcept gives a pointer to a noexcept function. Expected: accept.
- M. A lambda with an explicit object parameter must not be mutable. Expected: reject.
- M. Copy capture of an array copies each element. Copy capture of a reference copies the referred object. Expected: run, later changes to the source are not seen.
- M. A call of a lambda that captured a local by reference, after that local has ended, is undefined. Expected: undefined.
- M. The order of the closure members, the size of the closure type, and whether a reference capture adds a member are unspecified. Expected: unspecified.
- M. With a copy default inside a member function, the current object is captured implicitly by reference through this. This use is deprecated. Expected: accept, with a warning recommended.

## declarations/type-names.md

- E (TYN-1). Repeating an alias of the same type is allowed only outside a class. Inside a class, a second member alias with the same name is a redeclaration, which Clang and GCC reject. Expected: reject.
- E (TYN-2). The words "in any scope" are too broad. An alias in an inner block or namespace with the same name and a different type is valid and hides the outer one. Expected: accept.
- E (TYN-2). An alias can have the same name as a class in the same scope if it names that class. Expected: accept.
- M. A cv-qualifier added through an alias to a reference type or to a function type is ignored, not an error. On an alias of an array, it applies to the element type. Expected: accept.
- M. An alias of a function type cannot be used to define a function. Expected: reject.
- M. An elaborated type specifier in a member declaration of a class, when lookup finds nothing, declares the class in the enclosing namespace, not in the class. Expected: accept a later use at namespace scope.
- M. A scoped enumeration key cannot be used in an elaborated type specifier that only refers to the enumeration. Expected: reject.
- M. A declaration that consists only of a class key and a qualified name is invalid. GCC accepts it. Clang rejects it. Expected: reject.
- M. A function definition with an incomplete class return type is invalid. A deleted function can have an incomplete parameter or return type. Expected: reject for the first, accept for the second.

## declarations/static-assert.md

- E (SA-1). When a message is given and the assertion fails, the diagnostic must include the text of the message. Characters outside the basic character set need not appear. The rule says only that the implementation can show it. Expected: the diagnostic contains the message.

## declarations/inline.md

- E (INL-4). An out-of-class definition marked inline is not limited to const static data members. A non-const static data member declared in the class can be defined inline outside it. Clang and GCC accept. Expected: accept.
- M. If a function or variable with external linkage is declared inline in one translation unit, an inline declaration must be reachable in every translation unit that declares it. Expected: no diagnostic required.
- M. The definitions of one inline function or variable in different translation units must match. A mismatch needs no diagnostic. Expected: no diagnostic required.
- M. A friend declaration marked inline must be a definition, or the function must already be declared inline. Clang and GCC both accept the bad case. Expected: reject.
- M. A constexpr variable at namespace scope is not implicitly inline. Being const, it has internal linkage, so each translation unit has its own object. Expected: run, the addresses differ between translation units.
- M. Implicitly declared special member functions are inline. Expected: accept.

## declarations/alignment.md

- M. ALN-3 covers variables only. The same matching rule applies to classes. A class whose earlier declaration has an alignment specifier must repeat an equal one on its definition. GCC accepts a mismatch. Clang rejects it. Expected: reject.
- M. If declarations of one entity in different translation units have different alignment specifiers, the program is ill-formed with no diagnostic required. Expected: no diagnostic required.
- M. An alignment specifier may appear in a forward declaration of a class. Expected: accept.

## declarations/linkage-specifications.md

- M. A declaration that is directly inside the single-declaration form of a linkage specification must not have a storage class, for example static. Expected: reject.
- M. The braces of a linkage specification do not open a scope. A name declared inside is usable after the closing brace. Expected: accept.

## declarations/exception-specifications.md

- M. A deallocation function declared with no noexcept specifier is non-throwing. Clang treats a direct call of a class-specific one as non-throwing. GCC does not. Expected: accept, run with the noexcept operator giving true.
- M. An implicit default constructor is potentially throwing if a default member initializer contains a potentially-throwing expression. Expected: run, the noexcept operator gives false.
- M. A defaulted comparison operator follows the same rule as defaulted special members. Expected: run, true for members whose comparisons do not throw.
- M. The operand of the noexcept specifier can use the function's parameters. Expected: accept.

## declarations/volatile.md

- M. Access to a volatile object through a glvalue that is not volatile is undefined. Expected: undefined.
- M. VOL-5 omits two deprecated uses. A function parameter of volatile-qualified type and a volatile-qualified return type are deprecated, so an implementation should warn. A pointer to volatile in those places is fine. Expected: accept, with a warning recommended.
- M. A constructor or destructor cannot be declared volatile. A volatile object of class type is created with an ordinary constructor, and it behaves as volatile only after the constructor ends. Expected: reject the qualified constructor, accept the object.
- M. A const volatile variable at namespace scope that is not inline has external linkage. Only const without volatile gives internal linkage. Expected: run, one object shared by all translation units.
