// tidal-rules: AUTO-3
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
const int ci = 1;
int arr[3];
void fn();
auto& a = ci;
auto& b = arr;
auto& c = fn;
auto& d = "hi";
static_assert(same<decltype(a), const int&>);
static_assert(same<decltype(b), int (&)[3]>);
static_assert(same<decltype(c), void (&)()>);
static_assert(same<decltype(d), const char (&)[3]>);
