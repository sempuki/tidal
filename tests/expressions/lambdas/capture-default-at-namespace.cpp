// tidal-rules: LAM-6
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: non-local lambda expression cannot have a capture-default
// tidal-error-gcc: non-local lambda expression cannot have a capture-default
// tidal-note: Q-0037. EDG accepts a capture default on a lambda at namespace scope; Clang and GCC reject it.
auto g = [=] { return 1; };
