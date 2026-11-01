// tidal-rules: TPL-23
// tidal-expect: accept
// tidal-note: A member class of a class template, declared in the class and defined after the class body.
template <class T> struct Outer { struct Inner; };
template <class T> struct Outer<T>::Inner { T value; };
static_assert(sizeof(Outer<long>::Inner) == sizeof(long));
