// tidal-rules: ATTR-11
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
// tidal-expect-gcc: accept
// tidal-note: Q-PENDING: GCC accepts carries_dependency added on a redeclaration (it ignores the attribute). Clang and EDG reject it.
int* f(int* p);
[[carries_dependency]] int* f(int* p);
