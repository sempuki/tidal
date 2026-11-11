template <int D> struct Q { static constexpr int dim = D; };
template <int A, int B> constexpr Q<A - B> div(Q<A>, Q<B>) { return {}; }
static_assert(decltype(div(Q<1>(), Q<2>()))::dim == -1);
int main() { return 0; }
