// tidal-rules: INST-22
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: does not match expected type 'int\[3\]'
// tidal-note: The explicit instantiation leaves out the array bound of the variable template's type. GCC and EDG accept it, Clang rejects it (Q-0236).
template <class T> int table[] = {1, 2, 3};
template int table<int>[];
