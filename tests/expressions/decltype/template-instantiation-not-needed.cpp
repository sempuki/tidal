// tidal-rules: UNEV-1
// tidal-expect: accept
template <class T> struct W { static_assert(sizeof(T) == 0); };
template <class T> W<T> wrap(T);
using R = decltype(wrap(1));
