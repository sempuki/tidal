template <class T, class U> inline constexpr bool same = false;
template <class T> inline constexpr bool same<T, T> = true;
using A = decltype([] {});
using B = decltype([] {});
static_assert(!same<A, B>);
int main() { A a; a(); return 0; }
