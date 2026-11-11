struct V { int t; };
template <V S> struct Key { static constexpr auto name = S; };
template <class K> constexpr int f(K) { return K::name.t; }
static_assert(f(Key<V{3}>{}) == 3);
int main() {}
