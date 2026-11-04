// tidal-rules: DEFARG-10 DEP-30
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: cannot initialize a parameter of type .int \*. with an rvalue of type .double
// tidal-error-edg: default argument of type .double. is incompatible
// tidal-note: DEP-30 allows both results. The default does not depend on T, and no call uses it. GCC checks it only when a call uses it.
template <class T> void f(int* p = 1.5) {}
