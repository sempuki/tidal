// tidal-rules: DED-34
// tidal-expect: accept
// tidal-note: auto* adds a type parameter for the pointed-to type, and auto... adds a type parameter pack. The explicit argument goes to the first invented parameter.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
constexpr int deref(const auto* p) { return *p; }
constexpr int count(auto... xs) { return sizeof...(xs); }
auto first(auto* p, auto... rest) { return Tag<void(decltype(p), decltype(rest)...)>{}; }
constexpr int n = 4;
static_assert(deref(&n) == 4);
static_assert(count() == 0 && count(1, 'c', 2.0) == 3);
static_assert(same<decltype(first<int>(nullptr, 'c')), Tag<void(int*, char)>>);
