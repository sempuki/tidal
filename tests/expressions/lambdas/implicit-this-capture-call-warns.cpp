// tidal-rules: LAM-21
// tidal-expect: reject
// tidal-error-clang: implicit capture of .this. with a capture default of .=. is deprecated
// tidal-error-gcc: implicit capture of .this. via .\[=\]. is deprecated
// tidal-flags: -Wall -Wextra -Werror
// tidal-note: The body calls a member function, so [=] captures this implicitly.
struct S {
  int get() { return 1; }
  int f() { return [=] { return get(); }(); }
};
