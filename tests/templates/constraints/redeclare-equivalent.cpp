// tidal-rules: CNS-27
// tidal-expect: accept
template <class T> concept A = sizeof(T) > 1;
template <A T> int f(T);
template <A U> int f(U) { return 1; }
template <class T> requires A<T> int g(T);
template <class U> requires A<U> int g(U) { return 1; }
template <class T> int h(T) requires A<T>;
template <class U> int h(U) requires A<U> { return 1; }
int x = f(1) + g(1) + h(1);
