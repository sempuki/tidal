template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class Tag = decltype([] {})> struct Unique {};
static_assert(!same<Unique<>, Unique<>>);
using A = decltype([] {});
static_assert(same<A, A>);
int main() {}
