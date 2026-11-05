// tidal-rules: ORD-14 ORD-15
// tidal-expect: reject
// tidal-error-clang: ambiguous partial specializations
// tidal-error-gcc: ambiguous template instantiation
// tidal-error-edg: more than one partial specialization
// tidal-note: Both partial specializations match Pair<int, int>, and neither pattern is more specialized. The patterns Pair<T, int> and Pair<int, T> differ, so the constraint on the first does not decide.
template <class T> concept Small = sizeof(T) < 8;
template <class T, class U> struct Pair { static constexpr int value = 0; };
template <class T> requires Small<T> struct Pair<T, int> { static constexpr int value = 1; };
template <class T> struct Pair<int, T> { static constexpr int value = 2; };
int n = Pair<int, int>::value;
