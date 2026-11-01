// tidal-rules: DEP-30 DEP-17
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-error-clang: no member named 'missing' in 'Box<T>'
// tidal-error-gcc: .missing. is not a member of .Box<T>.
// tidal-error-edg: class template "Box<T>" has no member "missing"
// tidal-note: The template is never instantiated, and the name does not depend on T. GCC 14 accepts the template. GCC 16 and the other compilers reject the name at the definition. DEP-30 allows both (Q-0228).
template <class T> struct Box {
  int get() { return Box::missing; }
};
