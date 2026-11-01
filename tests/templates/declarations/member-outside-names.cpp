// tidal-rules: TPL-23 TPL-43
// tidal-expect: run
// tidal-note: The definitions rename the parameters, keep their order, and expand the pack.
template <class T, class U> struct Pair { int first(); };
template <class A, class B> int Pair<A, B>::first() { return sizeof(A); }
template <class... T> struct List { int count(); };
template <class... X> int List<X...>::count() { return sizeof...(X); }
int main() { return Pair<char, int>{}.first() == 1 && List<int, int>{}.count() == 2 ? 0 : 1; }
