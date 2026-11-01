// tidal-rules: DED-37
// tidal-expect: accept
// tidal-note: The pack A... is the parameter list of a function pointer type. It takes each parameter type of the argument's function type, and none for a function with no parameters.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... T> struct Tag {};
template <class... A> Tag<A...> params(void (*)(A...));
void two(int, char);
void none();
static_assert(same<decltype(params(two)), Tag<int, char>>);
static_assert(same<decltype(params(none)), Tag<>>);
