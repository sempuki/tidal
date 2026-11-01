// tidal-rules: CNS-4 CNS-24
// tidal-expect: reject
// tidal-error-clang: call to .f. is ambiguous
// tidal-error-gcc: call of overloaded .f\(int\). is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: The two mapped arguments have the same value and different tokens, so they are not equivalent (CNS-27). The constraints share no atom, and neither template is more constrained.
template <int N> concept Inner = N > 2;
template <class T> concept First = Inner<sizeof(T) + 1>;
template <class T> concept Second = Inner<1 + sizeof(T)>;
template <class T> requires First<T> constexpr int f(T) { return 1; }
template <class T> requires Second<T> && true constexpr int f(T) { return 2; }
int main() { return f(1) == 2 ? 0 : 1; }   // ambiguous
