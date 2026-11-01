// tidal-rules: CNS-25
// tidal-expect: reject
// tidal-expect-clang-18: accept
// tidal-expect-edg: accept
// tidal-error-clang: more constrained than template template parameter
// tidal-error-gcc: constraint mismatch
// tidal-note: The parameter accepts every type that A accepts, and Box accepts fewer. Clang 18 and EDG accept the argument (Q-0265).
template <class T> concept A = sizeof(T) > 1;
template <class T> concept AB = A<T> && sizeof(T) < 100;
template <template <A> class Tmpl> struct Use {};
template <AB T> struct Box {};
Use<Box> use;
