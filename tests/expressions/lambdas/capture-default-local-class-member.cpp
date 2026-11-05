// tidal-rules: LAM-6
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: non-local lambda expression cannot have a capture-default
// tidal-error-gcc: non-local lambda expression cannot have a capture-default
// tidal-note: The lambda is in a member declaration of a local class, outside any function body. A function encloses the class, but no block encloses the lambda, so the capture default is rejected. Clang and GCC reject it. EDG accepts it, as it accepts a capture default at namespace scope (Q-0037).
void outer() {
  struct Local {
    decltype([=] { return 1; }) member;
  };
}
