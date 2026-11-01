// tidal-rules: CNS-34
// tidal-expect: accept
// tidal-note: The second template names its parameters U and T in the other order. They correspond by position, so the function parameter types agree and the constraints are compared. The second template is more constrained.
template <class T> concept A = sizeof(T) > 0;
template <class T, class U> requires A<T> constexpr int f(T, U) { return 1; }
template <class U, class T> requires A<U> && (sizeof(T) > 0) constexpr int f(U, T) { return 2; }
static_assert(f(1, 2) == 2);
