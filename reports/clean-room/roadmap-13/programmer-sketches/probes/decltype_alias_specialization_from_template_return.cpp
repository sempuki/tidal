template <int D> struct Q { static constexpr int dim = D; };
template <int A, int B> constexpr Q<A - B> div(Q<A>, Q<B>) { return {}; }
using T = decltype(div(Q<1>(), Q<2>()));
static_assert(T::dim == -1);
int main() { return 0; }
