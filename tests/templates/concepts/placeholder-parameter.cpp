// tidal-rules: CPT-20
// tidal-expect: accept
template <class T> concept Big = sizeof(T) > 1;
constexpr int size(Big auto x) { return sizeof(x); }
auto lam = [](Big auto x) { return 1; };
template <class T> concept Sizes = requires (T t) { size(t); };
template <class F, class A> concept Callable = requires (F f, A a) { f(a); };
static_assert(Sizes<int> && !Sizes<char>);
static_assert(Callable<decltype(lam), int> && !Callable<decltype(lam), char>);
static_assert(size<double>(1) == sizeof(double));
