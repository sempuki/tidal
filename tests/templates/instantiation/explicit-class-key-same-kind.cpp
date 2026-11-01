// tidal-rules: INST-23
// tidal-expect: accept
template <class T> class Box {};
template struct Box<int>;
extern template class Box<char>;
