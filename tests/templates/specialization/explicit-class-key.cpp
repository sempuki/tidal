// tidal-rules: TSP-32
// tidal-expect: accept
// tidal-note: struct and class agree.
template <class T> struct Box {};
template <> class Box<int> {};
template <class T> class Box<T*> {};
