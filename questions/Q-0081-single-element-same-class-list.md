# Q-0081: Does a braced list with one object of the same class use the initializer-list constructor?

**Status:** settled
**Affects:** ILC-3, `tests/classes/initializer-lists/copy-from-single-same-class.cpp`
**Parent:** Q-0294, the model of braced lists for references, arrays, and conversion sequences

## Context

A class `S` has a copy constructor and a constructor that takes
`std::initializer_list<S>`. The program writes `S b{a};` where `a` is an
`S`. Clang 22 and GCC call the initializer-list constructor. Clang 18
calls the copy constructor. EDG does not run programs.

## Options

1. The initializer-list constructor is used, as for other braced lists.
2. The copy constructor is used.

## Answer

Yes; the initializer-list constructor is used, as for other braced lists (option 1). The newer Clang agrees with GCC.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) supports this answer with no doubt, and ILC-P2 gives the same answer: a class that accepts a `std::initializer_list` tries the initializer-list constructor first, and the elements become separate arguments only when that constructor does not fit. No newest family differs. Clang 18 calls the copy constructor, and the test keeps its override for Clang 18. EDG does not run the program.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. The special case for a braced list with one element of the same class applies only to aggregates. The class in the question is not an aggregate, so the initializer-list constructor is tried first.

### Inventory, 2026-10-31

The source-reader inventory of conversion sequences (`reports/review/2026-10-31-inventory-conversion-sequences.md`) names the question for a related case, a one-element list of the same class, and says only that compilers differ when the class has an initializer-list constructor that could take the element. It gives no reading.

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) names the question only to say that it did not review it, because the question belongs to another cluster.
