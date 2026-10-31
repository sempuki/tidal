// tidal-rules: LSUP-4
// tidal-expect: accept
namespace std {
using size_t = decltype(sizeof 0);
using nullptr_t = decltype(nullptr);
using ptrdiff_t = decltype((int*)nullptr - (int*)nullptr);
}
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<std::size_t, decltype(sizeof(int))>);
static_assert(sizeof(std::nullptr_t) == sizeof(void*));
int a[4];
static_assert(same<std::ptrdiff_t, decltype(&a[3] - &a[0])>);
