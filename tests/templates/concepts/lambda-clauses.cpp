// tidal-rules: CPT-16
// tidal-expect: accept
template <class T> concept Big = sizeof(T) > 1;
template <class F, class A> concept Callable = requires (F f, A a) { f(a); };
auto leading = []<class T> requires Big<T> (T) { return 1; };
auto trailing = []<class T>(T) requires (!Big<T>) { return 2; };
auto marked = []<class T> requires Big<T> [[nodiscard]] (T) { return 3; };
static_assert(Callable<decltype(leading), int> && !Callable<decltype(leading), char>);
static_assert(Callable<decltype(trailing), char> && !Callable<decltype(trailing), int>);
static_assert(Callable<decltype(marked), int> && !Callable<decltype(marked), char>);
