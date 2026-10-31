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
| [Q-0002](Q-0002-edg-floating-division-by-zero.md) | Does EDG accepting floating division by zero during constant evaluation change MUL-11? | provisional | no; likely EDG defect |
| [Q-0003](Q-0003-edg-int-min-divided-by-minus-one.md) | Does EDG accepting `INT_MIN / -1` during constant evaluation change MUL-10? | provisional | no; likely EDG defect |
| [Q-0004](Q-0004-floating-division-by-zero-at-run-time.md) | Should floating division by zero be implementation-defined on IEEE 754 targets? | provisional | no, keep undefined |
| [Q-0005](Q-0005-operand-order-without-observed-variation.md) | Can operand order be `unspecified` when no compiler varies it? | provisional | yes, on source evidence |
| [Q-0006](Q-0006-promoting-report-compilers.md) | When does a report compiler become a baseline compiler? | provisional | after one clean review of its results |
| [Q-0007](Q-0007-language-version.md) | Which language version does Tidal track, and how are newer features handled? | provisional | C++23; newer features rejected under strict mode |
| [Q-0008](Q-0008-similarity-thresholds.md) | Are the similarity thresholds (8-word run, 0.25 overlap) right? | provisional | keep for now |
| [Q-0009](Q-0009-license.md) | Which license covers the specification, tests, and tools? | settled | Apache-2.0 with LLVM exception |
| [Q-0010](Q-0010-decltype-auto-braced-single.md) | Is `decltype(auto) x{e};` valid, deducing from `e`? | provisional | yes, as all implementations do |
| [Q-0011](Q-0011-missing-definition-class.md) | Is using an entity with no definition `no-diagnostic` or `reject`? | provisional | no-diagnostic |
| [Q-0012](Q-0012-largest-array.md) | Should Tidal set a minimum for the largest array an implementation accepts? | provisional | no; the limit is implementation-defined |
| [Q-0013](Q-0013-edg-uninitialized-array-element.md) | Does EDG accepting a read of an uninitialized array element during constant evaluation change INIT-1? | provisional | no; likely EDG defect |
| [Q-0014](Q-0014-early-dynamic-initialization.md) | May an implementation perform dynamic initialization before the program starts? | provisional | yes, unspecified |
| [Q-0015](Q-0015-clang18-braced-structured-binding.md) | Is `auto [a, b]{arr};` valid when `arr` is an array? | provisional | yes; Clang 18's rejection is fixed in later versions |
| [Q-0016](Q-0016-constrained-structured-binding.md) | May a structured binding use a constrained `auto`? | provisional | no; likely EDG defect |
| [Q-0017](Q-0017-enum-value-out-of-range.md) | Is converting a value outside an enumeration's range undefined? | provisional | undefined |
| [Q-0018](Q-0018-enum-no-standard-underlying-type.md) | What happens when no standard integer type holds all of an unscoped enumeration's values? | provisional | impl-defined |
| [Q-0019](Q-0019-repeated-using-declaration-in-block.md) | May a using declaration be repeated in a block? | provisional | yes, as the newest Clang and GCC allow |
| [Q-0020](Q-0020-unimplemented-features.md) | How should tests record a compiler that doesn't implement a feature yet? | provisional | per-compiler expectation naming this question |
| [Q-0021](Q-0021-misplaced-standard-attribute.md) | Is a standard attribute applied to the wrong kind of entity rejected or ignored? | provisional | rejected |
| [Q-0022](Q-0022-func-outside-function.md) | Is `__func__` outside a function rejected? | provisional | yes |
| [Q-0023](Q-0023-gcc14-deleted-main.md) | Is `int main() = delete;` rejected? | provisional | yes; GCC 14's acceptance is fixed in GCC 16 |
| [Q-0024](Q-0024-main-parameter-lists.md) | Which parameter lists may `main` have? | provisional | () and (int, char**) everywhere; others implementation-defined |
| [Q-0025](Q-0025-constinit-placement.md) | Which declarations of a variable must carry `constinit`? | provisional | the initializing declaration, if any declaration does |
| [Q-0026](Q-0026-range-for-storage-class.md) | May a range-based for variable have a storage class? | provisional | no; GCC 14's acceptance is fixed in GCC 16 |
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
| [Q-0044](Q-0044-for-condition-destruction.md) | Is a `for` condition variable destroyed before or after the iteration expression? | provisional | after; GCC 14 is fixed in GCC 16 |
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

## Status values

- **open**: no provisional answer yet. The spec marks affected rules as open.
- **provisional**: the spec follows the provisional answer. Opinions welcome.
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
