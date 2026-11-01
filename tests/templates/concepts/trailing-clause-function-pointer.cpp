// tidal-rules: CPT-13
// tidal-expect: reject
// tidal-error-clang: can only be used when declaring a function
// tidal-error-gcc: requires-clause on declaration of non-function type
// tidal-error-edg: a requires-clause is not allowed here
template <class T> struct Box { void (*call)() requires true; };
