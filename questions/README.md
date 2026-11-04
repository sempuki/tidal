# Open questions

Every question the project raises gets a file here: classification calls,
compiler divergences, process choices. Anyone can weigh in by opening a pull
request.

The specification never waits on a question. Each question has a **provisional
answer**, and the specification follows it until a pull request changes it.

## Index

`tools/question_index.py` builds this table from each file's title, its
status, and the first sentence of its answer section.

| ID | Question | Status | Provisional answer |
|---|---|---|---|
| [Q-0001](Q-0001-floating-overflow-constant-evaluation.md) | Is floating overflow during constant evaluation undefined or implementation-defined? | provisional | Undefined |
| [Q-0002](Q-0002-edg-floating-division-by-zero.md) | Does EDG accepting floating division by zero during constant evaluation change CE-28? | compiler defect | No; likely EDG defect |
| [Q-0003](Q-0003-edg-int-min-divided-by-minus-one.md) | Does EDG accepting `INT_MIN / -1` during constant evaluation change MUL-10? | compiler defect | No; EDG's result is likely a defect, for both operators |
| [Q-0004](Q-0004-floating-division-by-zero-at-run-time.md) | Should floating division by zero be implementation-defined on IEEE 754 targets? | settled | Yes: the IEEE 754 result where the type follows IEEE 754 (MUL-11, MUL-20), undefined otherwise; constant evaluation rejects it (CE-28) |
| [Q-0005](Q-0005-operand-order-without-observed-variation.md) | Can operand order be `unspecified` when no compiler varies it? | provisional | Yes, on source evidence |
| [Q-0006](Q-0006-promoting-report-compilers.md) | When does a report compiler become a baseline compiler? | provisional | After one clean review of its results |
| [Q-0007](Q-0007-language-version.md) | Which language version does Tidal track, and how are newer features handled? | provisional | C++23, with newer features rejected under strict mode, for the proof of concept |
| [Q-0008](Q-0008-similarity-thresholds.md) | Are the similarity thresholds (8-word run, 0.25 overlap) right? | provisional | Keep the thresholds (option 1) until reports from more sections show how many flags each setting produces |
| [Q-0009](Q-0009-license.md) | Which license covers the specification, tests, and tools? | settled | Apache-2.0 with LLVM exception (option 1), added by the repository owner in `LICENSE.txt` |
| [Q-0010](Q-0010-decltype-auto-braced-single.md) | Is `decltype(auto) x{e};` valid, deducing from `e`? | settled | Yes, as all implementations do |
| [Q-0011](Q-0011-missing-definition-class.md) | Is using an entity with no definition `no-diagnostic` or `reject`? | provisional | No-diagnostic |
| [Q-0012](Q-0012-largest-array.md) | Should Tidal set a minimum for the largest array or class an implementation accepts? | settled | No; each limit is implementation-defined |
| [Q-0013](Q-0013-edg-uninitialized-array-element.md) | Does EDG accepting a read of an uninitialized array element during constant evaluation change INIT-1? | compiler defect | No; likely EDG defect |
| [Q-0014](Q-0014-early-dynamic-initialization.md) | May an implementation perform dynamic initialization before the program starts? | provisional | Yes, unspecified |
| [Q-0015](Q-0015-clang18-braced-structured-binding.md) | Is `auto [a, b]{arr};` valid when `arr` is an array? | settled | Yes; Clang 18's rejection is fixed in later versions |
| [Q-0016](Q-0016-constrained-structured-binding.md) | May a structured binding use a constrained `auto`? | compiler defect | No; likely EDG defect |
| [Q-0017](Q-0017-enum-value-out-of-range.md) | Is converting a value outside an enumeration's range undefined? | provisional | Undefined |
| [Q-0018](Q-0018-enum-no-standard-underlying-type.md) | What happens when no standard integer type holds all of an unscoped enumeration's values? | provisional | Impl-defined |
| [Q-0019](Q-0019-repeated-using-declaration-in-block.md) | May a using declaration be repeated in a block? | provisional | Yes, as the newest Clang and GCC allow |
| [Q-0020](Q-0020-unimplemented-features.md) | How should tests record a compiler that doesn't implement a feature yet? | provisional | Per-compiler expectation naming this question |
| [Q-0021](Q-0021-misplaced-standard-attribute.md) | Is a standard attribute applied to the wrong kind of entity rejected or ignored? | provisional | Rejected |
| [Q-0022](Q-0022-func-outside-function.md) | Is `__func__` outside a function rejected? | provisional | Yes |
| [Q-0023](Q-0023-gcc14-deleted-main.md) | Is `int main() = delete;` rejected? | settled | Yes; GCC 14's acceptance is fixed in GCC 16 |
| [Q-0024](Q-0024-main-parameter-lists.md) | Which parameter lists may `main` have? | settled | () and (int, char**) everywhere; others implementation-defined, including a first parameter of type `long` |
| [Q-0025](Q-0025-constinit-placement.md) | Which declarations of a variable must carry `constinit`? | provisional | The initializing declaration, if any declaration does |
| [Q-0026](Q-0026-range-for-storage-class.md) | May a range-based for variable have a storage class? | settled | No; GCC 14's acceptance is fixed in GCC 16 |
| [Q-0027](Q-0027-mixed-ref-qualifiers.md) | May a member function with a ref-qualifier be overloaded with one without? | provisional | No; the declarations are rejected |
| [Q-0028](Q-0028-float-narrowing-out-of-range.md) | Is converting a floating value above the target's finite range implementation-defined or undefined? | provisional | Implementation-defined |
| [Q-0029](Q-0029-chained-comparison.md) | Is a chained comparison such as `3 > 2 > 1` valid? | provisional | Valid; a warning is recommended |
| [Q-0030](Q-0030-pointer-relational-zero.md) | May a pointer be compared with `<` or `<=>` to the literal 0? | provisional | No; rejected |
| [Q-0031](Q-0031-pointer-outside-array.md) | Is forming a pointer outside an array undefined, even without a read through it? | provisional | Undefined |
| [Q-0032](Q-0032-braced-compound-assignment.md) | May the right operand of a compound assignment be a braced list? | provisional | No; rejected |
| [Q-0033](Q-0033-string-literal-identity.md) | May constant evaluation compare the addresses of two string literals? | provisional | No; rejected |
| [Q-0034](Q-0034-subscript-prvalue-array.md) | Is a subscript of an array xvalue or an array member of a prvalue an xvalue? | provisional | Yes, an xvalue (PFX-3); EDG gives an lvalue |
| [Q-0035](Q-0035-delete-void-pointer.md) | Is `delete` of a `void*` rejected? | provisional | Yes |
| [Q-0036](Q-0036-new-floating-size.md) | May the size in an array `new` have floating-point type? | provisional | Yes; converted to the type of `sizeof` |
| [Q-0037](Q-0037-capture-default-outside-function.md) | May a lambda outside a function have a capture default? | provisional | No; rejected |
| [Q-0038](Q-0038-parameter-in-unevaluated-default.md) | May a default argument name a parameter in an operand that is not evaluated? | provisional | Valid; the ban covers only evaluated uses of a parameter |
| [Q-0039](Q-0039-indeterminate-unsigned-char.md) | Is copying an uninitialized `unsigned char` undefined? | provisional | Not undefined at run time for `unsigned char` and `std::byte`, and rejected during constant evaluation |
| [Q-0040](Q-0040-aggregate-paren-reference-member.md) | Does parenthesized aggregate initialization extend the life of a temporary bound to a reference member? | provisional | Only braces extend the lifetime; parentheses do not |
| [Q-0041](Q-0041-jump-past-nontrivial-destructor.md) | May a jump skip a class object that has a non-trivial destructor and no initializer? | provisional | Yes; JMP-6 and SW-5 accept the jump; Clang and EDG reject it |
| [Q-0042](Q-0042-bit-field-narrowing.md) | Is converting a bit-field whose values all fit the target a narrowing conversion? | provisional | Not narrowing when every value of the bit-field's width fits the target |
| [Q-0043](Q-0043-paren-brace-elision.md) | Does parenthesized aggregate initialization allow brace elision? | provisional | No; parentheses allow no brace elision |
| [Q-0044](Q-0044-for-condition-destruction.md) | Is a `for` condition variable destroyed before or after the iteration expression? | compiler defect | After; GCC 14 is fixed in GCC 16 |
| [Q-0045](Q-0045-anonymous-union-unnamed-namespace.md) | Must an anonymous union in an unnamed namespace be declared `static`? | provisional | Yes, `static` at every namespace scope (STO-11, UNI-9); Clang differs |
| [Q-0046](Q-0046-likely-and-unlikely.md) | Is `[[likely]] [[unlikely]]` on one statement rejected or ignored? | provisional | Rejected |
| [Q-0047](Q-0047-using-enum-opaque.md) | May `using enum` name an enumeration that is declared but not defined? | provisional | No; the enumeration must be complete, so the `using enum` is rejected |
| [Q-0048](Q-0048-tuple-size-without-value.md) | If `std::tuple_size<E>` is complete but has no `value`, do structured bindings use the data members? | provisional | Yes; the bindings fall back to the data members (SB-3) |
| [Q-0049](Q-0049-base-pointer-derived-array.md) | Is arithmetic on a base class pointer into an array of a derived class undefined? | provisional | The subobject acts as an array of one element, so moving the pointer outside `p` and `p + 1`, a read through `p + 1`, and a difference of pointers into different elements are undefined |
| [Q-0050](Q-0050-three-way-enum-float.md) | May `<=>` compare an unscoped enumeration with a floating-point value? | provisional | No; rejected |
| [Q-0051](Q-0051-one-past-end-adjacent.md) | Is comparing a one-past-the-end pointer with a pointer to the next object rejected during constant evaluation? | provisional | Rejected during constant evaluation, and unspecified at run time |
| [Q-0052](Q-0052-using-declaration-reduces-access.md) | Can a using-declaration make a public base member private? | provisional | Yes; the using-declaration sets the access, so the use is rejected |
| [Q-0053](Q-0053-delegation-cycle.md) | Must an implementation diagnose an indirect delegation cycle? | provisional | No; the program is not valid, and an indirect cycle is `no-diagnostic` |
| [Q-0054](Q-0054-double-destruction-constant.md) | Is a second destruction rejected during constant evaluation? | provisional | Yes; the behavior is undefined, so constant evaluation rejects it |
| [Q-0055](Q-0055-defaulted-parameter-mismatch.md) | Is a defaulted copy or move constructor with a different parameter type deleted or invalid? | provisional | Deleted; only a use is rejected |
| [Q-0056](Q-0056-delete-without-virtual-destructor.md) | Is delete through a base pointer without a virtual destructor rejected during constant evaluation? | provisional | Yes; the behavior is undefined, so constant evaluation rejects it |
| [Q-0057](Q-0057-anonymous-union-unnamed-namespace.md) | Must an anonymous union in an unnamed namespace be static? | settled | Duplicate of Q-0045 |
| [Q-0058](Q-0058-unnamed-bitfield-cv.md) | Can an unnamed bit-field have a cv-qualifier? | provisional | No; a program must not put a cv-qualifier on an unnamed bit-field |
| [Q-0059](Q-0059-deduced-conversion-function.md) | Can a conversion function have a deduced return type? | provisional | Yes; a conversion function can have a deduced return type |
| [Q-0060](Q-0060-literal-operator-default-argument.md) | Can a literal operator have a default argument? | provisional | No; a literal operator must not have a default argument |
| [Q-0061](Q-0061-comparison-declared-before-default.md) | Can a friend comparison declared outside the class be defaulted in it? | provisional | No; the first declaration of a defaulted comparison must be in its class |
| [Q-0062](Q-0062-out-of-class-default-deleted.md) | Must an implementation diagnose a comparison defaulted outside its class that would be deleted? | provisional | No; the program is not valid, and the error is `no-diagnostic` |
| [Q-0063](Q-0063-floating-literal-out-of-range.md) | Is a floating literal outside the range of its type valid? | provisional | The value is implementation-defined, and an implementation can reject the literal |
| [Q-0064](Q-0064-decimal-literal-too-large-for-signed.md) | Is a decimal literal that fits only an unsigned type valid? | provisional | No; the program is not valid |
| [Q-0065](Q-0065-decltype-declarative-nested-name.md) | Can decltype name the class in a member definition outside the class? | provisional | No; `decltype` must not occur in the qualified name of a declaration |
| [Q-0066](Q-0066-member-changes-meaning.md) | Must an implementation diagnose a class member that changes the meaning of a name? | provisional | No; the class is not valid, and the error is `no-diagnostic` |
| [Q-0067](Q-0067-typedef-name-non-c-class.md) | Can a typedef name an unnamed class that is not C-like? | provisional | No; a class that gets its name from a typedef must be C-like |
| [Q-0068](Q-0068-inline-after-definition.md) | Can a function be declared inline after its definition? | provisional | No; a function must not be declared `inline` after its definition |
| [Q-0069](Q-0069-alignas-weaker-than-natural.md) | Can alignas make an alignment weaker than the natural one? | provisional | No; the program is not valid |
| [Q-0070](Q-0070-alignas-redeclaration-mismatch.md) | Must all declarations of a variable agree on alignas? | provisional | Yes; if a declaration has `alignas`, the definition must have the same alignment |
| [Q-0071](Q-0071-several-alignas-on-class.md) | Which alignas applies when a class has several? | provisional | The strictest alignment |
| [Q-0072](Q-0072-alignas-invalid-positions-gcc14.md) | Can alignas apply to a function, a type alias, or a bit-field? | settled | No; GCC 14's acceptance is fixed in GCC 16 |
| [Q-0073](Q-0073-alignas-on-enumeration.md) | Can alignas apply to an enumeration? | provisional | No; `alignas` must not apply to an enumeration |
| [Q-0074](Q-0074-language-linkage-in-function-type.md) | Is language linkage part of a function type? | provisional | Implementation-defined |
| [Q-0075](Q-0075-c-linkage-conflicts-global.md) | Can a name with C language linkage match a global variable? | provisional | No; the program is not valid |
| [Q-0076](Q-0076-c-linkage-variable-main.md) | Can main, or a variable named main, have C language linkage? | provisional | No; the program is not valid in both cases |
| [Q-0077](Q-0077-throw-empty-specification.md) | Is throw() a valid exception specification? | provisional | Implementation-defined; where `throw()` is accepted, it means `noexcept` |
| [Q-0078](Q-0078-noexcept-operand-narrowing.md) | Can the operand of noexcept narrow when it is converted to bool? | provisional | No; the operand must convert to `bool` without narrowing |
| [Q-0079](Q-0079-noexcept-operand-cycle.md) | Must an implementation diagnose a noexcept operand that depends on itself? | provisional | No; the program is not valid, and the error is `no-diagnostic` |
| [Q-0080](Q-0080-static-const-volatile-initializer.md) | Can a static const volatile member have an initializer in its class? | provisional | Yes; the initializer in the class is valid |
| [Q-0081](Q-0081-single-element-same-class-list.md) | Does a braced list with one object of the same class use the initializer-list constructor? | provisional | Yes; the initializer-list constructor is used, as for other braced lists |
| [Q-0082](Q-0082-array-new-deleted-destructor.md) | Can new create an array of a class whose destructor is deleted? | provisional | Yes; the array form is valid, as the single form is |
| [Q-0083](Q-0083-nested-default-member-init-early.md) | Can a default member initializer of a nested class be used before the enclosing class is complete? | provisional | No; the program is not valid |
| [Q-0084](Q-0084-reference-member-temporary.md) | Must an implementation reject a constructor that binds a reference member to a temporary? | provisional | No; the program is not valid, and the error is `no-diagnostic` |
| [Q-0085](Q-0085-constexpr-destructor-virtual-base.md) | Can a class with a virtual base have a constexpr destructor? | provisional | No, as for constexpr constructors |
| [Q-0086](Q-0086-call-returning-abstract.md) | Is a call to a function that returns an abstract class valid? | provisional | No; the call creates an object of the abstract class |
| [Q-0087](Q-0087-explicit-object-matches-virtual.md) | Can an explicit object member function have the name and parameters of a base virtual function? | provisional | No; the declaration is not valid |
| [Q-0088](Q-0088-union-user-assignment-active.md) | Does assignment through a user-provided operator activate an inactive union member during constant evaluation? | provisional | No; the access is undefined, so constant evaluation rejects it |
| [Q-0089](Q-0089-decltype-in-destructor-call.md) | Can decltype name the class in an explicit destructor call? | provisional | Yes; `decltype` can name the class in an explicit destructor call |
| [Q-0090](Q-0090-unknown-reference-constant.md) | Can constant evaluation use a reference whose referent is unknown, if it does not read through it? | settled | Yes; Clang 22, GCC, and EDG agree, Clang 18 rejects |
| [Q-0091](Q-0091-delete-array-through-base-constant.md) | Is delete[] of a derived array through a base pointer rejected during constant evaluation? | provisional | Yes; the behavior is undefined, so constant evaluation rejects the deletion |
| [Q-0092](Q-0092-lambda-capture-trailing-return.md) | Do names in a lambda's trailing return type find its captures? | provisional | Yes; captures are visible in the parameter list and trailing return type |
| [Q-0093](Q-0093-class-key-qualified-declaration.md) | Is a declaration of only a class key and a qualified name valid? | provisional | No; the declaration is not valid |
| [Q-0094](Q-0094-class-operator-delete-noexcept.md) | Is a class-specific operator delete without a specifier non-throwing? | provisional | Yes; a deallocation function is non-throwing without the keyword |
| [Q-0095](Q-0095-defaulted-comparison-member-throwing.md) | Is a defaulted == non-throwing if a member == is potentially throwing? | provisional | No; the defaulted comparison is potentially throwing, as for defaulted special members |
| [Q-0096](Q-0096-case-label-ends-block.md) | Can a case or default label be the last item of a block? | provisional | Yes; any label can be the last item of a block |
| [Q-0097](Q-0097-jump-skips-nontrivial-destructor.md) | Can a jump skip a variable whose destructor is not trivial? | settled | Duplicate of Q-0041 |
| [Q-0098](Q-0098-range-for-data-member-begin.md) | Do data members named begin and end select the member form of a range-based for? | provisional | Yes; any member named begin or end counts |
| [Q-0099](Q-0099-array-xvalue-subscript.md) | Is a subscript of an array xvalue an xvalue? | settled | Duplicate of Q-0034 |
| [Q-0100](Q-0100-char-zero-null-pointer.md) | Is a character literal with value zero a null pointer constant? | provisional | No; only an integer literal with value zero is a null pointer constant |
| [Q-0101](Q-0101-nullptr-logical-operand.md) | Can nullptr be an operand of a logical operator? | settled | Yes; GCC 14's rejection is fixed in GCC 16 |
| [Q-0102](Q-0102-conditional-zero-nullptr.md) | Does a conditional with the literal 0 and nullptr have the type of nullptr? | provisional | Yes; the result has the type of `nullptr` |
| [Q-0103](Q-0103-relational-nullptr-type.md) | Can two values of the type of nullptr be compared with a relational operator? | provisional | No, as for two pointers to members |
| [Q-0104](Q-0104-object-and-first-member-equal.md) | Do a class object and its first member compare equal through void pointers? | provisional | Yes; the pointers hold the same address |
| [Q-0105](Q-0105-block-extern-hidden-internal.md) | Does a block-scope extern declaration redeclare a hidden internal-linkage variable? | provisional | Yes; the declaration redeclares the namespace-scope entity and takes its internal linkage |
| [Q-0106](Q-0106-reference-to-qualified-function-type.md) | Can a parameter have a reference to a function type with a ref-qualifier? | provisional | No, as for a pointer to such a type |
| [Q-0107](Q-0107-constinit-structured-binding.md) | Can constinit apply to a structured binding declaration? | provisional | No; only static, thread_local, const, and volatile can occur |
| [Q-0108](Q-0108-carries-dependency-first-declaration.md) | Must carries_dependency be on the first declaration of a function? | provisional | Yes, as for noreturn |
| [Q-0109](Q-0109-extend-through-array-member.md) | Does a reference bound to an element of an array member of a temporary extend the temporary? | provisional | Yes; the binding is valid and extends the temporary |
| [Q-0110](Q-0110-null-dereference-reference-constant.md) | Is a reference bound through a null pointer rejected during constant evaluation? | provisional | Yes; the binding is undefined, so constant evaluation rejects it |
| [Q-0111](Q-0111-using-sees-later-default.md) | Does a using declaration see a default argument that a later namespace declaration adds? | provisional | Yes; the default argument is usable through the using declaration |
| [Q-0112](Q-0112-main-parameter-types.md) | Must the parameters of main have the types int and char**? | settled | Duplicate of Q-0024 |
| [Q-0113](Q-0113-scope-of-overload-resolution-and-templates.md) | Should Tidal specify overload resolution and templates in full? | settled | Yes; phases 3 and 4 drafted them as normative chapters (decisions 0033, 0050) |
| [Q-0114](Q-0114-type-of-a-growing-enumerator.md) | What type does an enumerator get when its value outgrows the type before it? | provisional | Unspecified: any integer type that holds the value |
| [Q-0115](Q-0115-gcc-copy-of-object-with-unset-part.md) | Does GCC accepting a copy of an object with an unset part during constant evaluation change CE-22? | compiler defect | No; likely GCC defect |
| [Q-0116](Q-0116-designer-persona-as-source-reader.md) | Should a designer persona that knows WG21 papers also run, as a source reader? | provisional | No, the clean-room designer only for now |
| [Q-0117](Q-0117-constant-union-with-no-active-member.md) | Is a union with no active member a valid constant? | provisional | Yes, and also a valid template argument (CE-21, TARG-19); GCC rejects it |
| [Q-0118](Q-0118-instantiation-before-deduction.md) | Should instantiation (N2) come before deduction (O) in phase 4? | settled | No, O first; phase 4 drafted O before N2 (decision 0050) |
| [Q-0119](Q-0119-diagram-format.md) | Should diagrams use a renderer such as Mermaid, or plain text? | provisional | Plain text |
| [Q-0120](Q-0120-edg-i686-long-long-alignment.md) | Does EDG's 8-byte alignment of `long long` on i686 change TYP-2? | settled | No; the alignment is implementation-defined |
| [Q-0121](Q-0121-write-to-skipped-variable-in-constant-evaluation.md) | Can constant evaluation write a variable whose declaration a jump skipped? | provisional | Yes; GCC and EDG reject it |
| [Q-0122](Q-0122-empty-init-in-range-for.md) | May a range-based `for` have an empty init statement? | provisional | Yes; GCC rejects it |
| [Q-0123](Q-0123-attribute-position-in-pointer-declarator.md) | Does an attribute in a pointer declarator come before or after the cv-qualifiers? | provisional | After; Clang differs |
| [Q-0124](Q-0124-increment-narrow-signed-out-of-range.md) | Is `++` or `--` on a signed operand narrower than `int` defined when the new value does not fit the operand's type? | provisional | The step is defined and the value converts back; EDG differs |
| [Q-0125](Q-0125-clang18-sized-deallocation.md) | Does Clang 18 omitting the sized deallocation functions change NEW-19? | settled | No; Clang 22 declares them |
| [Q-0126](Q-0126-const-object-union-member-value.md) | When does a union or an anonymous union give its members a value for a const object without an initializer? | provisional | A union gives a value when any variant member carries a default member initializer, and a class of only anonymous unions is valid |
| [Q-0127](Q-0127-include-guard-or-pragma-once.md) | Which mechanism does Tidal specify for including a header only once? | settled | Both; the include guard is required and `#pragma once` is implementation-defined |
| [Q-0128](Q-0128-internal-linkage-missing-definition.md) | Must an implementation diagnose a used function with internal linkage that has no definition? | settled | No; the rule needs no diagnostic and a warning is recommended; GCC is silent when the only use is in an uncalled inline function |
| [Q-0129](Q-0129-copy-of-class-constant.md) | Is a copy of a whole class-type constant a use of the constant? | provisional | Yes; the copy is a use, and the lambda must capture the constant |
| [Q-0130](Q-0130-edg-enclosing-constant-use.md) | Does EDG's acceptance of some uses of an enclosing function's constant change ODR-2? | settled | No; EDG differs |
| [Q-0131](Q-0131-hidden-friend-qualified-redeclaration.md) | Can a declaration with a qualified name outside the namespace redeclare a friend function that only a class has declared? | provisional | No; GCC and EDG reject, Clang accepts |
| [Q-0132](Q-0132-explicit-constructor-in-list-sequence.md) | Does a braced list have a conversion sequence to a class parameter when the chosen constructor is `explicit`? | provisional | Yes; GCC agrees, Clang and EDG form no sequence |
| [Q-0133](Q-0133-initializer-list-sequence-before-kind.md) | Does a list sequence to `std::initializer_list` beat a standard sequence when its own sequence is user-defined? | provisional | No; the kind comparison comes first, and ICS-22 decides only between sequences of the same kind |
| [Q-0134](Q-0134-bit-field-promotion-rank.md) | Does a bit-field narrower than `int` but declared with a wider type have a promotion to `int` in a conversion sequence? | provisional | Yes; Clang agrees, GCC and EDG differ |
| [Q-0135](Q-0135-scalar-against-array-reference.md) | Does a braced list prefer a scalar parameter to a reference to an array when both sequences are the identity? | provisional | No, they tie; GCC differs |
| [Q-0136](Q-0136-list-to-array-reference-edge-cases.md) | Do empty braces or a designated list have a sequence to a reference to an array? | provisional | No; Clang agrees, GCC and EDG differ |
| [Q-0137](Q-0137-array-argument-pointer-tie-breakers.md) | Do the pointer tie-breakers apply after an array-to-pointer step? | provisional | Yes; Clang and EDG agree, GCC differs |
| [Q-0138](Q-0138-function-reference-kind-tie-breaker.md) | Does the kind of a function reference break a tie between two conversion functions? | settled | Yes; GCC and EDG agree, Clang differs |
| [Q-0139](Q-0139-surrogates-with-one-parameter-list.md) | Is a call ambiguous when two surrogates differ only in the qualifiers of their conversion functions? | settled | Yes; Clang and EDG agree, GCC differs |
| [Q-0140](Q-0140-default-argument-from-two-scopes.md) | Is a call valid when it uses a default argument that two scopes give? | provisional | No, if the defaults differ; GCC agrees, Clang and EDG differ |
| [Q-0141](Q-0141-builtin-candidate-operand-conversion.md) | After a built-in candidate wins, does a class operand convert past its conversion function? | provisional | No; Clang and GCC agree, EDG differs |
| [Q-0142](Q-0142-constraints-in-address-of-overload-set.md) | Do constraints reduce an overload set whose address is taken, with or without a target type? | provisional | Yes, when the functions meet the conditions of ORD-13; Clang 22 and GCC 16 agree, EDG, Clang 18 and GCC 14 differ |
| [Q-0143](Q-0143-call-through-address-with-static-member.md) | Can a call through the address of an overload set choose a static member? | provisional | Yes; Clang 22 agrees, Clang 18 and GCC differ, EDG has an internal error |
| [Q-0144](Q-0144-const-object-surrogate.md) | Does a const object get a surrogate from a conversion function that is not const? | settled | No; Clang and EDG agree, GCC differs |
| [Q-0145](Q-0145-copy-list-init-explicit-conversion-function.md) | Can copy-list-initialization use an explicit conversion function for the copy constructor's parameter? | provisional | No; GCC and EDG agree, Clang differs |
| [Q-0146](Q-0146-constraints-between-templates-with-different-parameters.md) | Which conditions must two functions meet before their constraints decide between them? | settled | The conditions of ORD-13: same parameter lists, heads, and conversion results; Clang 22 and GCC agree, EDG differs |
| [Q-0147](Q-0147-rewritten-candidate-removes-builtin.md) | Does a rewritten candidate remove the built-in candidate with the same parameter list? | provisional | Yes; Clang and EDG agree, GCC differs |
| [Q-0148](Q-0148-reference-kind-against-template.md) | Does the function reference kind come before the template tie-breakers? | provisional | Yes, the kind comes first; GCC agrees, EDG differs, Clang does not apply the kind |
| [Q-0149](Q-0149-list-for-reference-to-unknown-bound-array.md) | What type does a braced list give a reference to an array of unknown bound? | provisional | The reference keeps its declared type, and empty braces are rejected; Clang differs on the type, GCC on empty braces |
| [Q-0150](Q-0150-explicit-constructor-for-reference-temporary.md) | Can a braced list for a reference use an explicit constructor to make the temporary? | provisional | No; GCC and EDG agree, Clang differs |
| [Q-0151](Q-0151-parenthesized-list-for-reference.md) | Can a parenthesized list of several values initialize a reference to an aggregate? | provisional | No; Clang and EDG agree, GCC differs |
| [Q-0152](Q-0152-same-rank-tie-breakers-before-rvalue-reference.md) | Do the same-rank tie-breakers come before the rvalue-reference preference, and does that preference come before the qualification step? | provisional | Yes to both; Clang agrees, GCC and EDG differ in part |
| [Q-0153](Q-0153-qualification-step-edges.md) | Does the qualification step compare array bounds, and does it compare a reference that binds a temporary? | provisional | Yes for bounds, yes for temporaries; ICS-31 prefers the known bound (option 2), and compares a reference that binds a temporary |
| [Q-0154](Q-0154-list-element-nearest-base.md) | Does the nearest-base tie-breaker apply to a braced list with one element of a derived class? | provisional | Yes; Clang and GCC agree, EDG differs |
| [Q-0155](Q-0155-braced-string-literal-for-array-reference.md) | When does one braced string literal have a sequence to a reference to a character array? | provisional | When the literal can initialize the array; GCC differs on UTF-8, Clang on size and bound |
| [Q-0156](Q-0156-explicit-conversion-value-for-reference.md) | Can direct initialization of an rvalue reference use an explicit conversion function that returns a value? | provisional | Yes, for an rvalue reference; the reference binds directly to the value |
| [Q-0157](Q-0157-list-class-element-for-reference.md) | Does a braced list with one class element bind a reference through the element's conversion function? | provisional | No; a temporary unless the element has the referenced type or a derived class; Clang and EDG agree, GCC differs |
| [Q-0158](Q-0158-equality-returning-reference-to-bool.md) | Can an `==` used for a reversed or rewritten comparison return a reference to `bool`? | provisional | No; Clang and EDG agree, GCC differs |
| [Q-0159](Q-0159-current-object-of-another-class.md) | Is `*this` the implied object argument when its class is not the class of the members found? | provisional | No, the call has no object; GCC agrees, Clang and EDG differ |
| [Q-0160](Q-0160-constraints-without-target-type.md) | Do constraints reduce an overload set whose address has no target type? | settled | Duplicate of Q-0142 |
| [Q-0161](Q-0161-address-of-member-set-other-forms.md) | Can a call through `&(C::f)` or `&f` use a set that holds a non-static member? | provisional | No for `&(C::f)`, yes for `&f`; the parenthesized form is not valid (option 1), and `&f` resolves the set as `&C::f` does |
| [Q-0162](Q-0162-constraint-tie-breaker-parameter-lists.md) | Does the constraint tie-breaker apply when the parameter lists differ beyond the parameters that the call uses? | settled | Duplicate of Q-0146 |
| [Q-0163](Q-0163-conversion-result-function-pointer-conversion.md) | Does the result tie-breaker prefer the identity to a function pointer conversion? | settled | Yes; Clang and GCC agree, EDG differs |
| [Q-0164](Q-0164-braced-array-for-unknown-bound-reference.md) | Does one braced array lvalue bind a reference to an array of unknown bound directly? | provisional | Yes; Clang and EDG agree, GCC makes a temporary |
| [Q-0165](Q-0165-braced-function-for-function-reference.md) | Does one braced function name bind a reference to a function as `=` does? | provisional | Yes; GCC rejects an overload set, EDG a noexcept function |
| [Q-0166](Q-0166-braced-string-literal-reference-temporary.md) | Can one braced string literal initialize the temporary array of a reference? | provisional | Yes, under LIST-6; Clang and EDG agree, GCC rejects |
| [Q-0167](Q-0167-braced-string-literal-char-array-overloads.md) | Does one braced string literal prefer a reference to a `char` array over one to an `unsigned char` array? | provisional | No, ambiguous; Clang and EDG agree, GCC picks `char` |
| [Q-0168](Q-0168-alias-template-same-as-class-template.md) | Is an alias template that passes its parameters through the same template argument as its class template? | provisional | Unspecified; Clang says no, GCC and EDG say yes |
| [Q-0169](Q-0169-template-template-argument-extra-defaults.md) | Can a template template argument have more parameters than the parameter, if the extra ones have defaults? | settled | Yes; Clang 22, GCC, and EDG agree, Clang 18 rejects |
| [Q-0170](Q-0170-type-or-expression-for-constant-parameter.md) | Is `int()` a type or a value when the parameter is a constant parameter? | settled | A type; Clang and GCC agree, EDG reads the value 0 |
| [Q-0171](Q-0171-explicit-specialization-in-class.md) | Can an explicit specialization of a member template be declared in the class? | provisional | Yes; Clang and EDG agree, GCC rejects |
| [Q-0172](Q-0172-function-partial-specialization-diagnosed-on-use.md) | Must an implementation reject a partial specialization of a function template that no call uses? | settled | Yes; Clang and GCC agree, EDG rejects only at a use |
| [Q-0173](Q-0173-partial-specialization-after-use.md) | Must an implementation diagnose a partial specialization declared after a use that it would match? | provisional | No, no diagnostic required; GCC and EDG reject, Clang accepts |
| [Q-0174](Q-0174-partial-specialization-not-more-specialized.md) | Is a partial specialization valid when it is not more specialized than the primary template? | provisional | No; Clang rejects, GCC and EDG accept |
| [Q-0175](Q-0175-static-class-member-read-before-initialization.md) | Can a dynamic initializer read the zero in a member of a later class-type variable? | provisional | No; the read is undefined (OBJ-2) |
| [Q-0176](Q-0176-volatile-initialization-observable.md) | Is the initialization of a volatile object an observable write? | provisional | Yes, for an initializer that runs at run time |
| [Q-0177](Q-0177-attribute-before-base-and-after-new-bound.md) | Can an attribute specifier come before a base class and after the array bound of a `new` expression? | settled | Yes; GCC 14 defect |
| [Q-0178](Q-0178-call-through-pointer-differing-in-noexcept.md) | Is a call through a function pointer whose type differs from the function only in `noexcept` undefined? | provisional | Yes, undefined |
| [Q-0179](Q-0179-throw-in-noexcept-warning-scope.md) | Which throws in a non-throwing function should get a warning? | provisional | Those that no enclosing handler catches by static type; a rethrow needs `catch (...)` |
| [Q-0180](Q-0180-dangling-reference-warnings.md) | Which dangling references should an implementation warn about? | provisional | A general recommended rule, tested on the `new` case |
| [Q-0181](Q-0181-clang18-explicit-object-from-prvalue.md) | Does a class prvalue initialize a by-value explicit object parameter without a copy or move? | provisional | Yes; Clang 18's rejection is fixed in Clang 22 |
| [Q-0182](Q-0182-evidence-reasons-for-implementation-choices.md) | May Evidence give the reason for an implementation's choice? | provisional | Only when a clean input states the reason |
| [Q-0183](Q-0183-cost-model-for-proposals.md) | Should Tidal give proposals a way to state implementation cost? | provisional | No cost model for now; a proposal states cost in words |
| [Q-0184](Q-0184-vendor-options-in-evidence.md) | Should Evidence record vendor options that change behavior? | provisional | Yes, when a test runs the compiler with the option |
| [Q-0185](Q-0185-change-notation-and-versions.md) | How does Tidal mark a change to a rule and the language version a rule holds for? | provisional | A change table for proposals now (option 1), because proposals need it and Tidal specifies one version |
| [Q-0186](Q-0186-proposals-and-outside-judgments.md) | May a proposal rest on judgments from outside Tidal? | provisional | Yes, marked as outside knowledge |
| [Q-0187](Q-0187-no-object-member-object-parameter.md) | In a call with no object, does a member's object parameter take part in overload resolution? | provisional | No; it takes no part; Clang and EDG agree, GCC differs |
| [Q-0188](Q-0188-known-bound-direct-binding.md) | When two references bind directly to one array, is the reference to the known bound better? | provisional | Yes; GCC and EDG agree, Clang finds a tie |
| [Q-0189](Q-0189-constructor-against-conversion-result.md) | Does the result tie-breaker compare a constructor with a conversion function? | provisional | Yes; the constructor's result has the identity; GCC and EDG agree, Clang differs |
| [Q-0190](Q-0190-matching-not-equal-ref-qualifier.md) | Do ref-qualifiers take part in the match of `!=` and `==`? | provisional | No; Clang and EDG agree, GCC differs |
| [Q-0191](Q-0191-inner-array-bound-drop.md) | Does an array bound dropped at an inner pointer level need `const` above it? | provisional | No; GCC and EDG agree, Clang differs |
| [Q-0192](Q-0192-constinit-trivial-default-constructor.md) | Does a class with a trivial default constructor and no initializer have a constant initializer? | provisional | No; the default constructor runs, the zero does not count, and a class with a scalar member has no constant initializer |
| [Q-0193](Q-0193-extended-temporaries-in-constants.md) | Can a constant refer to, and read, every temporary that a namespace reference extends? | provisional | Yes for any REF-4 binding, and a read needs a `const` type; EDG and GCC differ |
| [Q-0194](Q-0194-fold-operand-with-binary-operator.md) | Can an operand of a fold expression contain a binary operator without parentheses? | provisional | No, it needs parentheses; EDG differs |
| [Q-0195](Q-0195-parameter-after-pack-not-deducible.md) | Must an implementation diagnose a function template parameter after a pack that deduction cannot find? | provisional | Yes, an implementation must reject the declaration; EDG rejects, Clang and GCC accept |
| [Q-0196](Q-0196-default-on-friend-class-template.md) | Can a friend declaration of a class template give a default template argument? | provisional | No; Clang and EDG reject, GCC accepts |
| [Q-0197](Q-0197-friend-default-other-declaration.md) | Must an implementation diagnose another declaration of a friend function template that gives a default? | provisional | Yes, an implementation must reject the program; EDG rejects, Clang and GCC accept |
| [Q-0198](Q-0198-friend-named-as-template-parameter.md) | Can a friend function in a class template have the name of a template parameter? | provisional | Yes for the friend declaration (option 1), where Clang and GCC agree |
| [Q-0199](Q-0199-member-enumeration-defined-outside.md) | Can a member enumeration of a class template be defined after the class body? | provisional | Yes; Clang and EDG accept, GCC rejects under -pedantic-errors |
| [Q-0200](Q-0200-alias-template-lambda-same-arguments.md) | Do two uses of one alias template specialization with a lambda give one closure type? | provisional | Yes; GCC and EDG agree, Clang gives two types |
| [Q-0201](Q-0201-template-template-constant-types-differ.md) | Does a template template argument match when its constant parameter has another fixed type? | provisional | Yes; Clang 22 and GCC accept, Clang 18 and EDG reject |
| [Q-0202](Q-0202-union-argument-no-active-member.md) | Can a union with no active member be a template argument? | settled | Duplicate of Q-0117 |
| [Q-0203](Q-0203-thread-local-explicit-specialization-only.md) | Can an explicit specialization of a variable template be thread_local when the template is not? | provisional | Yes; Clang and GCC accept, EDG rejects |
| [Q-0204](Q-0204-access-in-explicit-specialization-function-type.md) | Is a private type in the function type of an explicit specialization access-checked? | provisional | No; Clang and EDG accept, GCC rejects |
| [Q-0205](Q-0205-explicit-specialization-member-enumeration.md) | Can a member enumeration of a class template have an explicit specialization? | provisional | Yes if the class template only declares it; Clang and EDG accept, GCC rejects under -pedantic-errors |
| [Q-0206](Q-0206-clang18-member-template-explicit-primary.md) | Are partial specializations of a member template ignored when its primary is explicitly specialized for one class? | settled | Yes; Clang 18's choice is fixed in Clang 22 |
| [Q-0207](Q-0207-explicit-specialization-of-member-partial-specialization.md) | Does an explicit specialization of one member partial specialization keep the others? | provisional | Yes, and it is selected; Clang and EDG agree, GCC ignores it |
| [Q-0208](Q-0208-noexcept-deduces-bool.md) | Does the noexcept of a function type give a value to a bool constant parameter? | settled | Yes; Clang and GCC agree, EDG differs |
| [Q-0209](Q-0209-placeholder-bound-type.md) | What type does a placeholder constant parameter get from an array bound? | provisional | The type of `sizeof`; Clang and EDG agree, GCC gives `int` |
| [Q-0210](Q-0210-constraints-different-heads-or-results.md) | Do constraints decide between templates whose heads, or whose conversion results, differ? | settled | Duplicate of Q-0146 |
| [Q-0211](Q-0211-swapped-candidate-ordering.md) | Are the parameters of a swapped candidate reversed for partial ordering? | settled | Yes; Clang and EDG agree, GCC differs |
| [Q-0212](Q-0212-edg-dynamic-cast-private-base-constant.md) | Does EDG letting a `dynamic_cast` from a private base succeed during constant evaluation change VIRT-10? | compiler defect | No; Clang and GCC give a null pointer, likely EDG defect |
| [Q-0213](Q-0213-edg-union-chain-assignment-nontrivial.md) | Does EDG accepting an assignment into a union member that has a default constructor that is not trivial change UNI-13? | compiler defect | No; Clang and GCC reject, likely EDG defect |
| [Q-0214](Q-0214-member-object-parameter-rvalue.md) | Is the object parameter of a member with no ref-qualifier an rvalue reference in partial ordering against an rvalue reference? | provisional | Yes; Clang 22 and GCC agree, Clang 18 and EDG differ |
| [Q-0215](Q-0215-default-arguments-before-parameter-check.md) | Are default template arguments substituted before the check of parameters with no template parameter? | settled | Yes; Clang and GCC agree, EDG checks first |
| [Q-0216](Q-0216-default-parameter-against-pack.md) | Does a parameter with a default argument count against a trailing pack in partial ordering? | provisional | Yes; Clang differs from GCC and EDG |
| [Q-0217](Q-0217-qualifiers-on-references-of-two-kinds.md) | Does the qualifier tie-breaker of partial ordering apply to an lvalue reference against an rvalue reference? | provisional | Yes; Clang differs from GCC and EDG |
| [Q-0218](Q-0218-object-parameter-before-reversal.md) | Is the object parameter of a swapped member candidate added before its parameters are reversed? | provisional | Yes; Clang differs from GCC and EDG |
| [Q-0219](Q-0219-placement-deallocation-templates-ordered.md) | Does partial ordering choose among placement deallocation function templates? | settled | Yes; GCC and EDG agree, Clang calls none |
| [Q-0220](Q-0220-placement-deallocation-function-before-template.md) | Does a placement deallocation function win over a template that also matches? | provisional | No; a function and a matching template are two matches, and no deallocation function is called |
| [Q-0221](Q-0221-deleted-deallocation-function-in-new.md) | Is a `new` expression valid when its matching deallocation function is deleted? | provisional | No; Clang and EDG reject, GCC accepts |
| [Q-0222](Q-0222-placement-deallocation-templates-unordered.md) | Is a `new` expression valid when two placement deallocation templates match and neither is preferred? | provisional | Yes, and no deallocation function is called; Clang and GCC agree, EDG rejects |
| [Q-0223](Q-0223-independent-errors-in-unused-templates.md) | Does an implementation diagnose an independent error in a template that is never instantiated? | provisional | No diagnostic required; Clang and EDG diagnose, GCC does not |
| [Q-0224](Q-0224-dependent-name-read-as-value-yields-type.md) | Is a dependent qualified name without typename that becomes a type at instantiation an error? | settled | Yes; Clang and GCC agree, EDG differs |
| [Q-0225](Q-0225-template-keyword-after-typename-qualifier.md) | Does a name after typename T:: need the template keyword before an argument list? | provisional | Yes; Clang and EDG agree, GCC differs |
| [Q-0226](Q-0226-template-keyword-without-argument-list.md) | Can the template keyword come before a member that has no argument list after it? | provisional | Only before a template template argument or a class or alias template; Clang and GCC each differ on one case |
| [Q-0227](Q-0227-template-keyword-in-declarator.md) | Can the template keyword follow the qualifier of a name that a declaration declares? | provisional | No; Clang agrees, GCC agrees for a class, and EDG differs |
| [Q-0228](Q-0228-missing-member-of-current-instantiation-unused.md) | Is a missing member of the current instantiation diagnosed in a template that is not used? | provisional | No diagnostic required while unused; GCC 14 accepts, others reject |
| [Q-0229](Q-0229-alias-template-as-current-instantiation.md) | Does an alias template specialization name the current instantiation? | settled | Yes; Clang and GCC agree, EDG differs |
| [Q-0230](Q-0230-qualified-member-ambiguous-at-instantiation.md) | Is a qualified member name of the current instantiation looked up again at the instantiation? | provisional | Yes; Clang and EDG agree, GCC differs |
| [Q-0231](Q-0231-inline-static-member-initializer-with-class.md) | Is the initializer of an inline static data member instantiated with its class? | settled | No; Clang and GCC agree, EDG differs |
| [Q-0232](Q-0232-scoped-enumeration-with-class.md) | Are the enumerators of a scoped member enumeration instantiated with the class? | provisional | No; Clang and EDG agree, GCC differs |
| [Q-0233](Q-0233-explicit-instantiation-storage-class-and-attribute.md) | Can an explicit instantiation have a storage class or an attribute after the keyword template? | provisional | No, except thread_local; Clang agrees, GCC and EDG each differ |
| [Q-0234](Q-0234-extern-template-internal-linkage.md) | Can an explicit instantiation declaration name a specialization with internal linkage? | provisional | No; Clang agrees, EDG agrees for a function, GCC differs |
| [Q-0235](Q-0235-explicit-instantiation-unqualified-outside-namespace.md) | Can an explicit instantiation outside the template's namespace use an unqualified name? | provisional | No; Clang and GCC agree for a class, Clang and EDG differ for a variable |
| [Q-0236](Q-0236-explicit-instantiation-array-bound.md) | Can an explicit instantiation of a variable leave out the array bound? | settled | Yes; GCC and EDG agree, Clang differs |
| [Q-0237](Q-0237-explicit-specialization-after-extern-template.md) | Can an explicit specialization follow an explicit instantiation declaration? | settled | No; Clang and GCC agree, EDG differs |
| [Q-0238](Q-0238-alignas-pack-on-class.md) | Does `alignas(T...)` on a class take the strictest alignment of the elements? | compiler defect | Yes; a class takes the strictest element, as a member does (option 1), and GCC's result on a class is a defect |
| [Q-0239](Q-0239-deduction-through-inherited-constructors.md) | Does a class template that inherits constructors get the deduction guides of its base? | provisional | Yes; GCC and EDG agree, Clang differs |
| [Q-0240](Q-0240-deduction-through-alias-templates.md) | Can a program deduce the arguments of a class template through an alias template? | settled | Yes; Clang 22 and GCC agree, Clang 18 and EDG 7.0 differ |
| [Q-0241](Q-0241-guide-parameter-not-deducible.md) | Must an implementation diagnose a deduction guide whose template parameter cannot be deduced? | provisional | Yes; Clang agrees, GCC and EDG differ |
| [Q-0242](Q-0242-guide-result-with-cv-qualifier.md) | Can the result of a deduction guide have a cv-qualifier? | provisional | No; Clang and EDG agree, GCC differs |
| [Q-0243](Q-0243-deduced-name-after-dependent-qualifier.md) | Can a deduced class template name follow a dependent qualifier? | provisional | Yes; every compiler accepts |
| [Q-0244](Q-0244-aggregate-candidate-in-new-expression.md) | Does a parenthesized list in a `new` expression form the aggregate deduction candidate? | provisional | Yes; Clang and EDG agree, GCC differs |
| [Q-0245](Q-0245-template-keyword-before-destructor.md) | Can the keyword template come before a destructor name? | settled | No; Clang and GCC agree, EDG differs |
| [Q-0246](Q-0246-current-instantiation-member-changes-at-instantiation.md) | Is a program in error when a name found in the current instantiation finds a different single member at the instantiation? | provisional | Yes, no diagnostic required; every compiler accepts |
| [Q-0247](Q-0247-pack-expanded-twice-in-base-list.md) | Is a base list that expands the same pack twice diagnosed before instantiation? | provisional | Yes; Clang and EDG agree, GCC differs |
| [Q-0248](Q-0248-conversion-function-of-dependent-base.md) | Does an unqualified conversion function name find a conversion function of a dependent base? | provisional | No; GCC and EDG agree, Clang differs |
| [Q-0249](Q-0249-later-default-argument-for-qualified-call.md) | Does a qualified call in a template use a default argument that a later declaration adds? | provisional | No diagnostic required; GCC uses it, Clang and EDG reject |
| [Q-0250](Q-0250-constant-function-defined-after-template.md) | Can a template's independent constant use a constexpr function that is defined after the template? | provisional | No diagnostic required; GCC and EDG accept, Clang rejects |
| [Q-0251](Q-0251-member-partial-specializations-become-same.md) | Is a class instantiation not valid when two member partial specializations become the same? | provisional | Yes; Clang only, GCC and EDG accept |
| [Q-0252](Q-0252-constexpr-function-in-unevaluated-operand.md) | When does an unevaluated operand instantiate a constexpr function template? | provisional | In a braced initializer; EDG and GCC differ on single rows |
| [Q-0253](Q-0253-local-class-default-argument-instantiated.md) | Is the default argument of a local class's member function instantiated with the enclosing function? | provisional | Yes; Clang and EDG agree, GCC differs |
| [Q-0254](Q-0254-variable-template-defined-after-explicit-instantiation.md) | Can a variable template's definition follow its explicit instantiation definition? | settled | Yes; Clang and EDG agree, GCC differs |
| [Q-0255](Q-0255-explicit-instantiation-exception-specification.md) | Must the exception specification of an explicit instantiation match the template? | settled | Yes; Clang and EDG agree, GCC differs |
| [Q-0256](Q-0256-explicit-instantiation-of-placeholder-variable.md) | How does an explicit instantiation name a variable template declared with auto? | settled | With the deduced type; Clang and GCC agree, EDG differs |
| [Q-0257](Q-0257-constant-evaluation-after-first-point.md) | Must an implementation reject a constant evaluation that needs a declaration after the first point of instantiation? | settled | No; the points differ, INST-32 applies, and no diagnostic is required |
| [Q-0258](Q-0258-virtual-function-trailing-clause.md) | Can a virtual function have a trailing requires clause? | provisional | No; Clang and EDG agree, GCC differs |
| [Q-0259](Q-0259-requires-expression-parameter-default-and-ellipsis.md) | Can a parameter of a requires expression have a default argument, or the list end in an ellipsis? | provisional | No; Clang and EDG agree, GCC differs |
| [Q-0260](Q-0260-trailing-clause-in-function-template.md) | Can a lambda or a member of a local class inside a function template have a trailing requires clause? | settled | Yes; Clang and GCC agree, EDG differs |
| [Q-0261](Q-0261-constraints-before-return-type.md) | Are the constraints of a function template checked before its return type is formed? | settled | Yes; every current compiler agrees, Clang 18 differs |
| [Q-0262](Q-0262-order-of-placeholder-constraints.md) | Is the requires clause after the template parameter list checked before the constraints of placeholder parameters? | provisional | Yes; GCC and EDG agree, Clang differs |
| [Q-0263](Q-0263-non-bool-atomic-constraint-skipped.md) | Is an atomic constraint whose type is never bool an error when a short circuit skips it? | provisional | Yes; Clang and EDG agree, GCC differs |
| [Q-0264](Q-0264-satisfaction-changes-between-checks.md) | Does an implementation diagnose an atomic constraint whose result changes between two checks? | provisional | No diagnostic required; GCC 16 diagnoses |
| [Q-0265](Q-0265-template-template-argument-more-constrained.md) | Can a template template argument be more constrained than its parameter? | settled | No; Clang 22 and GCC agree, Clang 18 and EDG differ |
| [Q-0266](Q-0266-eligible-constrained-special-members.md) | Do the constraints of special member functions decide which ones make a class trivial? | provisional | Yes; Clang and GCC agree, EDG differs |
| [Q-0267](Q-0267-constrained-friend-template-declaration.md) | Must a friend template whose constraint uses the enclosing class's template parameter be a definition? | settled | Yes; Clang and GCC agree, EDG differs |
| [Q-0268](Q-0268-constrained-friend-definitions-distinct.md) | Are the constrained friend definitions of two class specializations two functions? | settled | Yes; Clang and GCC agree, EDG differs |
| [Q-0269](Q-0269-placeholder-parameter-redeclares-template.md) | Does a function with a placeholder parameter redeclare the function template with the same invented head? | provisional | Yes, following DED-34 and TPL-20; Clang agrees, GCC and EDG differ |
| [Q-0270](Q-0270-attribute-on-concept.md) | Can a concept definition have an attribute after the concept's name? | settled | Yes; every current compiler agrees, Clang 18 differs |
| [Q-0271](Q-0271-expansion-for-alias-template-parameters.md) | Can a pack expansion give an argument to a parameter of an alias template that is not a pack? | settled | No; every compiler rejects |
| [Q-0272](Q-0272-expansion-for-concept-parameters.md) | Can a pack expansion give an argument to a parameter of a concept that is not a pack? | provisional | No; Clang and GCC agree, EDG differs |
| [Q-0273](Q-0273-expansion-before-end-outside-a-call.md) | How does a list with an expansion before its end take part in deduction outside a call? | provisional | Non-deduced; the partial specialization is rejected; Clang agrees, GCC and EDG differ |
| [Q-0274](Q-0274-using-declaration-empty-pack-name.md) | What does a name mean that finds an expanded using-declarator when the pack is empty? | provisional | Not valid, no diagnostic required; GCC and EDG accept, Clang rejects |
| [Q-0275](Q-0275-template-template-parameter-deduction.md) | Can a template template parameter stand without arguments for class template argument deduction? | provisional | Yes; every compiler accepts |
| [Q-0276](Q-0276-trailing-base-pack-length-fixed.md) | When an earlier element fixes the length of a trailing pack of bases, does each later element still deduce? | provisional | Yes; Clang and EDG agree, GCC differs |
| [Q-0277](Q-0277-string-literal-for-array-member.md) | How does a string literal initialize an array member in the aggregate candidate? | provisional | Braces elided for a fixed bound; Clang and GCC agree, EDG differs on a fixed char array |
| [Q-0278](Q-0278-alias-guide-rvalue-reference.md) | Is `T&&` of a class parameter a forwarding reference in a guide made through an alias template? | provisional | No; Clang and GCC agree, EDG differs |
| [Q-0279](Q-0279-alias-single-element-list.md) | Does one element of the class type skip the initializer-list phase through an alias template? | settled | Yes; Clang 22 and GCC agree, EDG differs |
| [Q-0280](Q-0280-guides-with-same-parameters.md) | Can two deduction guides have the same parameters and different results? | provisional | Templates yes, others no; Clang and GCC agree, EDG differs |
| [Q-0281](Q-0281-constrained-constant-parameter-more-constrained.md) | Is a function template with a constrained constant parameter more constrained than the same template with `auto`? | settled | Yes; Clang 22 and GCC 16 agree, Clang 18, GCC 14, and EDG differ |
| [Q-0282](Q-0282-order-of-constant-placeholder-constraint.md) | Is the constraint of a constrained constant parameter checked in the order of the template parameters? | provisional | Yes; Clang and GCC 16 agree, GCC 14 and EDG differ |
| [Q-0283](Q-0283-template-template-parameter-own-constraints.md) | Does an implementation diagnose a use of a template template parameter that fails the parameter's own constraints? | settled | No diagnostic required; EDG diagnoses |
| [Q-0284](Q-0284-parameter-mapping-holds-used-parameters.md) | Does the mapping of an atomic constraint hold only the template parameters that its expression uses? | settled | Yes; Clang 22 and GCC agree, EDG and Clang 18 differ |
| [Q-0285](Q-0285-type-constraints-on-packs-ordered.md) | Are two variadic templates ordered by the type constraints on their packs? | settled | No; Clang and GCC agree, EDG differs |
| [Q-0286](Q-0286-constrained-placeholder-structured-binding.md) | Can a structured binding declaration have a constrained placeholder? | settled | No; Clang and GCC agree, EDG differs |
| [Q-0287](Q-0287-compound-requirement-step-order.md) | In a compound requirement, are the arguments of the type constraint substituted only after the expression and the `noexcept` test hold? | provisional | Yes, and unused extra arguments are substituted; EDG agrees, Clang and GCC each differ in one case |
| [Q-0288](Q-0288-requirement-access-inside-member.md) | Can a requires expression inside a member function use a private member of the class? | settled | Yes; every current compiler agrees, GCC 14 differs |
| [Q-0289](Q-0289-nested-requirement-evaluates-parameter.md) | Is a nested requirement that evaluates a parameter of the requires expression an error? | settled | Yes; every current compiler agrees, Clang 18 differs |
| [Q-0290](Q-0290-requires-expression-void-parameter-list.md) | Does `(void)` give a requires expression no parameters inside a template? | provisional | Yes; GCC and EDG agree, Clang differs |
| [Q-0291](Q-0291-explicit-instantiation-unsatisfied-member.md) | Can a program explicitly instantiate a member function whose constraints fail for the class's arguments? | settled | No; every current compiler agrees, Clang 18 differs |
| [Q-0292](Q-0292-partial-specialization-repeats-primary-not-more-constrained.md) | Can a partial specialization repeat the primary template's parameters when it is not more constrained? | settled | No; Clang and GCC agree, EDG differs |
| [Q-0293](Q-0293-constraints-templates-differing-in-ellipsis.md) | Do constraints decide between two templates that differ only in an ellipsis? | provisional | Yes, the ellipsis does not count for templates (ORD-13); Clang and EDG agree, GCC differs |
| [Q-0294](Q-0294-braced-list-model.md) | How does a braced list bind a reference, initialize an array, and form a conversion sequence? | provisional | The table of REF-P3: direct, temporary, or none; 14 child questions keep each split |
| [Q-0295](Q-0295-partial-specialization-constant-parameter-type.md) | Must a constant argument's type fit the declared type of a partial specialization's constant parameter with no conversion? | provisional | Yes, as in a function template (DED-12); GCC and EDG agree, Clang differs |
| [Q-0296](Q-0296-later-default-before-parameter-without-one.md) | Must an implementation reject a later declaration whose new default comes before a parameter that has no default in any declaration? | provisional | Yes, TPL-14 counts the defaults of all declarations; Clang and EDG agree, GCC differs |
| [Q-0297](Q-0297-const-variable-template-linkage.md) | Does a `const` variable template have one object for each specialization across translation units? | settled | Yes; Clang and GCC 16 agree, GCC 14 differs |
| [Q-0298](Q-0298-pointer-difference-beyond-ptrdiff.md) | How do implementations treat a pointer difference that does not fit its type? | settled | Undefined (ADD-18); EDG rejects positions past the largest difference |
| [Q-0299](Q-0299-alignment-enumeration-shape.md) | Must an implementation reject a declaration of `std::align_val_t` with another shape? | provisional | No; LSUP-5 needs no diagnostic, GCC and EDG reject another shape, and Clang accepts it |
| [Q-0300](Q-0300-aligned-form-for-ordinary-type.md) | Does a `new` expression for a type of ordinary alignment fall back to an aligned allocation function? | provisional | No; there is no reverse step, and the program is rejected (NEW-30, option 1) |
| [Q-0301](Q-0301-erroneous-array-bound.md) | What does a `new` expression do with an erroneous run-time array bound? | provisional | No allocation call, throws `std::bad_array_new_length` (NEW-35); Clang throws `std::bad_alloc` from the allocation function, GCC allocates first for too few elements |
| [Q-0302](Q-0302-erroneous-bound-non-throwing.md) | Does an erroneous array bound give a null pointer when the allocation function is non-throwing? | provisional | Yes (NEW-36); Clang agrees, GCC throws |
| [Q-0303](Q-0303-nothrow-new-constant-evaluation.md) | Can constant evaluation use a `new` expression with the nothrow tag? | provisional | Yes (NEW-37); Clang and GCC 16 accept, GCC 14 and EDG reject |
| [Q-0304](Q-0304-byte-wise-pointer-arithmetic.md) | Can a pointer to a character type step through the bytes of an object that is not an array of bytes? | provisional | Yes; the bytes act as an array of `sizeof` elements (STG-13, option 1) |
| [Q-0305](Q-0305-char-array-hosting.md) | Does an array of plain `char` host an object created in it? | settled | No; only arrays of `unsigned char` and `std::byte` host objects (STG-4, option 1) |
| [Q-0306](Q-0306-signed-char-access.md) | Can a program read or write another object through a `signed char` glvalue? | settled | No; `signed char` is not an allowed type, and the access is undefined (STG-23, option 1) |
| [Q-0307](Q-0307-constant-evaluation-replacement.md) | Must constant evaluation reject a use of an old pointer after a replacement that STG-9 does not allow? | provisional | Yes, undefined (STG-19, STG-20); every constant evaluator accepts the base case, and only Clang 22 rejects the `const` case |
| [Q-0308](Q-0308-class-size-overflow.md) | Must an implementation reject a class whose size does not fit in `std::size_t`? | settled | Yes (STG-1); Clang 18 accepts |
| [Q-0309](Q-0309-union-member-read-after-destructor-constant.md) | Must constant evaluation reject a read of a union member after an explicit destructor call ends it? | compiler defect | Yes, undefined (UNI-14, OBJ-2); Clang and EDG reject, GCC accepts |
| [Q-0310](Q-0310-constexpr-function-first-point.md) | Which point of instantiation does a constexpr function specialization use outside constant evaluation? | provisional | Either point (INST-32); GCC takes the end of the unit, Clang and EDG the first point |
| [Q-0311](Q-0311-adl-template-id-argument.md) | Does an argument that is a template-id for a function template add the namespaces of its template arguments? | settled | Yes; a template-id adds what each type template argument adds (NAM-27, option 1), as GCC does |
| [Q-0312](Q-0312-token-sequences-no-production-accepts.md) | Should the specification list token sequences that no production accepts, such as `for constexpr`? | provisional | The grammar is the statement, and TR-12 gives its effect |
| [Q-0313](Q-0313-scoped-enumeration-integral-operand.md) | Can an enumerator of a scoped enumeration give the operand of `alignas`, the width of a bit-field, or the size in an array `new`? | provisional | No (ALN-8, BF-2, NEW-2); EDG accepts it in `alignas` and a width, GCC in `new` |
| [Q-0314](Q-0314-class-operand-integral-constant.md) | Can a constant of class type that converts to an integer give the operand of `alignas` or the width of a bit-field? | provisional | Yes (ALN-8, BF-1), as Clang and EDG do; GCC rejects |
| [Q-0315](Q-0315-template-argument-use.md) | Does a constant template argument use the variable or function that it names? | settled | Yes (ODR-3); Clang reports an undefined internal function, GCC does not |
| [Q-0316](Q-0316-member-access-class-being-defined.md) | Can a member access name an earlier member of the class whose definition contains the access? | provisional | Yes (PFX-11), as GCC and EDG do; Clang rejects |
| [Q-0317](Q-0317-explicit-object-default-argument.md) | Can an explicit object parameter have a default argument? | provisional | No (CLS-14); EDG accepts |
| [Q-0318](Q-0318-member-call-null-pointer-constant.md) | Must constant evaluation reject a member function call through a null pointer? | provisional | Yes, undefined (PFX-21); EDG accepts |
| [Q-0319](Q-0319-explicit-object-initialization-order.md) | When is an explicit object parameter initialized, relative to the other arguments? | provisional | In an unspecified place (SEQ-1); GCC 14 differs from GCC 16 and Clang |
| [Q-0320](Q-0320-volatile-object-read-through-plain-constant.md) | Must constant evaluation reject a read of a volatile object through a glvalue that is not volatile? | provisional | Yes, undefined (VOL-6); GCC and EDG accept |
| [Q-0321](Q-0321-initializer-list-when-needed.md) | When does a program need the class `std::initializer_list`? | provisional | Where a braced list initializes an object of that type or `auto` deduces it (LSUP-1); GCC checks every definition |
| [Q-0322](Q-0322-implementation-operations-without-headers.md) | How do tests reach the operations that only the implementation can provide? | provisional | Keep them referenced; tests use no standard header (decision 0051) |
| [Q-0323](Q-0323-allocation-array-element-pointer.md) | Does a `T*` from an allocation function's result point to element 0 of an array of `T` whose elements are not built yet? | provisional | Yes; `p + i` is valid before element `i` exists |
| [Q-0324](Q-0324-byte-array-cast-implicit-object.md) | Does a cast of a byte array's address reach an object that the array created implicitly? | settled | No; the cast points to a byte |
| [Q-0325](Q-0325-objects-at-fixed-addresses.md) | Does an object exist at an address that a program gets from an integer, such as a device register? | provisional | The platform decides |
| [Q-0326](Q-0326-volatile-access-width.md) | What size of memory access does a volatile read or write use, and can an implementation merge or split volatile accesses? | provisional | Implementation-defined |
| [Q-0327](Q-0327-byte-pointer-from-subobject.md) | Can a byte pointer made from the address of a member move over the bytes of the whole object? | provisional | Yes; the byte pointer can move over the bytes of the complete object |
| [Q-0328](Q-0328-padding-byte-value.md) | What does a read of a padding byte give, and can a program copy it through `char`? | provisional | A padding byte has no value until the program writes it, and a copy through `char` needs a value in each byte |
| [Q-0329](Q-0329-consteval-local-reference.md) | Does the initializer of a local `const` reference that binds a temporary run during constant evaluation? | provisional | The initializer runs at run time and `r` is 2 |
| [Q-0330](Q-0330-unknown-escape-warning.md) | Must an implementation reject a character literal with an unknown escape sequence? | provisional | The program is invalid and is rejected |
| [Q-0331](Q-0331-narrowing-unscoped-enumeration.md) | Which values does an unscoped enumeration without a fixed type have for narrowing? | provisional | The values that ENUM-22 gives |
| [Q-0332](Q-0332-gcc-shift-sanitizer-double-call.md) | Does GCC 16 calling the left operand of `<<` twice under the shift sanitizer change SHF-8? | compiler defect | No; GCC defect |
| [Q-0333](Q-0333-gcc-ubsan-member-function-pointer-equality.md) | Does GCC rejecting member function pointer equality in constant evaluation under `-fsanitize=undefined` change MPTR-5? | compiler defect | No; GCC defect |
| [Q-0334](Q-0334-ordering-not-equal-zero.md) | Does an ordering class with `!=` against a literal 0, and no `==`, serve a defaulted `<=>`? | provisional | The class must declare `==` or `!=` against `0` |
| [Q-0335](Q-0335-constant-temporary-literal-type.md) | Must a temporary that a constant evaluation makes have a literal type? | provisional | Yes; constant evaluation rejects a temporary of a type that is not literal |
| [Q-0336](Q-0336-enumeration-value-from-bytes.md) | Is a read of an enumeration object undefined when its bytes hold a value outside the enumeration's values? | provisional | Yes; the read is undefined, whatever stored the value |
| [Q-0337](Q-0337-explicit-conversion-value-for-const-reference.md) | Can direct initialization of an lvalue reference to `const` use an explicit conversion function that does not return an lvalue? | provisional | No; only a function that returns an lvalue reference serves it |
| [Q-0338](Q-0338-constant-equality-class-and-subobject.md) | During constant evaluation, does the address of a class object compare equal to the address of its first member or of its base? | settled | True |
| [Q-0339](Q-0339-copy-part-with-two-copy-constructors.md) | Does a part with a trivial and a non-trivial copy constructor make the enclosing copy constructor non-trivial when the copy selects the trivial one? | provisional | The selected function decides |
| [Q-0340](Q-0340-layout-compatible-overlap-mark.md) | Are two structs layout-compatible when matching members both have `[[no_unique_address]]`? | provisional | Layout-compatible |
| [Q-0341](Q-0341-standard-layout-base-type-at-offset-zero.md) | Does a base whose type sits deep inside the first member, or in a `[[no_unique_address]]` member, make a class not standard-layout? | provisional | Not standard-layout in all four cases |
| [Q-0342](Q-0342-tail-padding-defaulted-functions.md) | Does a base that declares defaulted special members lend its tail padding to a derived class? | provisional | The base keeps its padding |
| [Q-0343](Q-0343-trivially-copyable-deleted-functions.md) | Is a class trivially copyable when its destructor is deleted, or when all its copy and move functions are deleted? | provisional | No; neither class is trivially copyable |
| [Q-0344](Q-0344-byte-copy-over-const-member.md) | Can a byte copy write over a live object that has a `const` member? | provisional | The copy is undefined |
| [Q-0345](Q-0345-common-initial-sequence-base-members.md) | Do the members that a base declares count in the common initial sequence of a struct? | provisional | Yes; the members that a base declares count |
| [Q-0346](Q-0346-explicit-instantiation-variable-type-differs.md) | Can an explicit instantiation of a variable state a type that differs from the specialization's? | provisional | No; Clang agrees, GCC and EDG each differ on some cases |
| [Q-0347](Q-0347-likely-on-identifier-label.md) | Can `[[likely]]` apply to a label that is an identifier? | provisional | Valid |
| [Q-0348](Q-0348-pointer-to-member-ref-qualified-alias.md) | Does a pointer to member formed from an alias of a ref-qualified function type keep the ref-qualifier? | provisional | The ref-qualifier is kept |
| [Q-0349](Q-0349-allocator-result-other-type.md) | In constant evaluation, can a member of `std::allocator<T>` convert the result of `::operator new` to a pointer to a type other than `T`? | provisional | No; only the conversion to `T*` is valid |
| [Q-0350](Q-0350-clang-class-size-wrap.md) | Does Clang conform when it accepts a class whose size it cannot represent? | provisional | Clang has a defect in both cases |
| [Q-0351](Q-0351-instantiation-depth-measure.md) | How is the depth of nested instantiations counted, and must it reach a minimum? | settled | Both are implementation-defined, with no minimum |
| [Q-0352](Q-0352-aggregate-pack-parentheses.md) | Does a list in parentheses form the aggregate candidate for a class with a pack of bases? | provisional | Yes; the candidate is formed as for braces |
| [Q-0353](Q-0353-edg-converted-member-pointer-constant.md) | Does EDG losing the member that a converted member pointer names during constant evaluation change MPTR-4? | provisional | No; the converted pointer names the member, and EDG differs |
| [Q-0354](Q-0354-edg-decltype-of-placeholder-parameter.md) | Does EDG checking a call whose argument names `decltype` of a generic lambda's parameter before the call is instantiated change DEP-1? | provisional | No; the program is valid, and EDG differs |
| [Q-0355](Q-0355-edg-virtual-call-from-intermediate-class-constant.md) | Does EDG calling a base's function for a virtual call made inside an intermediate class during constant evaluation change VIRT-1? | provisional | No; the call reaches `W::f`, and EDG differs |
| [Q-0356](Q-0356-gcc14-member-pointer-comma-operand-order.md) | Does GCC 14 evaluating the right operand of `.*` first, when the left operand is a comma expression, change SEQ-2? | compiler defect | No; the left operand comes first, and GCC 14 is wrong |
| [Q-0357](Q-0357-byte-copy-into-shared-padding.md) | What value does a derived member have after a byte copy into the base subobject writes over it? | provisional | The member takes the value of its new bytes, and a padding byte with no value gives it none |
| [Q-0358](Q-0358-new-deallocation-noexcept-constructor.md) | Does a `new` expression use the deallocation function when its constructor is `noexcept`? | provisional | Yes; the `new` expression uses the deallocation function whatever the exception specification of the constructor |
| [Q-0359](Q-0359-volatile-byte-copy.md) | Is a read of a volatile object's bytes through `unsigned char` that is not volatile an access under VOL-6? | provisional | Yes; the plain byte copy is undefined, and a copy uses volatile byte glvalues |
| [Q-0360](Q-0360-alignment-above-largest-extended.md) | Does an implementation reject an `alignas` above its largest extended alignment? | provisional | The implementation rejects the alignment |
| [Q-0361](Q-0361-tail-padding-deleted-and-move-members.md) | Does a base with a deleted constructor or a user-provided move assignment operator lend its tail padding? | provisional | The base keeps its padding |
| [Q-0362](Q-0362-anonymous-union-member-shares-class-name.md) | Can a member of an anonymous union share its name with a class? | provisional | The member can share the name |
| [Q-0363](Q-0363-handler-reference-to-array-or-function.md) | Does a handler of a reference to an array or a function match a thrown array or function? | provisional | The handler never matches |
| [Q-0364](Q-0364-operator-lookup-block-declaration.md) | Does a block declaration of an operator function turn off argument-dependent lookup for an operator? | provisional | Argument-dependent lookup always adds its functions |
| [Q-0365](Q-0365-return-value-destroyed-on-throw.md) | Is the result of a return statement destroyed when a local destructor then throws? | provisional | The result is destroyed |
| [Q-0366](Q-0366-main-cpp-linkage-specification.md) | Can `main` be declared inside an `extern "C++"` block? | provisional | Accept the form |
| [Q-0367](Q-0367-odr-use-exemption-forms.md) | Which further forms does ODR-3 exempt from a use, and does a discarded statement exempt a use from the capture rules? | provisional | Follow the review in each form |
| [Q-0368](Q-0368-scalar-lifetime-own-initializer.md) | Is a write to a scalar during its own initializer outside the scalar's lifetime? | provisional | The write is outside the lifetime and undefined |

## Status values

- **open**: no provisional answer yet. The specification marks affected rules as open.
- **provisional**: the specification follows the provisional answer. Opinions welcome.
- **compiler defect**: the provisional answer is that one compiler, or an
  older version of one, is wrong, and the specification follows the others. The
  affected tests carry an override for that compiler with a note naming
  the question. The question could be reported to that compiler's
  project.
- **settled**: a pull request changed the status, giving the reason. Settled
  questions can be reopened by another pull request.

## How to contribute

**Add an opinion.** Edit the question file and add a subsection at the end of
"Opinions":

```markdown
### @your-handle, YYYY-MM-DD

**Position:** option 2

Your reasoning. Point at evidence: test files, compiler output, or
implementation source.
```

**Propose a change of answer.** In the same pull request, update the
"Provisional answer" section and the affected specification rules and
tests, and run `tools/question_index.py` to rebuild the index above. The test suite must still pass on baseline compilers.

**Raise a new question.** Copy `TEMPLATE.md` to `Q-NEW-<slug>.md` and cite
the question as `Q-NEW-<slug>`. `tools/number_questions.py` gives it the
next free number and adds it to the index.

## Ground rules

- Argue from evidence anyone can check: test programs, compiler behavior,
  and implementation source.
- Don't quote, paste, or paraphrase the ISO standard, its drafts, WG21
  papers, or cppreference. Pull requests that do will be closed. The
  project's input rules are in `documentation/clean-room.md`.
- One question per file. Keep opinions in the file they answer.
