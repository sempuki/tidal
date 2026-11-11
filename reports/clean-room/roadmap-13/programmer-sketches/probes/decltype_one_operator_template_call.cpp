template <int D> struct Q {};
template <int A, int B> constexpr Q<A + B> operator*(Q<A>, Q<B>) { return {}; }
template <class A, class B> inline constexpr bool same = false;
template <class A> inline constexpr bool same<A, A> = true;
static_assert(same<decltype(Q<2>() * Q<3>()), Q<5>>);
int main() { return 0; }
