// The site in W15/largest.cpp. Common needs Arithmetic A and Arithmetic B;
// the caller has the same type constraints. A call that picks the second
// overload shows that the caller's constraints subsume those of Common.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T, class... Us> concept OneOf = (same<T, Us> || ...);
template <class T> concept Arithmetic = OneOf<T, char, short, int, long, float, double>;
template <class T> concept Extra = sizeof(T) > 0;
template <Arithmetic A, Arithmetic B> constexpr int pick(A, B) { return 1; }            // Common
template <Arithmetic A, Arithmetic B> requires Extra<A> constexpr int pick(A, B) { return 2; } // caller, plus one atom
static_assert(pick(1, 2.0) == 2);
int main() {}
