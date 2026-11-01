// tidal-rules: INST-20
// tidal-expect: reject
// tidal-error-clang: explicit instantiation of undefined variable template
// tidal-error-gcc: but no definition available
// tidal-error-edg: no template definition was supplied
template <class T> extern int zero;
template int zero<int>;
