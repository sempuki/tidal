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
