// tidal-rules: DEP-28
// tidal-expect: accept
template <class T> int measure(T* p) { return p->size() + T::count; }
