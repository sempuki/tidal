// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
// tidal-expect-gcc: accept
// tidal-error-clang: carries_dependency.*after its first declaration
// tidal-error-edg: previously declared without the carries_dependency attribute
// tidal-note: Q-0108. GCC accepts carries_dependency added on a redeclaration (it ignores the attribute). Clang and EDG reject it.
int* f(int* p);
[[carries_dependency]] int* f(int* p);
