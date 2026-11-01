// tidal-rules: DEP-27
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: expected unqualified-id
// tidal-note: The template keyword comes after the qualifier of the name that the definition declares. Clang rejects the definition, GCC and EDG accept it (Q-0227).
template <class T> struct Box { template <class U> void put(U); };
template <class T> template <class U> void Box<T>::template put(U) {}
