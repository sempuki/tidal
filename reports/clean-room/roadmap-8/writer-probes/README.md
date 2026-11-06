# Writer probes, roadmap round 8

Commit read: 1e20bfa2. The two grids ran with `tools/probe_grid.py --errors`
on the baseline compilers (Clang 18, Clang 22 in its container and on the
host, GCC 14, GCC 16 in its container and on the host, EDG 7.0, and EDG 7.0
ILP32). The placement probe ran on the host `g++` and `clang++` only,
because it needs a link.

| File | Finding | Result |
|---|---|---|
| `g152-candidate-body.json`, `.out` | G152: is the body of a generic call operator with a deduced result type instantiated when overload resolution only forms the candidate? | Every compiler accepts all 15 cells. A plain `operator()(const Circle&)` wins against a generic one whose body reads `t.side`, with a deduced result, a trailing `decltype` result, or a declared `int` result, in a run and in a `static_assert`. A member template of a plain class behaves the same. No compiler instantiates the losing candidate's body. |
| `pu3-4-align-val-t-shape.json`, `.out` | PU3-4 and Q-0299: the shape of `std::align_val_t` | A scoped enumeration with underlying type `size_t` is accepted everywhere. A scoped enumeration with another underlying type, an unscoped enumeration, or a class is rejected by GCC 14, GCC 16, and EDG at the declaration, with no use. Clang accepts each shape, also with a `new` of an over-aligned type. With no declaration, GCC accepts a cast to `std::align_val_t`; Clang and EDG reject it as an unknown name. |
| `placement-new-declared-only.cpp` | The programmer's round 7 doubt left by item T (a placement allocation function declared and not defined) | With no use, both host compilers build and run it (exit 0). With a use in a `new` expression, both compile it and the link fails with an undefined reference. |
