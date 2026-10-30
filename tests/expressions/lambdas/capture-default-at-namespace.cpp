// tidal-rules: LAM-6
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: Q-0037. EDG accepts a capture default on a lambda at namespace scope; Clang and GCC reject it.
auto g = [=] { return 1; };
