// tidal-rules: LAM-6
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: non-local lambda expression cannot have a capture-default
// tidal-error-gcc: non-local lambda expression cannot have a capture-default
// tidal-note: Q-0037. The lambda is in a default argument of a member function of a local class. No block encloses the lambda, so the capture default is rejected. Clang and GCC reject it. EDG accepts it, as it accepts a capture default at namespace scope.
void outer() {
  struct Local {
    int get(int x = [=] { return 1; }()) { return x; }
  };
}
