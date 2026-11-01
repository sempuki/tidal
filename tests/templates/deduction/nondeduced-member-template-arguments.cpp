// tidal-rules: DED-13 DED-35
// tidal-expect: reject
// tidal-error-clang: couldn't infer template argument
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: U appears only in the argument list of a member template named through Outer<T>::. The whole qualified name is a non-deduced position, so U gets no value, although T gets one from the first argument.
template <class T> struct Outer { template <class U> struct In {}; };
template <class T, class U> void take(T, typename Outer<T>::template In<U>);
void use() { take(1, Outer<int>::In<char>{}); }
