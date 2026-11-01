// tidal-rules: INST-20
// tidal-expect: reject
// tidal-error-clang: explicit instantiation of undefined function template 'get'
// tidal-error-gcc: but no definition available
// tidal-error-edg: no template definition was supplied
template <class T> int get(T);
template int get(int);
