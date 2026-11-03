// tidal-rules: DEP-27
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: expected identifier
// tidal-error-gcc: keyword .template. not allowed in class-head-name
// tidal-note: The keyword template comes after the qualifier of the member class template that the definition declares. Clang and GCC reject the definition, EDG accepts it (Q-0227).
template <class T> struct Box { template <class U> struct Inner; };
template <class T> template <class U> struct Box<T>::template Inner { U v; };
Box<int>::Inner<int> i;
