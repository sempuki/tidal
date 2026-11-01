// tidal-rules: DEP-20
// tidal-expect: accept
// tidal-note: T::type names a type without typename in an alias template, a named cast, a default template argument, the type of a constant template parameter, and a lambda parameter.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Has { using type = long; };
template <class T> using Alias = T::type;
static_assert(same<Alias<Has>, long>);
template <class T> constexpr auto cast(int v) { return static_cast<T::type>(v); }
static_assert(same<decltype(cast<Has>(1)), long>);
template <class T, class U = T::type> struct Holder { using held = U; };
static_assert(same<Holder<Has>::held, long>);
template <class T, T::type N> constexpr auto value = N;
static_assert(same<decltype(value<Has, 3>), const long>);
template <class T> constexpr int call() { return [](T::type x) { return int(x); }(4); }
static_assert(call<Has>() == 4);
