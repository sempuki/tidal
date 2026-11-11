# Writer probes, roadmap round 13

Commit read: `a6a2b16c`. The grids ran with `tools/probe_grid.py --errors`
on the baseline compilers (Clang 18, Clang 22 in its container and on
the host, GCC 14, GCC 16 in its container and on the host, EDG 7.0, and
EDG 7.0 ILP32), each run under `flock build/run_tests.lock`, in a
worktree of the repository. `writer-scratch/make_grids.py` writes the
JSON files. Each `.out` file holds the table and the first error of each
compiler that rejects. The paths of the host compilers in the `.out`
files start at the worktree.

The grids compile only. A probe that needs a type or a value puts it in
a `static_assert`, so EDG takes part. The `p4-4-warnings` grid passes
`-Werror` with warning flags, so EDG skips it, as it skips the
warning-flag tests of the matrix.

`writer-scratch/run_probes.py` builds and runs programs on the six
compilers that build (Clang 18, Clang 22 in its container and on the
host, GCC 14, GCC 16 in its container and on the host). Its output is
`run-probes.out`.

| File | Finding | Result |
|---|---|---|
| `x25-constant.json`, `.out` | The X25 host-only claims on constant evaluation: each probe of `reports/clean-room/X25/p/` as written, with `construct.h` inlined, one row per case. `a9.cpp`'s test of validity becomes two `static_assert` rows. | Every compiler: rejects a read of the old member after a construction at an element of the array member (`a4` case 1), a read after an element's end (`a5` case 1), and a `constexpr` variable with elements outside their lifetimes (`a7` case 1); accepts a trivial copy of a union whose active array member has an element with no value (`a3`). Splits: GCC 16 and EDG accept, and Clang 18, Clang 22, and GCC 14 reject, a construction at an element of an array member that is not active (`a1`, `a6`, `m1`, `a2` cases 2 to 4, `a4` case 2, `a5` case 2), and the test of validity of `a9` is true on GCC 16 and EDG and false on the others (Q-0425); GCC 16 alone accepts the transient use of `a7` case 2, and EDG rejects it; EDG alone accepts a read of an element never built (`a2` case 1) and a read of an element with no value in the copy of a union (`a3c`); GCC alone accepts the copy of a class that is not a union (`a3b`, Q-0115); GCC 14 alone accepts `a10 -DEND`. |
| `x25-rules.json`, `.out` | X25-1, X25-5, X25-7 | Every compiler accepts `std::construct_at` of a whole member that is not active during constant evaluation, with another member active and with none (X25-1). Every compiler accepts a trivial copy, and a trivial copy assignment, of a union whose active member has a part with no value, for an `int` array member and for a member of class type (X25-5). Every newest compiler rejects a `constexpr` variable of a union or of a struct with an `int` array element ended by a pseudo-destructor call, and GCC 14 accepts both (X25-7). Splits: EDG alone accepts a read of the part with no value in the copy of the union (X25-5); GCC accepts the copy of the struct (Q-0115). |
| `p4-points.json`, `.out` | P4-1 to P4-5, P4-7, and the P4 host-only claims | Every compiler: accepts a member function defined in its class whose return type is `Box<Money>` or `List<Money, 16>`, or whose parameter type is `Box<Money>`, and rejects an in-class body whose return type is completed only after the class and a data member of type `Box<Money>` (P4-1); accepts a member `operator==` whose parameter is `std::initializer_list<int>` with a named operand and with `std::initializer_list<int>{1, 2}`, rejects a braced list as the right operand of `==`, and accepts a braced list as the operand of `operator[]` (P4-2); accepts a range-based `for` over an end type compared by `==` only, an adaptor over any sequence with the closure type as a template parameter and deduction from the aggregate, and `Window{Count{6}, 3}` as `Window<Count>` (P4-3); names the iterator type with `decltype` of a member call through a null pointer and through a declared function, and accepts a requires-expression parameter of that type (P4-4); rejects `return last;` for a `Maybe` with deleted copy functions and no move constructor, and accepts it with a move constructor and for a prvalue (P4-5); gives `19.99_usd` the value 1999 through the `long double` form plus one half, passes the `long double` form the value of `19.99L`, `0.1L`, and `1e300L`, which differs from the `double` value `19.99`, and passes the integer form `18446744073709551615` and `0x10` as 16 (P4-7). |
| `p4-4-warnings.json`, `.out` | P4-4 with `-Wall -Wextra -Werror` | GCC 14 warns "'this' pointer is null" (`-Wnonnull`) on `decltype(static_cast<const S*>(nullptr)->begin())`, used directly and through a member class of a class template. GCC 16 and Clang give no warning. The declared-function form gives no warning on any compiler. |
| `fe26-named-call.json`, `.out` | Front-end report 26, section 5: a literal operator template called by its name | Every compiler accepts `12_w`, `operator""_w<'1', '2'>()`, `operator""_w<>()`, and `operator""_w()` with the values that the `static_assert` checks (23, 23, 0, 0). |
| `q0445-template-id.json`, `.out` | RV9-R-4 and Q-0445: an attribute after the class key with a template-id | Every compiler accepts an attribute after the class key in an explicit specialization and in a partial specialization that are declared only, and in a forward declaration. GCC alone accepts one in an explicit instantiation and in a variable declaration (Q-0445). |
| `run-probes.out` | The X25 run claims and `a9`, and the P4 claims on warnings and sanitizers | On each of the six compilers, `a1 -DRUN` and `m1 -DRUN` exit 0 under `-fsanitize=address,undefined`; `a4 -DRUN` reads 7 through the member name and 3 from the old member, with no sanitizer report (X25-4); `a9` exits 1 on GCC 16 and host GCC and 0 on Clang 18, Clang 22, and GCC 14. The five P4 program tests and the two P4 reductions build and exit 0 under the sanitizers on each compiler, and under `-Wall -Wextra -pedantic-errors -Werror` on each compiler except GCC 14, which rejects `sequences.cpp` and `sequences-first-try.cpp` with the `-Wnonnull` warning of `p4-4-warnings`. |

P3-9 and P3-10 need no probe. Their tests,
`statements/range-for/begin-end-from-base-over-derived.cpp` and
`overloading/overload-resolution/assignment-operator-class-parameter.cpp`,
pass on every baseline compiler in the full matrix run of the phase 15
close (`tests/RESULTS.md`), and each row waits only for the front end.
