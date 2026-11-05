// tidal-rules: ORD-13
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: is ambiguous
// tidal-note: Q-0293. As in constraints-ellipsis.cpp, with the constraint on the template that has the ellipsis. Ordering by types ties, the ellipsis does not count for templates, and the constraint decides. GCC finds the call ambiguous. Clang and EDG choose the constrained template.
template <class T> constexpr int pick(T, ...) requires (sizeof(T) > 1) { return 1; }
template <class T> constexpr int pick(T) { return 2; }
static_assert(pick(1) == 1);
