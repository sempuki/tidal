// tidal-rules: DEP-27
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: expected unqualified-id
// tidal-note: The keyword template comes after the qualifier of the static member function template that the definition declares. Clang rejects the definition, GCC and EDG accept it (Q-0227).
template <class T> struct Box { template <class U> static int make(U); };
template <class T> template <class U> int Box<T>::template make(U) { return 1; }
