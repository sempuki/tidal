# Open questions

Every question the project raises gets a file here: classification calls,
compiler divergences, process choices. Anyone can weigh in by opening a pull
request.

The spec never waits on a question. Each question has a **provisional
answer**, and the spec follows it until a pull request changes it.

## Index

| ID | Question | Status | Provisional answer |
|---|---|---|---|
| [Q-0001](Q-0001-floating-overflow-constant-evaluation.md) | Is floating overflow during constant evaluation undefined or implementation-defined? | provisional | undefined |
| [Q-0002](Q-0002-edg-floating-division-by-zero.md) | Does EDG accepting floating division by zero during constant evaluation change MUL-11? | compiler defect | no; likely EDG defect |
| [Q-0003](Q-0003-edg-int-min-divided-by-minus-one.md) | Does EDG accepting `INT_MIN / -1` during constant evaluation change MUL-10? | compiler defect | no; likely EDG defect |
| [Q-0004](Q-0004-floating-division-by-zero-at-run-time.md) | Should floating division by zero be implementation-defined on IEEE 754 targets? | provisional | no, keep undefined |
| [Q-0005](Q-0005-operand-order-without-observed-variation.md) | Can operand order be `unspecified` when no compiler varies it? | provisional | yes, on source evidence |
| [Q-0006](Q-0006-promoting-report-compilers.md) | When does a report compiler become a baseline compiler? | provisional | after one clean review of its results |
| [Q-0007](Q-0007-language-version.md) | Which language version does Tidal track, and how are newer features handled? | provisional | C++23; newer features rejected under strict mode |
| [Q-0008](Q-0008-similarity-thresholds.md) | Are the similarity thresholds (8-word run, 0.25 overlap) right? | provisional | keep for now |
| [Q-0009](Q-0009-license.md) | Which license covers the specification, tests, and tools? | settled | Apache-2.0 with LLVM exception |
| [Q-0010](Q-0010-decltype-auto-braced-single.md) | Is `decltype(auto) x{e};` valid, deducing from `e`? | provisional | yes, as all implementations do |
| [Q-0011](Q-0011-missing-definition-class.md) | Is using an entity with no definition `no-diagnostic` or `reject`? | provisional | no-diagnostic |
| [Q-0012](Q-0012-largest-array.md) | Should Tidal set a minimum for the largest array an implementation accepts? | provisional | no; the limit is implementation-defined |
| [Q-0013](Q-0013-edg-uninitialized-array-element.md) | Does EDG accepting a read of an uninitialized array element during constant evaluation change INIT-1? | compiler defect | no; likely EDG defect |
| [Q-0014](Q-0014-early-dynamic-initialization.md) | May an implementation perform dynamic initialization before the program starts? | provisional | yes, unspecified |
| [Q-0015](Q-0015-clang18-braced-structured-binding.md) | Is `auto [a, b]{arr};` valid when `arr` is an array? | compiler defect | yes; Clang 18's rejection is fixed in later versions |
| [Q-0016](Q-0016-constrained-structured-binding.md) | May a structured binding use a constrained `auto`? | compiler defect | no; likely EDG defect |
| [Q-0017](Q-0017-enum-value-out-of-range.md) | Is converting a value outside an enumeration's range undefined? | provisional | undefined |
| [Q-0018](Q-0018-enum-no-standard-underlying-type.md) | What happens when no standard integer type holds all of an unscoped enumeration's values? | provisional | impl-defined |
| [Q-0019](Q-0019-repeated-using-declaration-in-block.md) | May a using declaration be repeated in a block? | provisional | yes, as the newest Clang and GCC allow |
| [Q-0020](Q-0020-unimplemented-features.md) | How should tests record a compiler that doesn't implement a feature yet? | provisional | per-compiler expectation naming this question |
| [Q-0021](Q-0021-misplaced-standard-attribute.md) | Is a standard attribute applied to the wrong kind of entity rejected or ignored? | provisional | rejected |
| [Q-0022](Q-0022-func-outside-function.md) | Is `__func__` outside a function rejected? | provisional | yes |
| [Q-0023](Q-0023-gcc14-deleted-main.md) | Is `int main() = delete;` rejected? | compiler defect | yes; GCC 14's acceptance is fixed in GCC 16 |
| [Q-0024](Q-0024-main-parameter-lists.md) | Which parameter lists may `main` have? | provisional | () and (int, char**) everywhere; others implementation-defined |
| [Q-0025](Q-0025-constinit-placement.md) | Which declarations of a variable must carry `constinit`? | provisional | the initializing declaration, if any declaration does |
| [Q-0026](Q-0026-range-for-storage-class.md) | May a range-based for variable have a storage class? | compiler defect | no; GCC 14's acceptance is fixed in GCC 16 |
| [Q-0027](Q-0027-mixed-ref-qualifiers.md) | May a member function with a ref-qualifier be overloaded with one without? | provisional | no; the declarations are rejected |
| [Q-0028](Q-0028-float-narrowing-out-of-range.md) | Is converting a floating value above the target's finite range implementation-defined or undefined? | provisional | implementation-defined |
| [Q-0029](Q-0029-chained-comparison.md) | Is a chained comparison such as `3 > 2 > 1` valid? | provisional | valid; a warning is recommended |
| [Q-0030](Q-0030-pointer-relational-zero.md) | May a pointer be compared with `<` or `<=>` to the literal 0? | provisional | no; rejected |
| [Q-0031](Q-0031-pointer-outside-array.md) | Is forming a pointer outside an array undefined, even without a read through it? | provisional | undefined |
| [Q-0032](Q-0032-braced-compound-assignment.md) | May the right operand of a compound assignment be a braced list? | provisional | no; rejected |
| [Q-0033](Q-0033-string-literal-identity.md) | May constant evaluation compare the addresses of two string literals? | provisional | no; rejected |
| [Q-0034](Q-0034-subscript-prvalue-array.md) | Is a subscript of an array member of a prvalue an xvalue? | provisional | yes; xvalue |
| [Q-0035](Q-0035-delete-void-pointer.md) | Is `delete` of a `void*` rejected? | provisional | yes |
| [Q-0036](Q-0036-new-floating-size.md) | May the size in an array `new` have floating-point type? | provisional | yes; converted to the type of `sizeof` |
| [Q-0037](Q-0037-capture-default-outside-function.md) | May a lambda outside a function have a capture default? | provisional | no; rejected |
| [Q-0038](Q-0038-parameter-in-unevaluated-default.md) | May a default argument name a parameter in an operand that is not evaluated? | provisional | option 1 |
| [Q-0039](Q-0039-indeterminate-unsigned-char.md) | Is copying an uninitialized `unsigned char` undefined? | provisional | option 1 |
| [Q-0040](Q-0040-aggregate-paren-reference-member.md) | Does parenthesized aggregate initialization extend the life of a temporary bound to a reference member? | provisional | option 1 |
| [Q-0041](Q-0041-jump-past-nontrivial-destructor.md) | May a jump skip a class object that has a non-trivial destructor and no initializer? | provisional | option 1 |
| [Q-0042](Q-0042-bit-field-narrowing.md) | Is converting a bit-field whose values all fit the target a narrowing conversion? | provisional | option 1 |
| [Q-0043](Q-0043-paren-brace-elision.md) | Does parenthesized aggregate initialization allow brace elision? | provisional | option 1 |
| [Q-0044](Q-0044-for-condition-destruction.md) | Is a `for` condition variable destroyed before or after the iteration expression? | compiler defect | after; GCC 14 is fixed in GCC 16 |
| [Q-0045](Q-0045-anonymous-union-unnamed-namespace.md) | Must an anonymous union in an unnamed namespace be declared `static`? | provisional | option 1 |
| [Q-0046](Q-0046-likely-and-unlikely.md) | Is `[[likely]] [[unlikely]]` on one statement rejected or ignored? | provisional | option 1 |
| [Q-0047](Q-0047-using-enum-opaque.md) | May `using enum` name an enumeration that is declared but not defined? | provisional | option 1 |
| [Q-0048](Q-0048-tuple-size-without-value.md) | If `std::tuple_size<E>` is complete but has no `value`, do structured bindings use the data members? | provisional | option 1 |
| [Q-0049](Q-0049-base-pointer-derived-array.md) | Is arithmetic on a base class pointer into an array of a derived class undefined? | provisional | option 1 |
| [Q-0050](Q-0050-three-way-enum-float.md) | May `<=>` compare an unscoped enumeration with a floating-point value? | provisional | option 1 |
| [Q-0051](Q-0051-one-past-end-adjacent.md) | Is comparing a one-past-the-end pointer with a pointer to the next object rejected during constant evaluation? | provisional | option 1 |
| [Q-0052](Q-0052-using-declaration-reduces-access.md) | Can a using-declaration make a public base member private? | provisional | option 1 |
| [Q-0053](Q-0053-delegation-cycle.md) | Must an implementation diagnose an indirect delegation cycle? | provisional | option 1 |
| [Q-0054](Q-0054-double-destruction-constant.md) | Is a second destruction rejected during constant evaluation? | provisional | option 1 |
| [Q-0055](Q-0055-defaulted-parameter-mismatch.md) | Is a defaulted copy or move constructor with a different parameter type deleted or invalid? | provisional | option 1 |
| [Q-0056](Q-0056-delete-without-virtual-destructor.md) | Is delete through a base pointer without a virtual destructor rejected during constant evaluation? | provisional | option 1 |
| [Q-0057](Q-0057-anonymous-union-unnamed-namespace.md) | Must an anonymous union in an unnamed namespace be static? | provisional | option 1 |
| [Q-0058](Q-0058-unnamed-bitfield-cv.md) | Can an unnamed bit-field have a cv-qualifier? | provisional | option 1 |
| [Q-0059](Q-0059-deduced-conversion-function.md) | Can a conversion function have a deduced return type? | provisional | option 1 |
| [Q-0060](Q-0060-literal-operator-default-argument.md) | Can a literal operator have a default argument? | provisional | option 1 |
| [Q-0061](Q-0061-comparison-declared-before-default.md) | Can a friend comparison declared outside the class be defaulted in it? | provisional | option 1 |
| [Q-0062](Q-0062-out-of-class-default-deleted.md) | Must an implementation diagnose a comparison defaulted outside its class that would be deleted? | provisional | option 1 |
| [Q-0063](Q-0063-floating-literal-out-of-range.md) | Is a floating literal outside the range of its type valid? | provisional | option 1 |
| [Q-0064](Q-0064-decimal-literal-too-large-for-signed.md) | Is a decimal literal that fits only an unsigned type valid? | provisional | option 1 |
| [Q-0065](Q-0065-decltype-declarative-nested-name.md) | Can decltype name the class in a member definition outside the class? | provisional | option 1 |
| [Q-0066](Q-0066-member-changes-meaning.md) | Must an implementation diagnose a class member that changes the meaning of a name? | provisional | option 1 |
| [Q-0067](Q-0067-typedef-name-non-c-class.md) | Can a typedef name an unnamed class that is not C-like? | provisional | option 1 |
| [Q-0068](Q-0068-inline-after-definition.md) | Can a function be declared inline after its definition? | provisional | option 1 |
| [Q-0069](Q-0069-alignas-weaker-than-natural.md) | Can alignas make an alignment weaker than the natural one? | provisional | option 1 |
| [Q-0070](Q-0070-alignas-redeclaration-mismatch.md) | Must all declarations of a variable agree on alignas? | provisional | option 1 |
| [Q-0071](Q-0071-several-alignas-on-class.md) | Which alignas applies when a class has several? | provisional | option 1 |
| [Q-0072](Q-0072-alignas-invalid-positions-gcc14.md) | Can alignas apply to a function, a type alias, or a bit-field? | provisional | option 1 |
| [Q-0073](Q-0073-alignas-on-enumeration.md) | Can alignas apply to an enumeration? | provisional | option 1 |
| [Q-0074](Q-0074-language-linkage-in-function-type.md) | Is language linkage part of a function type? | provisional | option 1 |
| [Q-0075](Q-0075-c-linkage-conflicts-global.md) | Can a name with C language linkage match a global variable? | provisional | option 1 |
| [Q-0076](Q-0076-c-linkage-variable-main.md) | Can main, or a variable named main, have C language linkage? | provisional | option 1 |
| [Q-0077](Q-0077-throw-empty-specification.md) | Is throw() a valid exception specification? | provisional | option 1 |
| [Q-0078](Q-0078-noexcept-operand-narrowing.md) | Can the operand of noexcept narrow when it is converted to bool? | provisional | option 1 |
| [Q-0079](Q-0079-noexcept-operand-cycle.md) | Must an implementation diagnose a noexcept operand that depends on itself? | provisional | option 1 |
| [Q-0080](Q-0080-static-const-volatile-initializer.md) | Can a static const volatile member have an initializer in its class? | provisional | option 1 |
| [Q-0081](Q-0081-single-element-same-class-list.md) | Does a braced list with one object of the same class use the initializer-list constructor? | provisional | option 1 |
| [Q-0082](Q-0082-array-new-deleted-destructor.md) | Can new create an array of a class whose destructor is deleted? | provisional | option 1 |
| [Q-0083](Q-0083-nested-default-member-init-early.md) | Can a default member initializer of a nested class be used before the enclosing class is complete? | provisional | option 1 |
| [Q-0084](Q-0084-reference-member-temporary.md) | Must an implementation reject a constructor that binds a reference member to a temporary? | provisional | option 1 |
| [Q-0085](Q-0085-constexpr-destructor-virtual-base.md) | Can a class with a virtual base have a constexpr destructor? | provisional | option 1 |
| [Q-0086](Q-0086-call-returning-abstract.md) | Is a call to a function that returns an abstract class valid? | provisional | option 1 |
| [Q-0087](Q-0087-explicit-object-matches-virtual.md) | Can an explicit object member function have the name and parameters of a base virtual function? | provisional | option 1 |
| [Q-0088](Q-0088-union-user-assignment-active.md) | Does assignment through a user-provided operator activate an inactive union member during constant evaluation? | provisional | option 1 |
| [Q-0089](Q-0089-decltype-in-destructor-call.md) | Can decltype name the class in an explicit destructor call? | provisional | option 1 |
| [Q-0090](Q-0090-unknown-reference-constant.md) | Can constant evaluation use a reference whose referent is unknown, if it does not read through it? | provisional | option 1 |
| [Q-0091](Q-0091-delete-array-through-base-constant.md) | Is delete[] of a derived array through a base pointer rejected during constant evaluation? | provisional | option 1 |
| [Q-0092](Q-0092-lambda-capture-trailing-return.md) | Do names in a lambda's trailing return type find its captures? | provisional | option 1 |
| [Q-0093](Q-0093-class-key-qualified-declaration.md) | Is a declaration of only a class key and a qualified name valid? | provisional | option 1 |
| [Q-0094](Q-0094-class-operator-delete-noexcept.md) | Is a class-specific operator delete without a specifier non-throwing? | provisional | option 1 |
| [Q-0095](Q-0095-defaulted-comparison-member-throwing.md) | Is a defaulted == non-throwing if a member == is potentially throwing? | provisional | option 1 |
| [Q-0096](Q-0096-case-label-ends-block.md) | Can a case or default label be the last item of a block? | provisional | option 1 |
| [Q-0097](Q-0097-jump-skips-nontrivial-destructor.md) | Can a jump skip a variable whose destructor is not trivial? | provisional | option 1 |
| [Q-0098](Q-0098-range-for-data-member-begin.md) | Do data members named begin and end select the member form of a range-based for? | provisional | option 1 |
| [Q-0099](Q-0099-array-xvalue-subscript.md) | Is a subscript of an array xvalue an xvalue? | provisional | option 1 |
| [Q-0100](Q-0100-char-zero-null-pointer.md) | Is a character literal with value zero a null pointer constant? | provisional | option 1 |
| [Q-0101](Q-0101-nullptr-logical-operand.md) | Can nullptr be an operand of a logical operator? | provisional | option 1 |
| [Q-0102](Q-0102-conditional-zero-nullptr.md) | Does a conditional with the literal 0 and nullptr have the type of nullptr? | provisional | option 1 |
| [Q-0103](Q-0103-relational-nullptr-type.md) | Can two values of the type of nullptr be compared with a relational operator? | provisional | option 1 |
| [Q-0104](Q-0104-object-and-first-member-equal.md) | Do a class object and its first member compare equal through void pointers? | provisional | option 1 |
| [Q-0105](Q-0105-block-extern-hidden-internal.md) | Does a block-scope extern declaration redeclare a hidden internal-linkage variable? | provisional | option 1 |
| [Q-0106](Q-0106-reference-to-qualified-function-type.md) | Can a parameter have a reference to a function type with a ref-qualifier? | provisional | option 1 |
| [Q-0107](Q-0107-constinit-structured-binding.md) | Can constinit apply to a structured binding declaration? | provisional | option 1 |
| [Q-0108](Q-0108-carries-dependency-first-declaration.md) | Must carries_dependency be on the first declaration of a function? | provisional | option 1 |
| [Q-0109](Q-0109-extend-through-array-member.md) | Does a reference bound to an element of an array member of a temporary extend the temporary? | provisional | option 1 |
| [Q-0110](Q-0110-null-dereference-reference-constant.md) | Is a reference bound through a null pointer rejected during constant evaluation? | provisional | option 1 |
| [Q-0111](Q-0111-using-sees-later-default.md) | Does a using declaration see a default argument that a later namespace declaration adds? | provisional | option 1 |
| [Q-0112](Q-0112-main-parameter-types.md) | Must the parameters of main have the types int and char**? | provisional | option 1 |
| [Q-0113](Q-0113-scope-of-overload-resolution-and-templates.md) | Should Tidal specify overload resolution and templates in full? | provisional | yes, after phases 1 and 2 |
| [Q-0114](Q-0114-type-of-a-growing-enumerator.md) | What type does an enumerator get when its value outgrows the type before it? | provisional | unspecified: any integer type that holds the value |
| [Q-0115](Q-0115-gcc-copy-of-object-with-unset-part.md) | Does GCC accepting a copy of an object with an unset part during constant evaluation change CE-22? | compiler defect | no; likely GCC defect |
| [Q-0116](Q-0116-designer-persona-as-source-reader.md) | Should a designer persona that knows WG21 papers also run, as a source reader? | provisional | no, the clean-room designer only for now |
| [Q-0117](Q-0117-constant-union-with-no-active-member.md) | Is a union with no active member a valid constant? | provisional | yes; GCC rejects it |
| [Q-0118](Q-0118-instantiation-before-deduction.md) | Should instantiation (N2) come before deduction (O) in phase 4? | provisional | no, O first |
| [Q-0119](Q-0119-diagram-format.md) | Should diagrams use a renderer such as Mermaid, or plain text? | provisional | plain text |
| [Q-0120](Q-0120-edg-i686-long-long-alignment.md) | Does EDG's 8-byte alignment of `long long` on i686 change TYP-2? | provisional | no; the alignment is implementation-defined |
| [Q-0121](Q-0121-write-to-skipped-variable-in-constant-evaluation.md) | Can constant evaluation write a variable whose declaration a jump skipped? | provisional | yes; GCC and EDG reject it |
| [Q-0122](Q-0122-empty-init-in-range-for.md) | May a range-based `for` have an empty init statement? | provisional | yes; GCC rejects it |
| [Q-0123](Q-0123-attribute-position-in-pointer-declarator.md) | Does an attribute in a pointer declarator come before or after the cv-qualifiers? | provisional | after; Clang differs |
| [Q-0124](Q-0124-increment-narrow-signed-out-of-range.md) | Is `++` or `--` on a signed operand narrower than `int` defined when the new value does not fit the operand's type? | provisional | the step is defined and the value converts back; EDG differs |
| [Q-0125](Q-0125-clang18-sized-deallocation.md) | Does Clang 18 omitting the sized deallocation functions change NEW-19? | compiler defect | no; Clang 22 declares them |
| [Q-0126](Q-0126-const-object-union-member-value.md) | When does a union or an anonymous union give its members a value for a `const` object without an initializer? | provisional | a union gives a value when one variant member has a default member initializer; a class of only anonymous unions is valid; the compilers split |
| [Q-0127](Q-0127-include-guard-or-pragma-once.md) | Which mechanism does Tidal specify for including a header only once? | provisional | both; the include guard is required and `#pragma once` is implementation-defined |
| [Q-0128](Q-0128-internal-linkage-missing-definition.md) | Must an implementation diagnose a used function with internal linkage that has no definition? | provisional | no; the rule needs no diagnostic and a warning is recommended; GCC is silent when the only use is in an uncalled inline function |
| [Q-0129](Q-0129-copy-of-class-constant.md) | Is a copy of a whole class-type constant a use of the constant? | provisional | no; Clang differs |
| [Q-0130](Q-0130-edg-enclosing-constant-use.md) | Does EDG's acceptance of some uses of an enclosing function's constant change ODR-2? | provisional | no; EDG differs |
| [Q-0131](Q-0131-hidden-friend-qualified-redeclaration.md) | Can a declaration with a qualified name outside the namespace redeclare a friend function that only a class has declared? | provisional | no; GCC and EDG reject, Clang accepts |
| [Q-0132](Q-0132-explicit-constructor-in-list-sequence.md) | Does a braced list have a conversion sequence to a class parameter when the chosen constructor is `explicit`? | provisional | yes; GCC agrees, Clang and EDG form no sequence |
| [Q-0133](Q-0133-initializer-list-sequence-before-kind.md) | Does a list sequence to `std::initializer_list` beat a standard sequence when its own sequence is user-defined? | provisional | yes; GCC and EDG agree, Clang compares kind first |
| [Q-0134](Q-0134-bit-field-promotion-rank.md) | Does a bit-field narrower than `int` but declared with a wider type have a promotion to `int` in a conversion sequence? | provisional | yes; Clang agrees, GCC and EDG differ |
| [Q-0135](Q-0135-scalar-against-array-reference.md) | Does a braced list prefer a scalar parameter to a reference to an array when both sequences are the identity? | provisional | no, they tie; GCC differs |
| [Q-0136](Q-0136-list-to-array-reference-edge-cases.md) | Do empty braces or a designated list have a sequence to a reference to an array? | provisional | no; Clang agrees, GCC and EDG differ |
| [Q-0137](Q-0137-array-argument-pointer-tie-breakers.md) | Do the pointer tie-breakers apply after an array-to-pointer step? | provisional | yes; Clang and EDG agree, GCC differs |
| [Q-0138](Q-0138-function-reference-kind-tie-breaker.md) | Does the kind of a function reference break a tie between two conversion functions? | provisional | yes; GCC and EDG agree, Clang differs |
| [Q-0139](Q-0139-surrogates-with-one-parameter-list.md) | Is a call ambiguous when two surrogates differ only in the qualifiers of their conversion functions? | provisional | yes; Clang and EDG agree, GCC differs |
| [Q-0140](Q-0140-default-argument-from-two-scopes.md) | Is a call valid when it uses a default argument that two scopes give? | provisional | no, if the defaults differ; GCC agrees, Clang and EDG differ |
| [Q-0141](Q-0141-builtin-candidate-operand-conversion.md) | After a built-in candidate wins, does a class operand convert past its conversion function? | provisional | no; Clang and GCC agree, EDG differs |
| [Q-0142](Q-0142-constraints-in-address-of-overload-set.md) | Does the more constrained function win when a target type selects from an overload set? | provisional | yes; Clang 22 and GCC 16 agree, EDG, Clang 18 and GCC 14 differ |
| [Q-0143](Q-0143-call-through-address-with-static-member.md) | Can a call through the address of an overload set choose a static member? | provisional | yes; Clang 22 agrees, Clang 18 and GCC differ, EDG has an internal error |
| [Q-0144](Q-0144-const-object-surrogate.md) | Does a const object get a surrogate from a conversion function that is not const? | provisional | no; Clang and EDG agree, GCC differs |
| [Q-0145](Q-0145-copy-list-init-explicit-conversion-function.md) | Can copy-list-initialization use an explicit conversion function for the copy constructor's parameter? | provisional | no; GCC and EDG agree, Clang differs |
| [Q-0146](Q-0146-constraints-between-templates-with-different-parameters.md) | Do constraints decide between two templates whose function parameters differ? | provisional | no; Clang and GCC agree, EDG differs |
| [Q-0147](Q-0147-rewritten-candidate-removes-builtin.md) | Does a rewritten candidate remove the built-in candidate with the same parameter list? | provisional | yes; Clang and EDG agree, GCC differs |
| [Q-0148](Q-0148-reference-kind-against-template.md) | Does the function reference kind come before the template tie-breakers? | provisional | yes, the kind comes first; GCC agrees, EDG differs, Clang does not apply the kind |
| [Q-0149](Q-0149-list-for-reference-to-unknown-bound-array.md) | What type does a braced list give a reference to an array of unknown bound? | provisional | the reference keeps its declared type, and empty braces are rejected; Clang differs on the type, GCC on empty braces |
| [Q-0150](Q-0150-explicit-constructor-for-reference-temporary.md) | Can a braced list for a reference use an explicit constructor to make the temporary? | provisional | no; GCC and EDG agree, Clang differs |
| [Q-0151](Q-0151-parenthesized-list-for-reference.md) | Can a parenthesized list of several values initialize a reference to an aggregate? | provisional | no; Clang and EDG agree, GCC differs |
| [Q-0152](Q-0152-same-rank-tie-breakers-before-rvalue-reference.md) | Do the same-rank tie-breakers come before the rvalue-reference preference, and does that preference come before the qualification step? | provisional | yes to both; Clang agrees, GCC and EDG differ in part |
| [Q-0153](Q-0153-qualification-step-edges.md) | Does the qualification step compare array bounds, and does it compare a reference that binds a temporary? | provisional | no for bounds, yes for temporaries; Clang differs on bounds, EDG on temporaries |
| [Q-0154](Q-0154-list-element-nearest-base.md) | Does the nearest-base tie-breaker apply to a braced list with one element of a derived class? | provisional | yes; Clang and GCC agree, EDG differs |
| [Q-0155](Q-0155-braced-string-literal-for-array-reference.md) | When does one braced string literal have a sequence to a reference to a character array? | provisional | when the literal can initialize the array; GCC differs on UTF-8, Clang on size and bound |
| [Q-0156](Q-0156-explicit-conversion-value-for-reference.md) | Can direct initialization of a reference use an explicit conversion function that returns a value? | provisional | no; GCC and EDG agree, Clang differs |
| [Q-0157](Q-0157-list-class-element-for-reference.md) | Does a braced list with one class element bind a reference through the element's conversion function? | provisional | no; a temporary unless the element has the referenced type or a derived class; Clang and EDG agree, GCC differs |
| [Q-0158](Q-0158-equality-returning-reference-to-bool.md) | Can an `==` used for a reversed or rewritten comparison return a reference to `bool`? | provisional | no; Clang and EDG agree, GCC differs |
| [Q-0159](Q-0159-current-object-of-another-class.md) | Is `*this` the implied object argument when its class is not the class of the members found? | provisional | no, the call has no object; GCC agrees, Clang and EDG differ |
| [Q-0160](Q-0160-constraints-without-target-type.md) | Do constraints reduce an overload set whose address has no target type? | provisional | yes; Clang 22 and GCC 16 agree, GCC 14 and EDG differ |
| [Q-0161](Q-0161-address-of-member-set-other-forms.md) | Can a call through `&(C::f)` or `&f` use a set that holds a non-static member? | provisional | no; GCC agrees, Clang 22 and EDG differ in part |
| [Q-0162](Q-0162-constraint-tie-breaker-parameter-lists.md) | Does the constraint tie-breaker apply when the parameter lists differ beyond the parameters that the call uses? | provisional | no; Clang 22 and GCC agree, EDG differs |
| [Q-0163](Q-0163-conversion-result-function-pointer-conversion.md) | Does the result tie-breaker prefer the identity to a function pointer conversion? | provisional | yes; Clang and GCC agree, EDG differs |
| [Q-0164](Q-0164-braced-array-for-unknown-bound-reference.md) | Does one braced array lvalue bind a reference to an array of unknown bound directly? | provisional | yes; Clang and EDG agree, GCC makes a temporary |
| [Q-0165](Q-0165-braced-function-for-function-reference.md) | Does one braced function name bind a reference to a function as `=` does? | provisional | yes; GCC rejects an overload set, EDG a noexcept function |
| [Q-0166](Q-0166-braced-string-literal-reference-temporary.md) | Can one braced string literal initialize the temporary array of a reference? | provisional | yes, under LIST-6; Clang and EDG agree, GCC rejects |
| [Q-0167](Q-0167-braced-string-literal-char-array-overloads.md) | Does one braced string literal prefer a reference to a `char` array over one to an `unsigned char` array? | provisional | no, ambiguous; Clang and EDG agree, GCC picks `char` |
| [Q-0168](Q-0168-alias-template-same-as-class-template.md) | Is an alias template that passes its parameters through the same template argument as its class template? | provisional | unspecified; Clang says no, GCC and EDG say yes |
| [Q-0169](Q-0169-template-template-argument-extra-defaults.md) | Can a template template argument have more parameters than the parameter, if the extra ones have defaults? | provisional | yes; Clang 22, GCC, and EDG agree, Clang 18 rejects |
| [Q-0170](Q-0170-type-or-expression-for-constant-parameter.md) | Is `int()` a type or a value when the parameter is a constant parameter? | provisional | a type; Clang and GCC agree, EDG reads the value 0 |
| [Q-0171](Q-0171-explicit-specialization-in-class.md) | Can an explicit specialization of a member template be declared in the class? | provisional | yes; Clang and EDG agree, GCC rejects |
| [Q-0172](Q-0172-function-partial-specialization-diagnosed-on-use.md) | Must an implementation reject a partial specialization of a function template that no call uses? | provisional | yes; Clang and GCC agree, EDG rejects only at a use |
| [Q-0173](Q-0173-partial-specialization-after-use.md) | Must an implementation diagnose a partial specialization declared after a use that it would match? | provisional | no, no diagnostic required; GCC and EDG reject, Clang accepts |
| [Q-0174](Q-0174-partial-specialization-not-more-specialized.md) | Is a partial specialization valid when it is not more specialized than the primary template? | provisional | no; Clang rejects, GCC and EDG accept |
| [Q-0175](Q-0175-static-class-member-read-before-initialization.md) | Can a dynamic initializer read the zero in a member of a later class-type variable? | provisional | no; the read is undefined (OBJ-2) |
| [Q-0176](Q-0176-volatile-initialization-observable.md) | Is the initialization of a volatile object an observable write? | provisional | yes, for an initializer that runs at run time |
| [Q-0177](Q-0177-attribute-before-base-and-after-new-bound.md) | Can an attribute specifier come before a base class and after the array bound of a `new` expression? | compiler defect | yes; GCC 14 defect |
| [Q-0178](Q-0178-call-through-pointer-differing-in-noexcept.md) | Is a call through a function pointer whose type differs from the function only in `noexcept` undefined? | provisional | yes, undefined |
| [Q-0179](Q-0179-throw-in-noexcept-warning-scope.md) | Which throws in a non-throwing function should get a warning? | provisional | those that no enclosing handler catches by static type; a rethrow needs `catch (...)` |
| [Q-0180](Q-0180-dangling-reference-warnings.md) | Which dangling references should an implementation warn about? | provisional | a general recommended rule, tested on the `new` case |
| [Q-0181](Q-0181-clang18-explicit-object-from-prvalue.md) | Does a class prvalue initialize a by-value explicit object parameter without a copy or move? | compiler defect | yes; Clang 18's rejection is fixed in Clang 22 |
| [Q-0182](Q-0182-evidence-reasons-for-implementation-choices.md) | May Evidence give the reason for an implementation's choice? | provisional | only when a clean input states the reason |
| [Q-0183](Q-0183-cost-model-for-proposals.md) | Should Tidal give proposals a way to state implementation cost? | provisional | not yet; front-end counts once Tidal has a corpus of whole programs |
| [Q-0184](Q-0184-vendor-options-in-evidence.md) | Should Evidence record vendor options that change behavior? | provisional | yes, when a test runs the compiler with the option |
| [Q-0185](Q-0185-change-notation-and-versions.md) | How does Tidal mark a change to a rule and the language version a rule holds for? | provisional | a change table for proposals now; a version field when a second version is specified |
| [Q-0186](Q-0186-proposals-and-outside-judgments.md) | May a proposal rest on judgments from outside Tidal? | provisional | yes, marked as outside knowledge |

## Status values

- **open**: no provisional answer yet. The spec marks affected rules as open.
- **provisional**: the spec follows the provisional answer. Opinions welcome.
- **compiler defect**: the provisional answer is that one compiler, or an
  older version of one, is wrong, and the spec follows the others. The
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
"Provisional answer" section, the index above, and the affected spec rules
and tests. The test suite must still pass on baseline compilers.

**Raise a new question.** Copy `TEMPLATE.md` to the next free number and add
it to the index.

## Ground rules

- Argue from evidence anyone can check: test programs, compiler behavior,
  and implementation source.
- Don't quote, paste, or paraphrase the ISO standard, its drafts, WG21
  papers, or cppreference. Pull requests that do will be closed. The
  project's input rules are in `docs/clean-room.md`.
- One question per file. Keep opinions in the file they answer.
