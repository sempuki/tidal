// tidal-rules: DED-24
// tidal-expect: accept
// tidal-note: I5-11, I5 program p05-array-of-functions.cpp. An array of functions and an array of void are not valid types, so substitution fails and the template with the ellipsis wins. The control with int keeps the first template.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct One {};
struct Two {};
template <class T> One pair(T (*)[2]);
template <class T> Two pair(...);
static_assert(same<decltype(pair<int(char)>(nullptr)), Two>);
static_assert(same<decltype(pair<void>(nullptr)), Two>);
static_assert(same<decltype(pair<int>(nullptr)), One>);
