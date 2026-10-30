// tidal-rules: FN-5
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-note: Q-0022. Clang accepts __func__ outside a function with a warning; GCC and EDG reject it.
const char* p = __func__;
