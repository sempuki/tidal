# Q-0346: Can an explicit instantiation of a variable state a type that differs from the specialization's?

**Status:** provisional
**Affects:** INST-22, `tests/templates/instantiation/explicit-variable-wrong-type.cpp`, `tests/templates/instantiation/explicit-array-bound-differs.cpp`
**Waits for:** a review (doubt); newest families that agree

## Context

`template <class T> int value = 1;` is explicitly instantiated as
`template long value<int>;`. `template <class T> int table[] = {1, 2,
3};` is explicitly instantiated as `template int table<int>[4];`, and
the specialization has the type `int[3]`.

| Compiler | `long` for `int` | `long[3]` for `int[3]` | bound 4, template has no bound | bound 4, template has bound 3 |
|---|---|---|---|---|
| Clang 18, 22, host | rejects: the type does not match | rejects | rejects | rejects |
| EDG 7.0 | rejects: the declaration is incompatible | rejects | accepts | rejects |
| GCC 14, 16, host | accepts | accepts | accepts | accepts |

For a function template, every compiler rejects a return type that
differs (`explicit-wrong-type.cpp`).

## Evidence

The tests above and probes of 2026-11-03. After `template int
table<int>[4];`, GCC and EDG still give `table<int>` the size of three
`int` objects, and a `static_assert` of the size of four fails on every
compiler. GCC also keeps the type `int` after `template long
value<int>;`. So the compilers that accept the program ignore the
stated type and keep the specialization's type. EDG rejects a
bound that differs only when the template itself is declared with a
bound. Its results fit a comparison with the template's declared type
`int[]`, which has no bound (Q-0236).

## Options

1. The stated type must be the specialization's type, and only the
   allowances of INST-22 apply (Clang; EDG except for a bound where the
   template has none).
2. The stated type of a variable is not checked (GCC).
3. A bound can differ when the template's declared type has no bound
   (EDG).

## Provisional answer

No; Clang agrees, GCC and EDG each differ on some cases (option 1). Two of three implementations agree on each case except the bound where the template has none, where GCC and EDG accept. INST-P3 supports option 1. An explicit instantiation names a specialization that already has a type, and the compilers that accept a different type keep the specialization's type, so the stated type has no effect.

The question stays open under decision 0051 (2026-11-03). GCC 16 accepts every case, and EDG 7.0 accepts the bound where the template has none, so two families disagree with Clang 22.

Kept on 2026-11-03. The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) agrees with option 1, with doubt for a bound where the template states none. The review's reading of how an initializer fixes the bound points to option 1, and Clang 22 follows it. INST-22 stays.

Kept open on 2026-11-03 under decision 0054, because the gap review does not support the answer with no doubt (it reads with doubt, weakly, unclearly, or for only part of the question).

Noted on 2026-11-05 under decision 0058, item PK2. A test whose notes name the question has two newest families that differ from the answer, so the question also waits for newest families that agree (`pending.md` row 1). The check of `tools/question_index.py` found the case.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) reads the reference as supporting option 1, with doubt. The explicit instantiation and the declaration made from the template declare one entity, and only the presence of the first bound may differ. A different element type, a different whole type, and a bound that differs from a stated bound are not valid, and a diagnostic is required, so GCC's acceptance is a defect. The doubt is the form where the template states no bound. The reference does not say clearly whether the declaration made from the template has its written type, with no bound, or the complete type that the initializer gives. The first reading makes the form valid, as EDG finds. The rule that an initializer defines the bound points to the second reading, which is option 1.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
