<!--
Gap review summary, 2026-10-31: classes pages.
Written by: a reviewer agent (a source reader under AGENTS.md) that read the
C++23 working draft N4950 and cppreference. It did not write or change any
file in this repository; the writer copied this summary here.
Checked before use: tools/similarity.py against cplusplus/draft n4950
(0 flagged; the reviewer reworded flagged passages before handing it over),
no code, no section names. Each item is a suggestion to probe, not a rule.
-->

# Review of the classes pages

Role: source reader. This report covers the Tidal pages in the classes chapter: members, access, constructors, initializer-lists, copy-move, derived, virtual, unions, conversion-functions, operators, and comparisons. The index page is not covered. Each item has a label. M marks behavior that no rule covers. E marks a possible error in an existing rule and names that rule. S marks a structural problem. Each item ends with the result that a probe should expect. Where Clang 22 and GCC 16 disagree, the item says so.

## members

1. M. A class can declare a static data member whose type is the class itself, even though the class is still incomplete at that point. The member is then defined after the class. Expected: accept.

2. M. A class must not declare a data member and a member function with the same name. CLS-9 lists only two data members with one name. Expected: reject.

3. M. CLS-9 bans the class name only for data members. The same ban applies to a nested class, a member type alias, an enumerator of an unscoped enumeration declared in the class, an ordinary member function, and a member of an anonymous union in the class. Expected: reject for each.

4. M. Two member functions with one name and identical parameter lists cannot differ only in that one is static. Expected: reject.

5. M. CLS-14 does not say that a member function with an explicit object parameter must not be virtual. Expected: reject.

6. M. Taking the address of a member function that has an explicit object parameter gives an ordinary pointer to function. It does not give a pointer to member. The object goes in as the first argument of a call through that pointer. Expected: accept, and the call runs with the given object.

7. M. A non-static data member must not have thread_local, extern, or register. A static data member can be thread_local. Expected: reject for the non-static cases and accept for the static case.

8. M. A class with no name must not have a static data member. This also applies to a class nested in an unnamed class. Expected: reject.

9. M. A local class must not contain the definition of a friend function. Expected: reject.

10. E (CLS-12). The rule allows only const integer variables of the enclosing function. A local class can also read any enclosing variable that is usable in constant expressions, as long as the read is not an odr-use. For example, a constexpr variable of floating type works. Expected: accept, and the read gives the constant value.

11. M. A name used in a class body must refer to the same declaration when it is looked up again in the completed class. One case is a member declaration that uses a namespace-scope type name, followed by a member type alias with that same name. Expected: no diagnostic required. GCC diagnoses it and Clang accepts.

12. M. CLS-2 lists the places where the class is complete. The operand of a noexcept specifier on a member function is one more such place, so it can use a member that is declared later. Expected: accept.

13. M. The in-class initializer of a const static data member of integer or enumeration type must be a constant expression. Expected: reject when it is not. If the program odr-uses such a member (for example, by binding a reference to it) and the member is not constexpr and has no definition outside the class, the result is no diagnostic required. In practice this is a link failure.

14. M. CLS-5 deals only with const. The same limit applies to volatile, so a member function that is not volatile cannot be called on a volatile object. Expected: reject.

15. E (CLS-10). The sentence about distinct objects having distinct addresses is too strong. An object and its first non-static data member can have the same address. An empty base subobject, or a member marked to need no unique address, can also share an address with another subobject of a different type. Expected: run. In a standard-layout class, the address of the object and the address of its first member compare equal.

16. S. No page in the chapter defines trivial special member functions, trivially copyable classes, standard-layout classes, or implicit-lifetime classes. UNI-3 and UNI-5 depend on whether a member is trivial, and item 15 depends on standard layout. These definitions need a home, probably on this page. One testable consequence is that a standard-layout object and its first member are pointer-interconvertible, so a reinterpret cast between pointers to them gives a usable pointer. Expected: run.

## access

17. M. In a class derived from a class that has a private base, the injected name of that base is a private member. An unqualified use of the base's name there fails. Naming the base through its namespace works. Expected: reject for the unqualified name and accept for the qualified one.

18. M. The friend syntax shows only the form with a class key. A friend declaration can also give a plain type name of a class that is already declared, or a type alias for one. A friend declaration that names a type that is not a class, such as a fundamental type, is ignored. Expected: accept for each.

19. M. A friend declaration that names a member function of another class needs access to that function. Expected: reject when that function is private in its class.

20. M. A member can be reachable through more than one inheritance path, for example a virtual base that is private on one path and public on another. The path that gives the most access applies. Expected: accept for a use from outside the classes.

21. M. Access for the names in a default argument is checked where the function is declared, not at the call. A public member function can have a default argument that names a private static member, and code outside the class can call it without that argument. Expected: accept.

22. M. A local class defined inside a member function has the same access as that member function. It can use the private members of the enclosing class. Expected: accept.

23. M. Access does not affect name lookup or hiding. If a derived class declares a private member function that hides a public base member function with the same name, then a call from outside that would match the base function is still an access error. ACC-3 covers only overloads in one class. Expected: reject.

## constructors

24. E (CTOR-3). The rule deletes the default constructor for any const member whose class has no user-provided default constructor. That is too broad. If every member of that class has a default member initializer, the const member has a value, and the outer default constructor is usable. Expected: accept.

25. E (CTOR-4). The conditions ignore base classes. A class with no data members of its own can have a base that leaves a member without a value. A const object of that class with no initializer is then invalid. Each base has to meet the same condition as the class itself. Expected: reject.

26. M. An implicit or defaulted default constructor is also deleted if a base or member has a destructor that is deleted or not accessible. Expected: reject at a use of the constructor.

27. M. A constructor must not bind a reference member to a temporary object. This covers a member initializer, and a default member initializer that the constructor uses. Expected: reject. Clang rejects it. GCC accepts it with only a warning.

28. M. A default member initializer of a nested class cannot be used before the enclosing class is complete. One case is a default argument of a constructor in the enclosing class that value-initializes the nested class with empty braces. Expected: reject.

29. E (DTOR-3). The rule says a new expression is valid when the destructor is deleted or not accessible. That holds for a single object. A new expression for an array of the class can call the destructor, so it needs a destructor that is usable. Expected: reject. Clang rejects it. GCC accepts it.

30. M. A using-declaration that inherits constructors does not count as a declared constructor, so the class still gets its implicit default constructor. Expected: accept for default initialization of the derived class, if the base has a default constructor.

31. M. A member initializer for a base can call a member function, or read a member of a base, before all bases are initialized. The behavior is then undefined. Expected: undefined.

## initializer-lists

32. E (ILC-3). The rule says copy list initialization must not select an explicit constructor. It does not say that explicit constructors still take part in overload resolution. If an explicit constructor is the best match, the program is invalid. A worse match that is not explicit is not used instead. Copy initialization from a single value with an equals sign is different, because explicit constructors are not candidates there. A probe is a class with an explicit constructor from int and a non-explicit constructor from long, initialized from the int value one. Expected: reject in the braced form and accept in the equals form.

## copy-move

33. M. If a class holds a member declared as an rvalue reference, its implicit copy constructor is deleted. CPY-7 mentions only reference members for assignment. Expected: reject when the program copies the object.

34. M. The implicit copy and move constructors are deleted if a base or member has a destructor that is deleted or not accessible. Expected: reject at a use.

35. M. An assignment operator that has an explicit object parameter and one more parameter of reference-to-class type is a copy (or move) assignment operator. It counts as user-declared, so the class gets no implicit one. Expected: accept, and assignment calls the user function without ambiguity.

36. M. A derived class's implicit copy assignment operator hides every assignment operator of its bases. An assignment that only a base assignment operator could handle, such as one from an int, is not valid on the derived object. Expected: reject.

37. M. A virtual base can be reachable through more than one path. Whether an implicit copy or move assignment assigns that base subobject more than once is unspecified. Expected: unspecified. Clang and GCC both assign it twice.

38. E (CPY-12). The freedom to make a named variable the return object does not apply to a function parameter. A parameter returned by value is always moved into the return object, exactly once. Expected: run, with one move.

39. M. Constant evaluation does not apply the named-return elision. The variable is always moved, so a counter in the move constructor is observable in a static assertion. Expected: run in constant evaluation, with one move.

40. M. If a class is trivially copyable, copying its bytes into another object of the same type gives that object the same value. Expected: run. See item 16 for the missing definition.

## derived

41. M. DER-12 bans a constexpr constructor in a class with a virtual base. A constexpr destructor in such a class is banned too. Expected: reject.

42. M. The constructor of an abstract class does not need to initialize a virtual base, even if that base has no default constructor. An abstract class is never the most derived class. Expected: accept.

43. M. No rule covers a static cast from a pointer or reference to a base, down to a derived class, when the base is not virtual, is accessible, and is unambiguous. Expected: accept. If the object is not actually of the derived class, the behavior is undefined. Expected: undefined, and constant evaluation rejects it.

44. M. A class can be both a direct base and an indirect base of the same derived class. The declaration is valid, though implementations warn. A use of a member of that base through the derived class is ambiguous. Expected: accept for the declaration and reject for the use.

## virtual

45. M. An override and the function it overrides must agree on being deleted. Deleting only one of the two is invalid in either direction. Expected: reject.

46. M. A member function with an explicit object parameter must not have the name and parameters that would make it override a base virtual function. Expected: reject.

47. E (VIRT-15). The rule requires the class in a covariant return type to be complete at the override. One exception applies. The class can be the class that declares the override, which is still incomplete there. Expected: accept.

48. M. A dynamic cast that goes to a base which is not accessible or is ambiguous is invalid at compile time. Expected: reject.

49. M. During construction or destruction, a dynamic cast on the object treats it as an object of the class whose constructor or destructor is running. A cast to a more derived class gives a null pointer. Expected: run, with a null result. If the operand's static type is neither that class nor a base of it, the result is undefined behavior.

50. M. A virtual function that is not pure must be defined, even if the program never calls it. Expected: no diagnostic required. In practice this is a link failure.

51. M. VIRT-8 allows a declaration of a function that returns an abstract class. A call to that function is invalid, because the call creates an object of the abstract class. Expected: reject. GCC rejects it and Clang accepts it.

52. M. If delete for an array destroys derived objects through a pointer to a base, the behavior is undefined, even when the destructor is virtual. Expected: undefined.

## unions

53. E (UNI-1). A union's alignment follows its most strictly aligned member. That member is not always the largest one. A probe is a union of a sixteen-byte character array and a double. Expected: run, with the alignment of a double.

54. M. UNI-3 talks only about a union. The same deletion applies to a class that has an anonymous union member. If a variant member has a non-trivial default constructor, the enclosing class's default constructor is deleted. Expected: reject at a use.

55. M. A union can hold two standard-layout structs whose leading members match in type. A program can read one of those matching leading members through the struct that is not active. Expected: run, and the read gives the value stored through the active struct.

56. E (UNI-5). Assignment to a member makes it active only if the assignment uses the built-in operator or a trivial assignment operator. If the member's type has a trivial default constructor but a user-provided assignment operator, assigning to the member while it is inactive does not make it active. Expected: undefined, and constant evaluation rejects it.

57. E (BF-6). The promotion rule skips a step. If int cannot hold every value of the bit-field but unsigned int can, the bit-field promotes to unsigned int, whatever its declared type. A probe is an unsigned long bit-field of width 32 on a target where int is 32 bits. Expected: accept, and unary plus on it gives unsigned int, not unsigned long.

## conversion-functions

58. M. A conversion function can have an explicit object parameter. Expected: accept, and an implicit conversion runs it.

59. M. Some contexts need a single conversion to an integral or enumeration type, such as a switch condition or an array size in new. A class with two non-explicit conversion functions to different integral types makes these contexts invalid. Expected: reject.

60. M. The operand of delete can have class type if the class has exactly one non-explicit conversion function to a pointer to object type. The operand is converted first. Expected: accept.

61. M. A program can apply the call syntax to a class object whose class can convert, without explicit, to a pointer or reference to function. The call uses that conversion. The call then runs the function that the conversion returns. Expected: run.

## operators

62. E (OPR-7). The list of operators that keep the built-in order of operand evaluation is incomplete. An overloaded logical and, logical or, and pointer-to-member arrow also evaluate the left operand before the right one. Expected: run, with the side effects of the left operand first.

63. E (OPR-10). A literal operator can be declared as a friend inside a class, including a friend definition. Expected: accept.

64. M. If both the numeric form and the raw form of a literal operator exist for a suffix, an integer literal with that suffix uses the numeric form. Expected: run, with the numeric form chosen.

65. M. Adjacent string literals can be joined when they have the same user-defined suffix, or when only one of them has a suffix. If they have two different suffixes, the program is invalid. Expected: accept for the first case and reject for the second.

66. M. The unary address-of operator can be applied to an lvalue of a class that is incomplete at that point, when the complete class overloads that operator. Whether the built-in operator or the overload is used is unspecified. Expected: unspecified.

## comparisons

67. E (DCMP-2). A member's equality operator does not have to return bool for a defaulted equality operator to work. Its result is converted to bool as in a condition. A probe is a member type whose equality operator returns int. Expected: accept, and the defaulted operator is usable.

68. M. A rule added in C++23 is missing. An equality operator is not used for reversed candidates or rewritten not-equal candidates if a matching not-equal operator is declared in the same scope. A probe is a class with member equality and inequality operators that take an int, compared with the int on the left. Expected: reject.

69. M. A defaulted three-way comparison with a deduced return type is deleted if a member's three-way comparison returns a type that is not a comparison category type. Expected: reject at a use.

70. M. A defaulted secondary relational operator is deleted when no usable three-way comparison exists for the class. Expected: reject at a use.

71. M. A defaulted comparison of a class with no bases and no members gives equal. Expected: run. Equality yields true, and the three-way comparison yields the equal value of strong ordering.
