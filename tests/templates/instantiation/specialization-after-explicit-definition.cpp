// tidal-rules: INST-19
// tidal-expect: reject
// tidal-error-clang: explicit specialization of 'get<int>' after instantiation
// tidal-error-gcc: specialization of .int get\(T\) \[with T = int\]. after instantiation
// tidal-error-edg: it has been explicitly specialized
template <class T> int get(T) { return 1; }
template int get(int);
template <> int get(int) { return 2; }
