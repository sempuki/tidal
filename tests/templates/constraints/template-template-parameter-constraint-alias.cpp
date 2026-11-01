// tidal-rules: CNS-38
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: template constraint not satisfied
// tidal-note: Tmpl<char> in the member alias fails the constraint of the parameter Tmpl. CNS-38 needs no diagnostic. EDG diagnoses the definition, Clang and GCC do not (Q-0283).
template <class T> concept Wide = sizeof(T) > 1;
template <template <Wide> class Tmpl> struct Use { using type = Tmpl<char>; };
