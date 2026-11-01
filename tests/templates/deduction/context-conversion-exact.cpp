// tidal-rules: DED-31
// tidal-expect: accept
// tidal-note: The needed types const int* and const int& give T = const int by an exact match. The differences of DED-31 are not needed.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int n = 0;
struct Point { template <class T> operator T*() const { static_assert(same<T, const int>); return nullptr; } };
struct Refer { template <class T> operator T&() const { static_assert(same<T, const int>); return n; } };
const int* p = Point{};
const int& r = Refer{};
