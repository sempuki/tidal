template <class... T> struct List {};
template <class... A> constexpr int f(List<A...>) { return 1; }
template <class T, class... A> constexpr int f(List<T, A...>) { return 2; }
static_assert(f(List<int, float>{}) == 2);
