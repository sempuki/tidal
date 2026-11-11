# Writer probes, roadmap round 12

Commit read: `bd87b27c`. The grids ran with `tools/probe_grid.py --errors`
on the baseline compilers (Clang 18, Clang 22 in its container and on
the host, GCC 14, GCC 16 in its container and on the host, EDG 7.0, and
EDG 7.0 ILP32), each run under `flock build/run_tests.lock`.
`writer-scratch/make_grids.py` writes the JSON files. Each `.out` file
holds the table and the first error of each compiler that rejects.

The grids compile only. A probe that needs a type or a value puts it in
a `static_assert`, so EDG takes part. The two G165 grids pass
`-Werror` with a warning flag, so EDG skips them, as it skips the
warning-flag tests of the matrix.

`writer-scratch/run_probes.py` builds and runs programs on the six
compilers that build (Clang 18, Clang 22 in its container and on the
host, GCC 14, GCC 16 in its container and on the host). Its output is
`run-probes.out`.

| File | Finding | Result |
|---|---|---|
| `i7-ctad.json`, `.out` | I7-1, I7-3, I7-6 to I7-9, I7-11, I7-13, I7-14, and the I7-16 host-only claims. One program per row. | Every compiler: a member of a class type or an `int` array that does not depend on the parameters has its braces elided (I7-1); a class template that inherits constructors is not an aggregate (I7-3); a user guide whose first parameter is `std::initializer_list<int>` and whose second has a default wins a braced list of two (I7-6); the constraint of an inner alias applies through an outer alias (I7-8); a derived class gets neither the base's guide with no parameters nor the base's aggregate candidate (I7-9); `new Box` with no initializer deduces from the default (I7-13); copy-list initialization that chooses a guide that is explicit for its deduced argument is rejected (I7-14); `Entry a{.key = 3}` and `Entry b(4L)` give `Entry<int>` and `Entry<long>` (I7-16). Splits: for `C c{}` with a guide with no parameters and an initializer-list guide, Clang 18 and 22 choose the guide with no parameters, and GCC 14 and 16 and EDG choose the initializer-list guide (I7-6); in copy initialization, a guide whose only route is a user-defined conversion to its parameter is viable on Clang 22, GCC, and EDG, and Clang 18 rejects (I7-11); a static data member of the deduced class type with no initializer, `inline` or not, is accepted by GCC and rejected by Clang and EDG (I7-13); in copy initialization from `'c'`, Clang and EDG leave out the guide that is explicit for `char` and take the user guide `N(int) -> N<int>`, and GCC 14 and 16 deduce `N<char>` and then reject the initialization (I7-14). Known splits: EDG rejects `Named b{"ab", 1}` (Q-0277); EDG rejects a conversion to the fixed parameter of an alias, and Clang 18 has no deduction through an alias (Q-0240); Clang has no inherited guides (Q-0239). |
| `p3-points.json`, `.out` | P3-6 to P3-8: members of a base class template over a derived class that is incomplete when the base is made. The second column calls `begin()` after the derived class is complete. | Every compiler accepts, with and without the call: an in-class body that returns `Cursor<D>`, a declaration in the class with the body outside, a `virtual` in-class body, an in-class body that takes `Cursor<D>` by value, a deduced return type, and a `Cursor<D>` constructor that calls a member of `D`. Every compiler rejects a return type written `typename D::value_type` and a non-static data member of type `Cursor<D>`. Splits: a `static` data member of type `Cursor<D>` that is declared only is rejected by GCC 14 and 16 and accepted by Clang and EDG; a `static inline` data member of that type is accepted by Clang 18 and 22 and rejected by GCC and EDG. |
| `p3-copy-init.json`, `.out` | P3-11 and P3-3 | Every compiler accepts `Action<int> a = closure;` where `Action` has deleted copy operations and a converting constructor template, and `Action<int> a = Action<int>(closure);`. Every compiler rejects the same form with an `explicit` converting constructor, and a copy of an lvalue `Action<int>`. Every compiler accepts a function template that is declared only and named in `decltype` (P3-3). |
| `g165-wall.json`, `.out` | G165 with `-Wall -Werror` | GCC and Clang: no warning when a derived function overrides one of two base overloads and hides the other, with or without `using B::f`. Both warn when no derived function of the name overrides, with a new `f(char)` or a single hidden `f(double)`. |
| `g165-woverloaded.json`, `.out` | G165 with `-Woverloaded-virtual -Werror` | As with `-Wall`, except that GCC 14 and 16 also warn when one overload is overridden and the other is hidden, and Clang does not. |
| `run-probes.out` | P3-5, the P3 host-only claim on warnings, and the I7 host-only claims on programs | On each of the six compilers, `reports/clean-room/P3/probes/dangling-stage.cpp` under `-fsanitize=address` exits 1 with `stack-use-after-scope`, and the five P3 program tests build with `-Wall -Wextra -Werror` and exit 0. The 12 I7 programs and reduction r2 build and exit 0 on each compiler, with these exceptions: Clang 18 and 22 reject p03 (Q-0239), and Clang 18 rejects p02 (Q-0240). Reduction r1 has no `main`, so its link fails on every compiler; its compile-only form is the `I7-16` rows of `i7-ctad.out`. |

The P3 claims on the four reductions (P3-7 to P3-10) and on a declared
function used only in `decltype` (P3-3, through `pipeline.cpp`) already
hold on the matrix. The tests and program tests of commit 8b10f65f pass
on every baseline compiler.
