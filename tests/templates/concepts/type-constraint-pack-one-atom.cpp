// tidal-rules: CPT-19 CNS-3 CNS-24
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: call to 'count' is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-note: Each pack's type constraint is one fold expression, so neither template is more constrained although AB subsumes A. EDG chooses the second template (Q-0285).
template <class T> concept A = sizeof(T) > 0;
template <class T> concept AB = A<T> && sizeof(T) < 100;
template <A... T> constexpr int count(T...) { return 1; }
template <AB... T> constexpr int count(T...) { return 2; }
int chosen = count(1, 2);
