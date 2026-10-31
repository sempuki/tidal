// tidal-rules: CMP-9
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: Q-PENDING: EDG accepts a relational comparison of two nullptr_t values. Clang and GCC reject it.
// tidal-note: Gap review 2026-10-31 (third).
bool f(decltype(nullptr) a, decltype(nullptr) b) { return a < b; }
