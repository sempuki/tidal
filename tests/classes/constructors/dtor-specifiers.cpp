// tidal-rules: DTOR-1
// tidal-expect: accept
struct S {
  virtual ~S() {}
};
struct T {
  constexpr ~T() {}
};
struct U {
  ~U() noexcept(false) {}
};
