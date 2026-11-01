// tidal-rules: CNS-27
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-edg: reject
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: The invented parameter of A auto gives the head of the first declaration, so the definition redeclares it. GCC and EDG see two templates (Q-0269).
template <class T> concept A = sizeof(T) > 1;
template <A T> int f(T);
int f(A auto) { return 1; }
int x = f(1);
