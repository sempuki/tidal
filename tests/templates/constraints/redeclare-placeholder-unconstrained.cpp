// tidal-rules: CNS-40 CNS-27
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: Without constraints, int f(auto) declares a template of its own (CNS-27), and the two templates accept the same arguments. GCC and EDG find the call ambiguous. Clang treats the definition as a redeclaration and accepts. CNS-40 needs no diagnostic, so both results conform (Q-0269).
template <class T> int f(T);
int f(auto) { return 1; }
int x = f(1);
