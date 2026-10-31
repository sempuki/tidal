<!--
Gap review summary, 2026-10-31: third review, operator pages.
Written by: a reviewer agent (a source reader under AGENTS.md) that read the
C++23 working draft N4950 and cppreference. It did not write or change any
file in this repository; the writer copied this summary here.
Checked before use: tools/similarity.py against cplusplus/draft n4950
(0 flagged), no code, no section names. Each item is a suggestion to probe,
not a rule.
-->

# Second gap review: binary and assignment operator pages

Written by a reviewer agent acting as a source reader. This review covers nine Tidal pages under spec/expressions: multiplicative, additive, shift, comparison, bitwise, logical, conditional, assignment, and comma. It skips items from the first gap review of 2026-10-30 unless the current rule still looks wrong. Each item has a label. M means a behavior with no rule. E means a possible error in an existing rule, and the item names the rule. S means a structure issue. Each item gives the result a probe should expect. Where I compiled a probe with Clang and GCC in C++23 mode with pedantic errors, the item says what they did.

## expressions/multiplicative

- M1. Floating-point operands and results can be kept at a wider range and precision than their declared type. The type of the expression does not change. A cast or a store by assignment still rounds the value to the declared type. This affects MUL-12, because an intermediate product that is too large for double may still be finite in the wider format. The same freedom applies to addition and subtraction. Expect unspecified (the implementation picks the evaluation format). On x86-64 with SSE, Clang and GCC round every operation to its type.
- M2. The page gives the value of integer division and remainder, but no rule gives the value of a product, or of a floating-point quotient that does not overflow. Expect run: the mathematical product or quotient, rounded for floating types.
- M3. MUL-3 lists pointer, scoped enumeration, and class operands as invalid. It leaves out the null pointer type and pointers to members. Expect reject for a multiplication of the null pointer constant keyword by an integer.

## expressions/additive

- M1. Floating-point addition or subtraction whose exact result is finite but too large for the type is undefined, the same as MUL-12. The additive page has no such rule. Probe a constant initializer that adds the largest double to itself. Expect undefined. Clang accepts it and gives infinity. GCC rejects it as not constant. This matches the split in MUL-12 and question Q-0001.
- M2. ADD-3 says a pointer can be added to an integer. The other operand can also have unscoped enumeration, bool, or character type. A scoped enumeration operand is rejected. Expect accept for an array name plus an unscoped enumerator, and reject for the scoped case. Clang and GCC agree.
- M3. The null pointer type is not a pointer type, so adding an integer to the null pointer keyword is invalid. Expect reject. Clang and GCC agree.
- M4. ADD-14 speaks of pointer arithmetic in general. The same undefined case also covers subtraction of two pointers, where either one has an element type that is not similar to the real array element type. Probe the difference of two base class pointers into an array of derived objects with extra members. Expect undefined.

## expressions/shift

- M1. SHF-3 lists floating-point, pointer, and scoped enumeration operands as invalid. It does not list class types without a usable conversion, the null pointer type, or pointers to members. Expect reject for each.

## expressions/comparison

- E1 (CMP-8). The rule promises that for two unrelated pointers one of less-than and greater-than is true. No such promise exists. When neither pointer is required to compare greater, each of the four relational operators gives its own unspecified result. Expect unspecified, with no link between the less-than and greater-than results.
- E2 (CMP-15). The rule says pointers to two distinct objects are never equal. Two pointers compare equal when they hold the same address, even if they point to different objects. Examples are a struct and its first member, two members of a union, and an empty member that shares its address with a neighbor. Probe a struct whose first member is an int, convert the address of the struct and the address of the member to pointers to const void, and compare them for equality in a constant expression. Expect accept, with the result true. Clang and GCC agree, also for the union case.
- E3 (CMP-13). The rule bans any narrowing conversion for three-way comparison. A conversion from an integer type to a floating-point type is exempt, even when it can lose precision. Probe a three-way comparison of a long long variable with a double literal. Expect accept. Clang and GCC agree.
- E4 (CMP-12). The rule bans three-way comparison of two function pointers only. It is also invalid for two pointers to members, for two values of the null pointer type, and for a function pointer with the null pointer keyword. The first review raised this, and the rule still names only function pointers. Expect reject for each. Clang and GCC agree.
- E5 (CMP-17). The rule says three-way comparison gives less, equal, or greater. For floating-point operands the result type has no value named equal. Equal operands give the value named equivalent. Negative zero and positive zero also give equivalent. Expect accept for a static assertion that the comparison of negative zero with zero gives the equivalent value. Clang and GCC agree.
- M1. The relational operators reject two pointers to members. They also reject two values of the null pointer type. Expect reject. Clang and GCC agree. GCC's message describes the null pointer case as a comparison with integer zero.
- M2. Equality between a pointer to a noexcept function and a pointer to an ordinary function of the same signature is valid. The common type drops the noexcept. Expect accept. Clang and GCC agree.
- M3. The null pointer keyword compares equal to the literal zero. Expect accept, and true in a static assertion. Clang and GCC agree.
- M4. Two non-null pointers to members are equal when they would select the same member of the same subobject. In a class that inherits one base twice through two paths, a pointer to the base member converted along one path and the same pointer converted along the other path are unequal. Expect run: false.
- M5. CMP-7 orders pointers to members of one object by declaration order. Members of zero size, such as an empty member marked with the attribute that removes its unique address, are excluded, and so are members of a union. For those, neither pointer must compare greater, so the result is unspecified. Expect unspecified. Clang and GCC both give a result in a constant expression.
- M6. The ordering in CMP-7 also holds for subobjects. A pointer into one array element compares less than a pointer into a later element, and a pointer into one member compares less than a pointer into a later member. Expect run: true.
- M7. Three-way comparison of pointers to unrelated objects is also unspecified and is not a constant expression. CMP-8 names only the relational operators.
- M8. A comparison between an unscoped enumeration and a floating-point value is deprecated in C++23. CMP-21 covers two enumeration types only. Expect accept with a warning. Clang and GCC both warn.

## expressions/bitwise

- M1. BIT-3 does not list class types without a usable conversion, the null pointer type, or pointers to members as invalid operands. Expect reject for each.

## expressions/logical

- M1. An operand of the null pointer type is valid, because contextual conversion to bool allows it. Its value is false. Expect accept, and run: the logical not of the null pointer keyword is true. Clang and GCC accept. Clang warns.
- M2. A pointer to member is a valid operand. A null member pointer converts to false. Expect accept. Clang and GCC agree.
- S1. The alternative spellings of these operators, such as the words for and, or, and not, and the word forms of the bitwise and compound operators, have no rule anywhere. Tidal has no lexical chapter. These words are always available without a header. Expect accept.

## expressions/conditional

- E1 (COND-3). The first bullet says two lvalues that differ only in cv-qualifiers give an lvalue with the combined qualifiers. An lvalue result needs one operand to bind directly to a reference to the other operand's type. That works only when one qualification includes the other. A const int lvalue and a volatile int lvalue give a prvalue int. Probe taking the address of that conditional. Expect reject. Clang and GCC agree.
- E2 (COND-3). The last bullet covers a class type that converts to another class type, and it does not give the value category. A derived class lvalue and a base class lvalue give an lvalue of the base type. Probe binding a non-const reference to the base type to such a conditional. Expect accept. The conversion step also applies when only one operand has class type. A class with a conversion to int and an int literal give an int prvalue. Expect accept. Clang and GCC agree on both.
- M1. When the operands have different types, at least one is a class, and neither converts to the other, overload resolution over the built-in candidates picks conversions for both. Two unrelated classes that each convert to int give an int prvalue. Expect accept. Clang and GCC agree.
- M2. The literal zero and the null pointer keyword give a result of the null pointer type. Expect accept, with that type. Clang accepts. GCC rejects, saying the operand types differ.
- M3. Two class prvalues of the same type give a prvalue that initializes the target directly. No copy or move is needed. Probe a class with deleted copy and move constructors, and a conditional between two calls that return it by value, used as an initializer. Expect accept. Clang and GCC agree.

## expressions/assignment

- E1 (ASN-2). The rule forbids a prvalue left operand in all cases. For a class type, assignment to a prvalue calls the class's assignment operator and is valid. Probe assigning one temporary of an empty struct to another. Expect accept. Clang and GCC agree.
- M1. ASN-2 lists prvalues but not xvalues. A non-class xvalue, such as a variable cast to an rvalue reference, cannot be assigned to. Expect reject. Clang and GCC agree.
- M2. When the left operand is a bit-field, the result is a bit-field. Taking the address of the assignment is then invalid. Expect reject. Clang and GCC agree.
- M3. When the stored value does not fit in the bit-field, the bit-field gets an implementation-defined value. BF-5 on the unions page makes reduction modulo two to the width a required rule, and the assignment page has no rule for this case. Expect implementation-defined. Clang and GCC both reduce the value and warn.
- M4. A compound assignment counts as one evaluation with respect to a function call that is indeterminately sequenced with it. A call cannot run between the read and the write of the compound assignment. Probe a function call with two arguments, one a compound assignment to a global and the other a call that also changes the global. Expect unspecified (one of the two orders), not undefined.

## expressions/comma

- M1. Places whose grammar asks for a constant expression, such as an array bound or a case label, do not allow a comma operator at the top level. A parenthesized comma is allowed there. Expect reject without parentheses and accept with them. Clang and GCC agree. Both warn that the left operand has no effect.
- M2. COMMA-4 names argument lists and initializer lists. A comma also separates items in template argument lists, declarator lists, base and member initializer lists, and enumerator lists. A comma operator in any of these needs parentheses. Expect the comma to act as a separator.
