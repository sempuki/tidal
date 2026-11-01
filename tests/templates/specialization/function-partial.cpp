// tidal-rules: TSP-7
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: function template partial specialization is not allowed
// tidal-error-gcc: non-class, non-variable partial specialization
// tidal-note: EDG diagnoses the declaration only when a call uses the template (Q-0172).
template <class T, class U> int pick(T, U) { return 1; }
template <class T> int pick<T, int>(T, int) { return 2; }
