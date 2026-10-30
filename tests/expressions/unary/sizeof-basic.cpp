// tidal-rules: UN-12
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(sizeof(char) == 1);
static_assert(sizeof(signed char) == 1 && sizeof(unsigned char) == 1);
int arr[10];
int* p = arr;
static_assert(sizeof arr == 10 * sizeof(int));
static_assert(sizeof p == sizeof(int*));
static_assert(same<decltype(sizeof(int)), decltype(sizeof 0)>);
static_assert(sizeof(int&) == sizeof(int));
struct S { char c[3]; };
static_assert(sizeof(S) >= 3);
