// tidal-rules: ORD-13
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: is ambiguous
// tidal-note: Q-0293. The two templates have the same head and the same parameter T, and differ only in the ellipsis. Ordering by types ties, and ORD-13 does not count the ellipsis for templates, so the constraint decides. GCC finds the call ambiguous. Clang and EDG choose the constrained template.
template <class T> constexpr int pick(T) requires (sizeof(T) > 1) { return 1; }
template <class T> constexpr int pick(T, ...) { return 2; }
static_assert(pick(1) == 1);
