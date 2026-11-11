template <int D> struct Q { static constexpr int dim = D; };
template <int A, int B> constexpr Q<A + B> operator*(Q<A>, Q<B>) { return {}; }
int main() { auto x = Q<1>() * (Q<2>() * Q<3>()); return x.dim - 6; }
