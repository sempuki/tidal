// tidal-rules: CTAD-1 CTAD-6
// tidal-expect: accept
// tidal-note: new Box with no initializer deduces with no arguments. The aggregate Box declares no constructor, so its guide with no parameters gives the default of T.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T = int> struct Box { T v; };
auto p = new Box;
static_assert(same<decltype(p), Box<int>*>);
