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
