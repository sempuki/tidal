// tidal-rules: CPT-2
// tidal-expect: accept
template <int N> concept Positive = N > 0;
template <template <class> class Tmpl> concept Wraps = sizeof(Tmpl<int>) > 0;
template <class... Ts> concept Few = sizeof...(Ts) < 3;
template <class T> concept OddSize = sizeof(T) % 2 == 1 || !true;
template <class T> struct Box { T value; };
static_assert(Positive<3> && !Positive<0>);
static_assert(Wraps<Box>);
static_assert(Few<int, char> && !Few<int, int, int>);
static_assert(OddSize<char> && !OddSize<short>);
