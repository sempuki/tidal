// tidal-rules: CPT-16
// tidal-expect: accept
// tidal-note: One lambda has a requires clause after its template parameter list and a trailing requires clause. A call needs both clauses satisfied.
template <class T> concept Big = sizeof(T) > 1;
template <class T> concept Small = sizeof(T) < 8;
template <class F, class A> concept Callable = requires (F f, A a) { f(a); };
auto both = []<class T> requires Big<T> (T) requires Small<T> { return 1; };
struct Huge { char bytes[16]; };
static_assert(Callable<decltype(both), int>);
static_assert(!Callable<decltype(both), char>);
static_assert(!Callable<decltype(both), Huge>);
