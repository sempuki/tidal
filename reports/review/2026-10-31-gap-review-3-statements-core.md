<!--
Gap review summary, 2026-10-31: third review, statements, value categories, conversions, sequencing.
Written by: a reviewer agent (a source reader under AGENTS.md) that read the
C++23 working draft N4950 and cppreference. It did not write or change any
file in this repository; the writer copied this summary here.
Checked before use: tools/similarity.py against cplusplus/draft n4950
(0 flagged), no code, no section names. Each item is a suggestion to probe,
not a rule.
-->

<!--
Gap review summary 3a, 2026-10-30.
Written by: a reviewer agent (a source reader under AGENTS.md) that read the
C++23 working draft N4950 sources. It did not write or change any file in
the Tidal repository. Probes were compiled with Clang 22.1 and GCC 16.2 in
C++23 mode with pedantic errors.
-->

# Tidal second gap review: statements, value categories, conversions, sequencing

This report covers nine Tidal pages: the statement pages for blocks, if, switch, loops, range-based for and jumps, and the expression pages for value categories, conversions and sequencing. It skips every item of the first gap review unless the current rule text still looks wrong. Each item has a label. M means missing behavior. E means a possible error in an existing rule, and it names the Tidal rule. S means structure. Each item also says what a probe should expect. Unless an item says otherwise, Clang 22 and GCC 16 gave the expected result.

---

## statements/blocks

- M1. An expression statement that only names a volatile object performs a read of that object. The read is a side effect and must happen. Expect run: both compilers emit a load of the object, also with optimization on.
- M2. The tie-break in STMT-6 looks at the whole statement. If the beginning of a statement looks like a declaration but the rest cannot belong to one, the statement is an expression. Probe a functional-style conversion of a variable to a class type that has an overloaded arrow operator, followed by an arrow, a member name, and an assignment. Expect accept.
- M3. In C++23 a case label or a default label can also be the last thing in a block, with no statement after it. The grammar on this page lets only identifier labels end a block. Probe a switch whose body has one case label and nothing after it. Expect accept. Clang accepts. GCC 16 rejects it with a parse error, though it accepts an identifier label in that place.

## statements/if

- E1 (IF-7). The list of allowed initializers is too wide. The initializer can be an expression statement, a simple declaration (variables, a class or enumeration definition, a function declaration) or an alias declaration. It cannot be any other declaration that a block allows. A static assertion, a using-directive, a using-declaration, a namespace alias, and a using-enum declaration are all out. Probe an if whose initializer is a static assertion, and another whose initializer is a using-directive. Expect reject.
- S1. The grammar for a condition declaration lists an equals sign followed by an expression, or a braced list without the equals sign. It leaves out an equals sign followed by a braced list. Probe an if and a while whose condition declares an int with an equals sign and a one-element braced list. Expect accept.

## statements/switch

- E1 (SW-5). The rule still says that only a scalar without an initializer can be skipped. JMP-6 now has the wider list: scalars, classes whose default constructor and destructor are both trivial, cv versions of these, and arrays of them. The two rules should agree.
  - Probe a case label that skips the declaration, with no initializer, of a plain struct of two ints. Expect accept.
  - Probe the same skip where the struct has a user-provided destructor. Expect reject. Clang rejects. GCC 16 accepts it without a diagnostic.
- M1. A name that the condition or the initializer of a switch declares must not be declared again in the outermost block of the body. SW-4 points to IF-5 and IF-7 but not to the scope rules of IF-9 and IF-10. Probe a switch whose condition declares a variable that the body block declares again. Expect reject.
- M2. A case label inside the body of a lambda (or of a member function of a local class) that is written inside a switch body does not belong to that switch. The lambda body is a separate function, so the label has no enclosing switch. Probe this. Expect reject.

## statements/loops

- M1. The body of a do statement is its own scope, and the condition comes after that scope ends. A name that the body block declares is not visible in the condition. Probe a do statement whose condition uses a variable declared inside its body block. Expect reject.
- M2. The initializer of a for statement has the same allowed forms as the if initializer. An alias declaration is allowed in C++23, and a static assertion is not. Probe a for whose initializer is an alias declaration. Expect accept. Probe a for whose initializer is a static assertion. Expect reject.
- M3. A name that the initializer of a for declares must not be declared again by a condition declaration of the same for. LOOP-4 only forbids redeclaration in the body. Probe a for whose condition declares the same name as its initializer. Expect reject.
- M4. A variable that the initializer of a for declares is created once and destroyed once, when the for statement ends. This also holds when a break ends the loop. Probe a for whose initializer declares an object with a printing destructor, with a body that prints and then breaks. Expect run: the body output, then one destructor output, then the output of the statement after the loop.

## statements/range-for

- E1 (RFOR-2). The member lookup rule is different from the current text. Since C++20, the loop uses the member form only when the class has a member named begin and also a member named end. Any kind of member counts, data members too. If only one of the two names is a member, the loop uses the free functions that argument-dependent lookup finds. The current text says that a member function with either name forces both to be member functions, and that a data member does not count.
  - Probe a class with a member function begin, no member end, and free begin and end in its namespace. Expect accept.
  - Probe a class with a data member of type int named begin and a member function end, plus free begin and end in its namespace. Expect reject, because the member form is chosen and the int member cannot be called.
  - Probe a class whose begin and end are data members of a callable class type that return pointers into an array. Expect accept, and run visits the elements.
- M1. The range expression is evaluated before the loop variable is declared, so the loop variable's name is not yet in scope there. If the loop variable has the same name as an outer array, the range expression refers to the outer array. Probe a global array of 1, 2 and 3, and a loop whose variable has the same name as the array and adds each element to a sum. Expect accept, and run gives a sum of 6.

## statements/jumps

- E1 (JMP-3). A constructor or a destructor must not return an expression of type void. Only a function with a void return type can do that, and a constructor or destructor has no return type. JMP-3 allows it. Probe a constructor whose body returns a call to a function that returns void. Expect reject.
- M1. A break, continue, or goto inside a lambda body cannot reach a loop, switch, or label of the enclosing function. JMP-7 does not say that a lambda body ends the search for an enclosing loop. Probe a break as the only statement of a lambda body, where the lambda is written inside a while loop. Expect reject.
- M2. JMP-10 covers more names than the text suggests, and fewer than some readers might assume.
  - A local variable or parameter of rvalue reference type also becomes an xvalue when it is returned by name. With a return type of rvalue reference to int and a parameter of the same type returned by name, expect accept.
  - A name in parentheses still counts. Probe returning a local move-only object by name in parentheses. Expect accept.
  - A static local variable does not count. Probe returning a static local move-only object by name. Expect reject.
- M3. JMP-11 says a returned local may or may not be copied. A function parameter is never elided. Returning a by-value parameter by name always runs the move constructor. Probe a class whose copy and move constructors print, passed by value and returned by name. Expect run: one move output and no copy output.

## expressions/value-categories

- E1 (VC-2). The rule forbids assignment to any prvalue. For class types this is wrong. A class prvalue can be the left operand of an assignment, because the class assignment operator is a member function call. Probe a function that returns a plain struct by value, with a braced value assigned to the call result. Expect accept.
- M1. VC-3 misses two xvalue forms. A built-in subscript of an array xvalue or array prvalue is an xvalue. A pointer-to-data-member operator applied to an xvalue object is an xvalue. Probe the declared type of each, with the object turned into an xvalue by a cast to an rvalue reference. Expect accept, with an rvalue reference to int in both cases.
- M2. VC-1 lacks member access on a class lvalue with the dot operator, and member access with the arrow operator. Both give lvalues for data members. Member access to a non-static member function gives a prvalue that can only be called. Probe taking the address of an object's member function through dot access. Expect reject.
- M3. A discarded class prvalue makes a temporary, so its destructor must be usable. Inside the operand of decltype, a function call prvalue makes no temporary, so the destructor is not needed. Probe a call statement to a function that returns a class with a deleted destructor. Expect reject. Probe the same call as the operand of decltype in an alias declaration. Expect accept.
- S1. VC-4 says that a temporary exists only when a reference binds to the prvalue or a member is accessed. VC-5 and VC-8 list more cases, so the word only contradicts them. The rule should point to the other two.
- S2. VC-6 lists only references as a way to make a temporary live longer. RFOR-4 is a second case. The rule should point to it.

## expressions/conversions

- E1 (CONV-5). The rule says that any negative floating value converted to an unsigned type is undefined. The test is on the value after the fraction is dropped. A value between minus one and zero becomes zero, which fits, so the conversion is defined. Probe a constant initialization of an unsigned variable from minus one half, with a static assertion that it is zero. Expect accept.
- M1. A null pointer constant also converts to a null pointer to member of any member type. A character literal with the value zero is not an integer literal, so it is not a null pointer constant. Probe initializing a data member pointer from the literal zero. Expect accept. Probe initializing an int pointer from a character literal with value zero. Expect reject. A literal zero in parentheses is accepted as a null pointer constant by both compilers, which a probe can confirm.
- M2. Qualification conversions through more than one level of pointers are missing. Adding const at a deep level is allowed only when const is also added at every level between it and the top. Probe converting a pointer to pointer to int into a pointer to pointer to const int. Expect reject. Probe converting it into a pointer to const pointer to const int. Expect accept.
- M3. The CONV-10 conversion that drops noexcept also applies to pointers to member functions. Probe converting a pointer to a noexcept member function into a pointer to member function without noexcept. Expect accept. The reverse direction: expect reject.
- M4. CONV-15 does not say when the base-to-derived member pointer conversion is ill-formed. It is rejected when the base is ambiguous, inaccessible, or virtual. Probe converting a pointer to a member of a virtual base into a pointer to member of the derived class. Expect reject.
- M5. CONV-14 does not say what happens to a null pointer. A null derived pointer converts to a null base pointer, also when the base sits at a nonzero offset. Probe a null pointer to a class with two bases, converted to a pointer to the second base, then compared with null. Expect run: equal.
- M6. Bit-field promotion depends on the width of the bit-field and not on its declared type. A bit-field declared with a type wider than int but with a small width promotes to int. An unsigned bit-field as wide as int promotes to unsigned int. A bit-field wider than int does not promote and keeps its declared type. Probe the type of a long long bit-field of width 3 plus zero. Expect int. Probe the same with width 40. Expect long long. CONV-1 only mentions bit-fields narrower than int.

## expressions/sequencing

- E1 (SEQ-4). The rule speaks of a scalar object. The condition is on a memory location, and adjacent bit-fields can share one. Two unsequenced writes to two different bit-fields that share a memory location are undefined. Probe a struct with two adjacent non-zero-width bit-fields, both incremented in one expression with no sequencing between them. Expect undefined.
- M1. A read of a volatile object is a side effect. So two unsequenced reads of the same volatile object, such as the two operands of one addition, are undefined. Expect undefined. Neither compiler warns.
