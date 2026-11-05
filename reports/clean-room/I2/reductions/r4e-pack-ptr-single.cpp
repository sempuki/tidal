template <class... T> struct List {};
template <class... A> constexpr int f(List<A*...>) { return 3; }
static_assert(f(List<float*>{}) == 3);
