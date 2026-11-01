// tidal-rules: CNS-13 CNS-14
// tidal-expect: reject
// tidal-error-clang: constraints not satisfied for class template 'Box'
// tidal-error-gcc: template constraint failure
// tidal-error-edg: template constraint not satisfied
// tidal-note: After substitution, Tmpl<char> is Box<char>, which fails the constraint of Box.
template <class T> concept Wide = sizeof(T) > 1;
template <template <class> class Tmpl> struct Use { Tmpl<char> member; };
template <Wide T> struct Box {};
Use<Box> use;
