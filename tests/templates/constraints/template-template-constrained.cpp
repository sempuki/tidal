// tidal-rules: CNS-25
// tidal-expect: accept
template <class T> concept A = sizeof(T) > 1;
template <class T> concept AB = A<T> && sizeof(T) < 100;
template <template <class> class Tmpl> struct Plain {};
template <template <AB> class Tmpl> struct Strict {};
template <A T> struct Box {};
Plain<Box> plain;
Strict<Box> strict;
