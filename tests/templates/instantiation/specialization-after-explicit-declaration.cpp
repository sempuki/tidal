// tidal-rules: INST-19
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: An explicit specialization follows an explicit instantiation declaration. Clang and GCC reject it, EDG accepts it (Q-0237).
// tidal-error-clang: explicit specialization of 'get<int>' after instantiation
// tidal-error-gcc: specialization of .int get\(T\) \[with T = int\]. after instantiation
template <class T> int get(T) { return 1; }
extern template int get(int);
template <> int get(int) { return 2; }
