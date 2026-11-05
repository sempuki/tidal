template <class... T> struct List {};
template <class T, class... A> constexpr int f(List<T, A...>) { return 2; }
template <class T, class... A> constexpr int f(List<T, A&...>) { return 3; }
static_assert(f(List<int>{}) == 3);
static_assert(f(List<int, float&>{}) == 3);
