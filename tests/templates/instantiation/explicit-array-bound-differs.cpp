// tidal-rules: INST-22
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: does not match expected type 'int\[3\]'
// tidal-note: The variable template's type has no bound, and its initializer gives the specialization the type int[3]. The explicit instantiation writes the bound 4. Clang rejects it, GCC and EDG accept it and keep the type int[3] (Q-0346).
template <class T> int table[] = {1, 2, 3};
template int table<int>[4];
