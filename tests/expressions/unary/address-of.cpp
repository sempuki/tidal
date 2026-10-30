// tidal-rules: UN-8
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int x;
const int c = 1;
int arr[3];
void f();
static_assert(same<decltype(&x), int*>);
static_assert(same<decltype(&c), const int*>);
static_assert(same<decltype(&arr), int (*)[3]>);
static_assert(same<decltype(&f), void (*)()>);
