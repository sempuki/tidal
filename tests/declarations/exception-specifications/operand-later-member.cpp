// tidal-rules: EXS-1
// tidal-expect: accept
struct S {
  void f() noexcept(noexcept(g()));
  void g() noexcept;
};
static_assert(noexcept(S{}.f()));
