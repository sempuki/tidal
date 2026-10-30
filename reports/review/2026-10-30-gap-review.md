<!--
Gap review summary, 2026-10-30.
Written by: a reviewer agent (a source reader under AGENTS.md) that read the
C++23 working draft N4950 and cppreference. It did not write or change any
file in this repository; the writer copied this summary here.
Checked before use: tools/similarity.py against cplusplus/draft n4950
(0 flagged after one round of rewording by the reviewer), no code, no
section names. Each item is a suggestion to probe, not a rule.
-->

# Tidal gap review against C++23

This report compares Tidal pages with the C++23 reference material. It contains facts and ideas only, in prose. There is no code and no proposed wording. Each item names a behavior and the result a probe should expect: accept, reject, run (with the observable outcome), undefined, unspecified, or implementation-defined.

The drafting pages (unary, postfix, casts, constant-expressions, new-delete, lambdas) were skipped.

Item labels:
- M: missing behavior
- E: possible error in an existing rule
- S: structure

---

## Statements

### statements/blocks

- M1. When a statement could be read as either an expression statement or a declaration, it is a declaration. Probe a block that writes a type name with a parenthesized existing variable name as a statement. Expect run: the statement declares a new local that hides the outer variable, so a later assignment does not change the outer one.
- M2. Automatic block variables are initialized each time control passes through their declaration, and block statics only the first time. Those rules are on the storage page. Consider a cross-reference from STMT-2 or STMT-3, because the block page has no rule for declaration statements as statements.
- S1. The page does not mention the declaration-statement rules for static locals: the retry after an exception and recursive re-entry. These are listed under storage-linkage below.

### statements/if

- E1 (IF-6). The list of forbidden specifiers in a condition declaration is incomplete. The reference allows only type specifiers and constexpr, so typedef, inline, register and mutable are also forbidden, along with static, extern and thread_local. A function declarator is forbidden as well as an array. Probe a condition declaration that uses inline or typedef. Expect reject.
- M1. In a template, the discarded branch of if constexpr is still checked for constructs that do not depend on a template parameter. Probe a template whose discarded branch calls a function name that was never declared and does not depend on the template parameter. Expect reject.
- M2. In a function that is not constexpr or consteval, the then branch of if consteval never executes. Probe a plain function with if consteval that returns different values per branch, called at run time. Expect run: the else value.
- M3. The reference does not define the case of an if consteval inside an if constexpr discarded branch, or the reverse. Low priority.

### statements/switch

- E1 (SW-2). "Exactly one conversion function" is too strict. A class condition is valid when exactly one target type is reachable through non-explicit conversion functions. Two overloads that convert to the same integer type (for example, ones that differ by ref-qualifier or const) are fine. Probe a class with two non-explicit conversion functions to int that differ only by qualifier, used as the condition. Expect accept.
- M1. Case values are converted to the promoted condition type. With a char condition, a case value larger than char can hold is valid and never matches. Probe a switch on a char with a case label of 300 and a default. Expect accept, and run takes the default.
- M2. Case and default labels can appear deep inside nested statements of the body, for example inside a loop (the "Duff's device" shape). Probe a case label placed inside a while loop in the switch body. Expect accept, and run enters the loop body directly.
- M3. In nested switches, each label belongs to the innermost enclosing switch. Probe an inner switch with a case value that also matches the outer condition. Expect run: the outer dispatch ignores the inner label.
- M4. The condition can have bool or scoped enumeration type. Check that SW-2's "enumeration type" explicitly includes scoped ones, and that the case labels then must be of that same scoped type. Probe a scoped enum condition with a plain integer case label. Expect reject.

### statements/loops

- M1. A for statement's condition variable is created and destroyed in each iteration. The destruction happens after the iteration expression runs. Probe a for loop whose condition declares a class object with a printing destructor and whose iteration expression also prints. Expect run: the iteration expression output comes before the destructor output in each iteration.
- M2. The rule against redeclaring in the outermost block of the body also applies to while conditions. LOOP-4 states it only for for. Probe a while whose condition declares a name that the body block declares again. Expect reject.
- M3. The forward-progress assumption. A loop with no observable effects, no volatile access, no atomic or synchronization operation and no termination may be assumed to end, so such a loop is undefined in C++23. The infinite-loop relaxation is C++26-only. This sits near the threads topic, but it affects loop meaning. Probe an empty infinite while(true) loop in main. Expect undefined in C++23.

### statements/range-for

- E1 (RFOR-5). The forbidden specifiers on the loop variable go beyond static and thread_local. The reference allows the same set as a condition declaration (type specifiers and constexpr only), so extern, register, inline and typedef are also out. Probe extern on the loop variable. Expect reject.
- M1. The loop is defined by an equivalent rewritten loop. It holds the range in a forwarding reference, computes begin and end once, compares with inequality, advances with prefix increment, and initializes the loop variable from a dereference. Several observable consequences follow:
  - Probe an iterator class that has only postfix increment. Expect reject.
  - Probe an iterator that has only equality comparison. Expect accept, because inequality is rewritten from equality.
  - Probe a container that grows during iteration, with end computed once. Expect run: the loop stops at the original end.
- M2. The loop variable must not be redeclared in the outermost block of the body. Probe a body block that declares the loop variable's name again. Expect reject.
- M3. A range-based for over an array whose size is not yet known, or whose elements have an incomplete type, is ill-formed. Probe an extern array declared with empty brackets as the range. Expect reject.
- M4. Free begin and end are found only by argument-dependent lookup. Ordinary unqualified lookup is not used. Probe a class in one namespace whose begin and end functions are declared in an unrelated namespace that is visible at the loop. Expect reject.
- M5. Member lookup selects the member form if it finds any member named begin or end, even one that is not a function, such as a data member. Probe a class with a data member called begin plus free begin and end functions. Expect reject.

### statements/jumps

- E1 (JMP-6). "A jump can skip a scalar variable that has no initializer" is too narrow. The permission also covers class types with a trivial default constructor and a trivial destructor, cv-qualified versions of these types, and arrays of them. A class with a trivial constructor but a non-trivial destructor cannot be skipped.
  - Probe a goto that skips a declaration, with no initializer, of a plain struct of two ints. Expect accept.
  - Probe the same with a struct that has a user-provided destructor. Expect reject.
- M1. C++23 implicit move on return. A return statement that names a local variable or parameter treats it as an xvalue.
  - Probe a function that returns a parameter by name, where the parameter has a move-only class type. Expect accept.
  - Probe a function that returns an lvalue reference and returns its own by-value parameter by name. Expect reject in C++23, because the xvalue cannot bind to a non-const lvalue reference.
- M2. Copy elision on return. Returning a prvalue of the result type never calls a copy or move constructor. Returning a named local may or may not elide the copy, and which happens is unspecified. Probe a class whose copy and move constructors print, returned by name from a function. Expect unspecified: zero or one print.
- M3. Jumping into a try block or a handler from outside is ill-formed. Exceptions are not in the charter. Mention this only if the charter widens.
- M4. Constructors and destructors may use return only without an operand. Probe a destructor body that returns a value. Expect reject.

---

## Expressions

### expressions/value-categories

- E1 (VC-3). "A member of an xvalue is an xvalue" is wrong for a reference data member and for a static data member. Both are lvalues even when the object expression is an xvalue or prvalue. Probe a class with an int reference member, created as a prvalue, where the member is used to initialize a non-const lvalue reference. Expect accept.
- E2 (VC-3). A cast to an rvalue reference to a function type gives an lvalue, and so does a call that returns one. Probe taking the address of such a cast expression. Expect accept.
- M1. Temporary materialization also happens in these places:
  - when a class prvalue is a discarded-value expression,
  - when an array prvalue is subscripted or decays,
  - when an initializer list is created.

  Probe a statement that consists only of a prvalue of a class with a printing destructor. Expect run: the destructor runs at the end of that statement.
- M2. An implementation may create extra temporaries when it passes or returns objects of trivially copyable class types (ones with eligible trivial copy/move constructors and a trivial destructor). This permission makes some address comparisons unspecified. Probe a function that stores the address of its by-value trivial parameter and compares it with the caller's object. Expect unspecified.
- M3. A prvalue of cv-qualified non-class type has its cv-qualifiers dropped. Probe decltype of a call to a function declared to return const int. Expect plain int.
- M4. Other prvalues are missing from VC-2: this, enumerators, lambda expressions, and a cast to a non-reference type. Other lvalues are missing from VC-1: a cast to an lvalue reference type, prefix increment of a class type through its result, a comma whose right operand is an lvalue, and a conditional whose operands are both lvalues. These may just need cross-references.
- S1. The general rules for temporaries (a temporary dies when the whole enclosing expression statement or initializer finishes, and later-created ones die first) have no rule here, though the page title mentions temporaries. Lifetime extension lives on the references page.

### expressions/conversions

- M1. Lvalue-to-rvalue, array-to-pointer and function-to-pointer conversions have no rules. Reading an object whose value is indeterminate is undefined in C++23. Copying such a value through unsigned char or std::byte is an exception (see initialization/forms E1).
- M2. Bit-fields promote to int when int can hold all their values. Probe an unsigned bit-field of width 3 holding 1, with 2 subtracted from it. Expect run: the result is negative because the arithmetic is done in int.
- M3. Promotion of unscoped enumerations without a fixed type, of wchar_t, char8_t, char16_t and char32_t (to the first of int, unsigned int, long, and so on that holds all values), and of a fixed-type enum (to its underlying type, then promoted). Probe the sign of a value of an enum with a fixed underlying type of unsigned char after unary plus. Expect int.
- M4. Floating-point promotion from float to double exists. It matters for variadic arguments and overload ranking. CONV-1 only says arithmetic does not apply it.
- M5. Derived-to-base pointer conversion. It is implicit, and it is rejected when the base is ambiguous or inaccessible. Probe converting a pointer to a class with a private base to a pointer to that base, outside the class. Expect reject.
- M6. Pointer-to-member conversions go from base member to derived member, which is the reverse of the object pointer direction, and the null member pointer exists. Probe an implicit conversion of a pointer to a derived member to a pointer to a base member. Expect reject.
- M7. Since C++20, a pointer to an array of known bound converts to a pointer to an array of unknown bound. Probe an implicit conversion between the two. Expect accept.
- M8. The boolean literal false is not a null pointer constant, because it is not an integer literal. Probe initializing a pointer from false. Expect reject.
- M9. Conversions from a pointer to member to bool, with null giving false. Probe a null data member pointer in a bool condition. Expect run: false.
- M10. Conversion rank for the C++23 extended floating-point types. Conversions to a type of lower or unordered rank are not implicit, and arithmetic between types of unordered rank is ill-formed. This is only relevant if the implementation provides those types, so mark it conditionally supported. The glossary's "usual arithmetic conversions" lists only float, double and long double.
- E1 (CONV-11). The reference says that a floating-to-floating conversion of a value outside the destination range is undefined. The implementation-defined choice applies only between two adjacent representable values. Tidal treats infinity as an adjacent value, which is a reading the reference does not clearly support. Keep this open under Q-0028. Probe converting a double far larger than the float maximum to float in a constexpr initializer. Expect: if undefined, rejected during constant evaluation.
- M11. NaN converts to true. CONV-3 lists only zero, negative zero and null. Probe a NaN double used as a condition. Expect run: true.
- M12. An integer to floating conversion of a value outside the floating type's range is undefined. This is rare with standard types, but it applies to any wide extended integer type.

### expressions/sequencing

- M1. Since C++17, overloaded operators follow the sequencing of the built-in operator they replace. This covers shifts, assignment, compound assignment, subscript, comma, and the pointer-to-member operators. Probe a class with an overloaded shift operator chained with two calls that record their order. Expect run: left to right.
- M2. More fixed orders exist than SEQ-2 and SEQ-3 list:
  - The left operand of the pointer-to-member operators is sequenced first.
  - Compound assignment sequences the right operand before the left.
  - The elements of a braced list are sequenced left to right (LIST-7 says this, but it should be cross-referenced here).

  Probe a braced initialization of a two-int aggregate from two increments of one variable. Expect run: defined values in order.
- M3. A call's function body is indeterminately sequenced with other evaluations in the caller. The body does not interleave with them. Probe two calls in one addition that each increment the same global. Expect run: defined, and the order is unspecified.
- M4. Each parameter's initialization, with all its side effects, is indeterminately sequenced with the others. SEQ-1 says this for arguments, but parameter destruction timing is missing (see functions/definitions M2).
- M5. Default arguments and the destruction of temporaries are part of the call's full-expression. Probe a default argument that creates a temporary with a printing destructor. Expect run: destroyed at the end of the calling statement.

### expressions/multiplicative

- M1. In C++20 and C++23, arithmetic between an enumeration and a floating-point type, or between two different enumeration types, is deprecated. In C++26 it is ill-formed. MUL-2's evidence notes the warning. Consider a recommended-practice rule.
- M2. Nothing more is missing for built-in types. The rules match the reference.

### expressions/additive

- M1. Pointer arithmetic is undefined when the pointer's static element type differs from the actual array element type, apart from cv. An example is a pointer to base used to step through an array of derived objects. Probe adding one to a pointer to base that points into an array of derived objects with extra members. Expect undefined.
- M2. A multidimensional array's inner arrays are separate arrays for pointer arithmetic. Moving past the end of one row is undefined even though the storage continues. Probe adding 3 to a pointer to the first element of row 0 of a 2 by 2 array, then reading through it. Expect undefined.
- M3. A pointer difference whose value does not fit in the result type is undefined.
- M4. As in multiplicative M1, enum with float and mixed enum operands are deprecated in C++23 and ill-formed in C++26.

### expressions/shift

- M1. Overloaded shift operators keep left-to-right order. This belongs in sequencing M1.
- M2. Nothing else is missing for the built-in operators. The C++20 two's complement semantics in SHF-5 and SHF-6 match the reference.

### expressions/comparison

- E1 (CMP-12). Three-way comparison is ill-formed when the composite type is a function pointer, a pointer to member, or the null pointer type. Tidal says pointer operands give strong ordering without that exception. Probe three-way comparison of two function pointers. Expect reject.
- E2 (CMP-11). Comparing two arrays with equality or relational operators is deprecated in C++20 and C++23 and ill-formed in C++26. Three-way comparison of two arrays is already ill-formed in C++20. Probe three-way comparison of two int arrays. Expect reject.
- M1. Three-way comparison of two different unscoped enumeration types is ill-formed, and so is comparing an unscoped enumeration with a floating-point value. An unscoped enumeration with an integer is fine. Probe three-way comparison of enumerators from two different unscoped enums. Expect reject.
- M2. The composite pointer type allows multi-level qualification mixing. A pointer to pointer to int compares equal-able with a pointer to pointer to const int, because the composite type adds const at every level. Probe an equality comparison of those two types. Expect accept.
- M3. Derived and base pointers compare after derived-to-base conversion. Probe equality of a derived object's address and a base pointer to it. Expect run: true.
- M4. A relational operator with a nullptr operand is ill-formed, just as with the literal 0. Probe a less-than comparison of an object pointer with nullptr. Expect reject.
- M5. When a pointer just past one object is compared for equality with a pointer to some other object that sits right after it in memory, the outcome can be either true or false. The result is unspecified, and such a comparison is not a constant expression. Probe the address after one local int compared with the address of another local int. Expect unspecified.
- M6. Two values of the null pointer type compare equal. Pointer-to-member equality also exists: null member pointers compare equal, and pointers to virtual member functions compare unspecified. Probe two nullptr values compared for equality in a static assertion. Expect accept.
- M7. C++20 rewritten and reversed candidates. Inequality uses equality. A reversed equality with the class on the right is found. Relational operators use three-way comparison. This is overload resolution, but it decides whether comparison expressions on classes are valid. Probe a class with only a member equality operator, compared with an int on the left. Expect accept.
- M8. In C++20 and C++23, comparing two different unscoped enumeration types with the relational and equality operators is deprecated, and in C++26 it is ill-formed.

### expressions/bitwise

- E1 (BIT-2). Bitwise operators between two different enumeration types are deprecated in C++20 and C++23 and ill-formed in C++26. The rule says this is accepted, which is right for C++23, but it should record the deprecation, perhaps as a recommended warning.

### expressions/logical

- M1. Overloaded && and || lose short-circuit evaluation. Since C++17, they still evaluate the left operand before the right. The introduction says this, but no rule does. Probe an overloaded && whose right operand has a side effect, with a left operand that is false. Expect run: the side effect happens.

### expressions/conditional

- M1. When both the second and third operands are xvalues of the same type, the result is an xvalue. When one is an lvalue and the other an xvalue of the same type, the result is a prvalue. Probe binding an rvalue reference to a conditional whose operands are two casts to rvalue reference. Expect accept.
- M2. With an lvalue and a prvalue of the same class type, the result is a prvalue copy-initialized from the selected operand. Probe this with a class whose copy constructor is deleted. Expect reject.
- M3. If either operand is a bit-field lvalue and the result is an lvalue, the result is a bit-field. Taking its address is then rejected. Probe the address of a conditional between two bit-field members. Expect reject.
- M4. The second operand can be a comma expression without parentheses. Probe this. Expect accept.
- E1 (COND-3). "Both are pointers or null pointer constants" gives a pointer result. That is false when both are the integer literal 0 (the result is int), and false when both are nullptr (the result is the null pointer type). Probe decltype of a conditional with two literal zeros. Expect int.
- M5. Composite types also apply to pointers to members. Probe a conditional between a member pointer and nullptr. Expect accept, with a member pointer type.
- M6. The condition is fully evaluated, with its side effects, before the selected operand. COND-2 says "first" but does not say "with side effects".

### expressions/assignment

- E1 (ASN-6). The ban on a braced list after a compound assignment cannot cover class types. For a class, the braced list is passed to the selected operator function, and compound operators work this way as well. Probe a class with a member += that takes the class type and has a two-int constructor, used with a braced pair. Expect accept. The open Clang/GCC split concerns scalars only.
- M1. Assigning from an object that partly overlaps the target is undefined. Exact overlap with the same type is fine. Probe two union members where one struct sits at an offset inside the other, assigning one to the other. Expect undefined.
- M2. Deprecation of volatile in C++20 and C++23. Using the result of an assignment to a volatile object is deprecated (for example, chaining through a volatile). The arithmetic compound assignments on volatile are deprecated, and C++23 restored the bitwise ones. ASN-7 presents all of these as plain required. Consider a recommended warning.
- M3. A compound assignment with a bool left operand converts the arithmetic result back to bool. Probe adding 2 to a false bool with +=. Expect run: true.
- M4. Since C++17, assignment through an overloaded operator= also evaluates the right operand first. Cross-reference this with sequencing M1.

### expressions/comma

- M1. An overloaded comma also evaluates left before right since C++17. Probe this with a printing operator. Expect run: left first.
- M2. A bit-field on the right side makes the whole comma result a bit-field. A class prvalue on the left side becomes a real temporary object, which lives until the enclosing statement or initializer is done.

---

## Declarations

### declarations/auto

- M1. A later declarator in the same declaration can use a name declared by an earlier declarator. Probe one auto declaration where the second declarator is a pointer initialized with the address of the first. Expect accept.
- M2. A non-static data member cannot use auto. This is in CLS-9, so it only needs a cross-reference.
- M3. A function whose return type has not been deduced yet cannot be the initializer of an auto variable. This is covered on the return-deduction page, so cross-reference RET-3.
- M4. Placeholder deduction for a static data member with an in-class initializer works for inline or constexpr members. Probe an inline static auto member with an initializer. Expect accept.

### declarations/storage-linkage

- M1. If a block-scope static variable's initialization exits by an exception, the variable is not initialized. The next time control reaches the declaration, it tries again. Probe an initializer that throws on the first call and succeeds on the second. Expect run: the second call initializes it. This depends on exceptions, which are out of the charter, so the probe may need to wait.
- M2. A recursive call that reaches a static local's declaration again before its first initialization has finished is undefined. Probe a function whose static local's initializer calls the same function. Expect undefined.
- M3. Concurrent first passes through a static local's declaration are synchronized. Other threads wait. This refers to the threads topic.
- M4. Objects with static storage are torn down when the program ends, last-constructed first, where "constructed" means the moment the constructor finished. This includes block statics that were reached. Probe two namespace-scope objects and one block static, all with printing destructors. Expect run: reverse order of construction.
- M5. A block-scope extern declaration takes the linkage of a visible earlier namespace-scope declaration, including internal linkage. Probe a namespace-scope static int and a function that declares the same name extern inside a block and reads it. Expect run: it reads the internal variable.
- M6. An anonymous union at namespace scope must be declared static, unless it is in an unnamed namespace. Probe an anonymous union at global scope without static. Expect reject.
- M7. LNK-3 is missing a case. A const variable previously declared extern keeps external linkage. Probe an extern const declaration, then a const definition in the same unit, used from another unit. Expect run: one entity.

### declarations/declarators

- M1. Pointer-to-member declarators have no rule. A pointer to a member of reference type is ill-formed, and so is a pointer to a member of void type. Probe forming a pointer to a reference data member. Expect reject.
- M2. The function-declaration ambiguity goes beyond empty parentheses. A declarator whose parenthesized parts can be read as parameter declarations declares a function. Probe a variable declaration whose initializer in parentheses is a type name followed by empty parentheses, then use it as an object. Expect reject.
- M3. Redundant parentheses around a declarator name are allowed. Probe a variable declared with its name in parentheses and initialized. Expect accept.
- M4. A non-member function, or a static member function, must not have a ref-qualifier. DCL-7 mentions only cv. Probe a free function declared with a trailing & qualifier. Expect reject.
- M5. The exception specification is part of the function type. Redeclaring a function with a different noexcept is ill-formed. Probe a function declared noexcept and then redeclared without it. Expect reject.
- M6. Since C++20, a volatile-qualified return type, a volatile-qualified parameter type, and volatile on a structured binding are deprecated.
- M7. Accessing a volatile object through a non-volatile glvalue is undefined. Probe reading a volatile int through a pointer cast to remove volatile. Expect undefined.
- E1 (DCL-3). "A reference variable must have an initializer, except in an extern declaration" also misses function parameters, the return type, and a reference data member declared in a class definition. Probe a function declared with a reference parameter. Expect accept.
- E2 (DCL-5). The bound is a converted constant expression of the size type. A class type with a constexpr conversion function to an integer type is therefore valid. A scoped enumeration is not. Probe an array whose bound is a constexpr object of a class with a constexpr conversion to int. Expect accept.
- E3 (DCL-7). A trailing return type requires the leading type to be the plain placeholder auto. Probe a declaration that gives int both at the start and after the arrow. Expect reject.
- M8. A static data member declaration in a class can have an array type of unknown bound, completed by the later definition. Probe this. Expect accept.

### declarations/structured-bindings

- E1 (SB-3). The data members need to be accessible where the declaration appears. Being public is not required, so a member function or a friend can bind private members. Probe a friend function that decomposes an object of a class with two private int members. Expect accept.
- E2 (SB-4). "Never a reference type" is wrong when the data member itself has reference type. decltype of the binding then gives that reference type. Probe a class with an int reference member, decomposed, with decltype of the binding used to declare a variable that has no initializer. Expect reject, because a reference needs an initializer.
- M1. A mutable member stays modifiable through a binding even when the hidden variable is const. Probe a const decomposition of a class with a mutable int, then an assignment to that binding. Expect accept.
- M2. Binding to a bit-field member is allowed, and the binding behaves as that bit-field. Taking its address is rejected. Probe a reference decomposition of a struct with one bit-field, followed by an assignment, then taking the address. Expect accept for the assignment and reject for the address.
- M3. In the tuple protocol, the member form of get is chosen only if the class has a get member template that takes a value (not a type) as its leading template argument. Otherwise argument-dependent lookup is used. Each binding is a reference variable, lvalue or rvalue depending on the initializer's category, and the get calls happen in order.
- M4. If std::tuple_size of the type is complete but has no value member, the declaration falls back to data members. This needs library declarations, so it may be out of reach without headers.
- M5. Since C++20, a lambda can capture a structured binding. This crosses into lambdas, which are being drafted.

### declarations/enumerations

- M1. Before the closing brace, each enumerator has an integer type: the underlying type if fixed, otherwise the type of its initializer. So inside a scoped enum's definition, an enumerator can be used in arithmetic to define the next one. Probe a scoped enum whose second enumerator's value is the first plus one. Expect accept.
- M2. The list-initialization in ENUM-8 must be direct-list. Copy-list-initialization from an integer is rejected. Probe initializing a fixed-type enum with an equals sign and a braced integer. Expect reject.
- M3. A using enum declaration needs the enumeration's enumerators to be known. An opaque declaration alone is not enough. Probe an opaque scoped enum declaration followed by using enum on it. Expect reject.
- M4. Since C++20, a using declaration can name a single enumerator of a scoped enum. Probe this, then use the enumerator unqualified. Expect accept.
- M5. A fixed underlying type can be bool or a character type, and cv-qualifiers on it are ignored. Probe an enum whose fixed type is bool and that has two enumerators. Expect accept.
- M6. An enumeration with no enumerators and no fixed type behaves as if it had one enumerator with value 0 for its range. Converting 1 to it is in range, because the range covers one bit. Probe a constexpr conversion of 1 to such an empty enum. Expect accept.

### declarations/namespaces

- M1. A using directive makes the members appear, for lookup, as though they lived in the innermost namespace containing both the directive's location and the named namespace. A global variable with the same name therefore causes an ambiguity, and is not hidden. Probe a global int, a namespace with an int of the same name, and a function that has a using directive for the namespace and reads the name. Expect reject (ambiguous).
- M2. A name declared both in an inline namespace and in its enclosing namespace makes qualified lookup through the enclosing namespace ambiguous. Probe qualified access through the outer namespace. Expect reject.
- M3. A name declared in an unnamed namespace and also at the enclosing scope is ambiguous when used unqualified. Probe this. Expect reject.
- M4. The first name of a nested namespace definition cannot be marked inline. Probe inline before the namespace keyword in a nested definition. Expect reject.
- M5. A using declaration of an enumerator (see enumerations M4) is missing here too.

### declarations/attributes

- M1. likely and unlikely must not both appear on the same statement or label. Probe this. Expect reject.
- M2. nodiscard on a constructor warns when an object created by that constructor through an explicit conversion or a functional cast is discarded. Probe a class with a nodiscard constructor, created as a discarded temporary. Expect a recommended warning.
- M3. noreturn must be on the first declaration of a function, if it is on any declaration. Probe a function declared once without the attribute and then with it. Expect reject.
- M4. The expression in assume must be convertible to bool contextually. It is unevaluated, but it is still checked and odr-uses what it names. Probe an assumption with a class operand that has no bool conversion. Expect reject.
- M5. deprecated and maybe_unused apply to more entities: enumerators, namespaces, type aliases, structured bindings (maybe_unused only), and template specializations. Probe deprecated on an enumerator and then a use of it. Expect a recommended warning.
- M6. An attribute that appertains to the wrong entity is ill-formed for every standard attribute. ATTR-2 covers only some. Probe nodiscard on a variable. Expect reject (Clang and GCC may only warn, so record the split).
- M7. The no_unique_address member can share its address with another member of a different type. Two members of the same empty type still get distinct addresses. Probe two such members of the same empty type compared by address. Expect run: not equal.

### declarations/constexpr

- E1 (CX-2). The condition is wider than "static storage". A constexpr pointer or reference may also refer to a function, or point one past the end of a static object. A constexpr reference or pointer declared as a static local may refer to another static local. In C++23, a constexpr pointer at block scope pointing to an automatic variable is still rejected. Probe a constexpr pointer to one past the end of a global array. Expect accept.
- M1. In C++23, a constexpr function no longer has to be able to produce a constant expression (no diagnostic is needed), and its parameter and return types do not have to be literal. CX-3 states the first part. Check whether the tests also cover non-literal return types.
- M2. A consteval function is implicitly inline, as a constexpr one is. Probe a consteval function defined identically in two translation units. Expect accept.
- M3. C++23 immediate-escalation: a constexpr function that is a template specialization, or a lambda, which calls a consteval function with non-constant arguments, itself becomes immediate. Templates are out of scope, but the effect on calls is visible. Probe a function template that forwards its argument to a consteval function, called with a constant. Expect accept.
- M4. constinit cannot apply to a reference that binds to a temporary with automatic lifetime. constinit also lets an initializer depend on other constant-initialized variables. Low priority.
- M5. Placing a constexpr variable's declaration in an if consteval branch or in a discarded statement. Low priority.
- M6. The page has no rule on what may appear in a constexpr function body in C++23. Since C++23, static and thread_local variables and goto are allowed in the body but not evaluated (CX-3 covers this). Labels are allowed. A try block is allowed, but throwing is not constant. This is fine. Cross-reference the constant-expressions draft.

### declarations/functions/definitions

- M1. A function definition cannot appear at block scope. Probe a function defined inside another function's body. Expect reject.
- M2. Whether a parameter is destroyed when the called function returns, or at the end of the caller's full-expression, is implementation-defined. Probe a by-value class parameter with a printing destructor, plus a temporary in the same caller statement. Expect implementation-defined order.
- M3. main must not have a deduced return type, must not be a coroutine, and must not be deleted (DEL-2 covers deleted). Probe main declared with auto and returning 0. Expect reject.
- M4. Functions with an explicit object parameter (C++23 "deducing this") have no rule anywhere. This belongs to the members page, but it is a declarator form.
- M5. Returning from main runs the destruction of objects with static storage. This links to storage-linkage M4.

### declarations/functions/default-arguments

- E1 (DEFARG-4). A function parameter may appear in a default argument when it is in an unevaluated operand. Probe a two-parameter function whose second default is the size of the first parameter. Expect accept. Also, this must not appear in a member function's default argument. Probe a member whose default argument is this. Expect reject.
- E2 (DEFARG-1). The "later parameters must have defaults" check counts defaults from earlier declarations. A later declaration can give a default to an earlier parameter when the following ones already have defaults. Probe a function declared with a default on its last parameter, then redeclared with a default on the first parameter only. Expect accept.
- M1. Names in a default argument are bound where the declaration appears, not at the call. Probe a default that names a global, called from a scope that declares a local with the same name. Expect run: the global's value is used.
- M2. Overloaded operator functions other than the function call operator cannot have default arguments. Probe a binary operator with a default on its second parameter. Expect reject.
- M3. A virtual function call uses the default argument from the static type of the object expression. This belongs to classes.
- M4. A default argument for a function parameter pack is ill-formed. That pack belongs to templates, so this is low priority.

### declarations/functions/deleted

- E1 (DEL-4). A defaulted comparison operator can be a friend that is not a member, declared in the class. "A function that is not a class member must not use it" contradicts this. Probe a class with a friend equality operator that is defaulted in the class and takes const references. Expect accept.
- M1. A deleted function is implicitly inline, so the same deleted definition can appear in several translation units.

### declarations/functions/return-deduction

- M1. A virtual function cannot have a deduced return type. Probe a virtual member returning auto. Expect reject.
- M2. decltype(auto) that returns a parenthesized local returns a reference to the local. Using the result is undefined. Probe a function that returns a parenthesized local by decltype(auto), with the result read. Expect undefined (compilers warn).
- M3. A redeclaration with the deduced type written out is ill-formed. RET-4 covers this, so confirm the test includes a later declaration that writes the concrete type.
- M4. A function with a deduced return type can be declared and used after its definition in another declaration. A function declared with auto, then defined, then used is fine.

### declarations/initialization/forms

- E1 (INIT-1). Reading an indeterminate value is undefined except in a few cases. Initializing an unsigned char, a plain char that is unsigned, or a std::byte object from an indeterminate value of the same kind is not undefined, and the indeterminate value propagates. Probe copying an uninitialized unsigned char into another unsigned char and never using the copy further. Expect run: not undefined. (C++26 replaces this area with erroneous behavior.)
- M1. A variable's own name is in scope in its initializer. Initializing an automatic int from itself reads an indeterminate value. Probe this. Expect undefined.
- M2. Parenthesized aggregate initialization (C++20) does not do brace elision, does not allow designators, and does not extend the lifetime of temporaries bound to reference members. Probe a parenthesized initialization of an aggregate holding an inner two-int aggregate, giving three flat values. Expect reject.
- M3. Default-initialization of a const object of class type needs a user-provided default constructor or initializers for all members ("const-default-constructible"). This is noted as classes (referenced).

### declarations/initialization/list

- E1 (LIST-8). "If it has none, it is set to zero" is wrong for some members. A member without a value is copy-initialized from an empty list. A member of class type gets its default constructor, which may set non-zero values. A reference member without a default initializer makes the program ill-formed.
  - Probe an aggregate whose second member is a class with a default constructor that sets 7, initialized with one value. Expect run: 7.
  - Probe an aggregate with a reference member left out. Expect reject.
- E2 (page intro). The aggregate definition is incomplete. Since C++17, an aggregate may have public non-virtual base classes, which are initialized first from the list. It also must have no virtual functions, no private or protected direct non-static data members, and no user-declared or inherited constructors (C++20). Probe an aggregate with one public base, both holding an int, initialized with two values. Expect run: the base gets the first value.
- M1. Brace-initializing an aggregate from a single object of the same class type copies that object. It does not use that object as the first member's initializer. Probe this with a class that has one member of a type implicitly constructible from the class. Expect run: a copy.
- M2. Designated initialization of a union names exactly one member. Without designators, only the first member is initialized. Probe a union initialized with a designator for its second member. Expect accept, and that member is active.
- M3. A bit-field source whose width means every value fits is not narrowing, by a defect report applied to C++23. Probe list-initializing a char from an unsigned bit-field of width 4. Expect accept.
- M4. A pointer to member to bool conversion is also narrowing in braces. Probe brace-initializing a bool from a data member pointer. Expect reject.
- M5. An enumeration source without a fixed type counts as an integer source for narrowing. Probe brace-initializing an unsigned char from an unscoped enum value that is not a constant. Expect reject when the enum's range exceeds unsigned char.
- S1. List-initialization of classes with constructors (initializer-list constructors preferred, empty braces giving value-initialization) is only referenced. No classes page exists yet, so this is a gap between this page and classes.

### declarations/initialization/references

- E1 (REF-2). A const lvalue reference that is also volatile cannot bind to an rvalue or a converted temporary. Only a non-volatile const lvalue reference can. Probe a const volatile int reference initialized from the literal 1. Expect reject.
- E2 (REF-3). An rvalue reference to a function type can bind to a function lvalue. Probe an rvalue reference to function initialized with a function name. Expect accept.
- E3 (REF-5). "Only a direct binding to a variable extends the life" leaves out two cases. A reference member of an aggregate initialized with braces also extends the temporary's lifetime to that of the aggregate. The parenthesized form does not. Probe an aggregate with a const int reference member, brace-initialized from a literal, then read later. Expect run: defined.
- M1. A non-const lvalue reference cannot bind to a bit-field. A const lvalue reference binds to a temporary copy instead. Probe a non-const int reference to a bit-field member. Expect reject.
- M2. A reference can bind directly to a base class subobject of an lvalue of derived type. Binding an rvalue reference to an lvalue of derived type is rejected (reference-related types). Probe an rvalue reference to base initialized from a derived lvalue. Expect reject.
- M3. Lifetime extension passes through parentheses, the selected conditional operand, the right operand of comma, member access, array subscript of an array temporary, and casts without user conversion. Probe a const reference bound to a member of a class prvalue. Expect run: the whole temporary lives.
- M4. A temporary bound to a reference member in a constructor's member initializer list makes the program ill-formed. This belongs to classes.
- M5. A temporary bound to a reference with static storage lives until program end. Probe a namespace-scope const reference to a class temporary with a printing destructor. Expect run: destroyed at exit.

### declarations/initialization/static

- E1 (SINIT-2). "Dynamically initialized before main runs" is too strong. The implementation chooses whether this is done ahead of main's body or postponed until something from that translation unit is first used. Probe a namespace-scope object whose constructor prints, with main printing first. Expect implementation-defined order. Clang and GCC initialize before main.
- M1. Inline variables are partially ordered. Static data members of class templates and variable template specializations are unordered. Probe two inline variables in one unit where the second reads the first. Expect run: ordered within that unit.
- M2. Destruction order of static-storage objects (see storage-linkage M4).

---

## Classes

### classes/members

- E1 (CLS-10). In C++23, the address order of non-static data members does not depend on access. It excludes members of zero size and variant members. "With the same access" is the older rule, and it also conflicts with CMP-7. Probe a class with a public member, then a private member, comparing their addresses in a member function. Expect run: increasing.
- E2 (CLS-9). An anonymous union is an unnamed member of class type, and it is valid. Anonymous structs are a compiler extension. Probe a struct that contains an anonymous union with two members. Expect accept.
- E3 (CLS-9). A static data member, member function, nested type, member template, member enumerator, or anonymous union member must never have the class's name. The constructor condition applies only to non-static data members. Probe a static data member named after its class. Expect reject.
- E4 (CLS-12). Local classes may name automatic variables in unevaluated operands. They may also use a const integer local with a constant initializer, provided it is not odr-used. Probe a local class member function that returns a const int local of the enclosing function initialized with 5. Expect accept.
- E5 (CLS-3). A const static data member of enumeration type can also have an in-class initializer, as integer types can. Probe this. Expect accept.
- M1. Default arguments, exception specifications, and constructor member initializers are also places where the class is complete. Probe a member function whose default argument names a static member declared later. Expect accept.
- M2. C++23 explicit object parameters. A member function may declare its first parameter as the object, written with the this keyword. Such a function has no implicit this and cannot be static, virtual, or cv- or ref-qualified. Probe a member function with an explicit object parameter called on an rvalue. Expect accept and run.
- M3. C++23 allows a static operator() and a static operator[]. C++23 also allows operator[] with several parameters, which links to COMMA-6. Probe a class with a static function call operator. Expect accept.
- M4. A static member function cannot have a ref-qualifier and cannot be virtual. Probe a static member with a trailing & qualifier. Expect reject.
- M5. A default member initializer must use braces or the equals form. Parentheses are read as a function declaration. Probe a member initialized with parentheses. Expect reject.
- M6. A member function cannot be declared twice in a class body. Probe this. Expect reject.
- M7. mutable cannot apply to a const member, a reference member, or a static member. Probe a mutable reference member. Expect reject.
- M8. A nested class is a member, so it can access the enclosing class's private members. Probe a nested class's member function reading the outer class's private static member. Expect accept.
- M9. A local class cannot have member templates, and its member functions must be defined inside the class. Probe a local class member function defined outside the local class. Expect reject.
- M10. In a class, the injected-class-name refers to the class itself. Low priority.

---

## Glossary

- E1 ("usual arithmetic conversions"). Step 1 orders only float, double and long double. C++23 uses floating-point conversion rank with extended types, and two types of unordered rank make the program ill-formed. Mixed-enumeration and enum/float operands are deprecated. Also check whether the entry handles a long and an unsigned int of the same width, which gives unsigned long.
- E2 ("integer promotion"). The entry says types narrower than int go to int or unsigned int. The C++ rule for wchar_t, char8_t, char16_t and char32_t can promote to long or larger. Check the wording.

---

## Whole topics in charter scope with no page yet

Excluding the pages being drafted (unary, postfix, casts, constant-expressions, new-delete, lambdas), these have no page:

Expressions:
- Primary expressions: names and id-expressions, this, literals as expressions, and parenthesized expressions.
- The pointer-to-member operators (.* and ->*). They are their own precedence level.
- Discarded-value expressions and unevaluated operands as general concepts.
- The decltype specifier rules. These are referenced from AUTO-10 but not specified.
- Temporaries in general: when they are created, the order they die in, and what counts as the enclosing complete expression.
- throw expressions (if exceptions stay out, record that as excluded).

Declarations:
- typedef and alias declarations.
- static_assert declarations (including the narrowing-to-bool rule shared with if constexpr).
- The inline specifier as such (only LNK-4 and CX-4 mention it).
- alignas and alignment.
- Linkage specifications (extern "C"), which are referenced only.
- Elaborated type specifiers and forward class declarations.
- The explicit specifier, including C++20's conditional explicit.
- noexcept as part of function types.
- Pointer-to-member declarators.
- volatile semantics.

Classes (the charter lists all of these, but only members exists):
- Access control and friends.
- Constructors, including member initializer order and delegating constructors.
- Destructors.
- Copy and move special members, and when they are implicitly declared or deleted.
- Derived classes and base subobject layout.
- Virtual functions and abstract classes.
- Unions, including active members and variant members.
- Bit-fields (width, signedness, and storing values that do not fit).
- Conversion functions.
- Operator overloading.
- Defaulted comparisons.
- Initialization of classes by constructors, and initializer-list constructors.

Statements:
- No page lists the C++23 forward-progress rule (loops M3).
- Declaration statements have no dedicated rules (blocks M2).
