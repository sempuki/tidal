// tidal-rules: INST-26
// tidal-expect: reject
// tidal-error-clang: duplicate explicit instantiation of 'get<int>'
// tidal-error-gcc: duplicate explicit instantiation
// tidal-error-edg: explicitly instantiated more than once
template <class T> int get(T) { return 1; }
template int get(int);
template int get(int);
