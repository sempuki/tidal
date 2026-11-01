// tidal-rules: INST-10
// tidal-expect: accept
template <class T> struct Pair { T first; int second = T::nothing; };
Pair<int> p{1, 2};
