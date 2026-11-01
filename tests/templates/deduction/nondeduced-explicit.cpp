// tidal-rules: DED-13
// tidal-expect: accept
// tidal-note: Explicit arguments give the parameters that no position deduces.
template <class T> struct Id { using type = T; };
template <class T> void only(typename Id<T>::type) {}
template <int N> void fill(int values[N]) {}
int arr[3];
void use() { only<long>(1); fill<3>(arr); }
