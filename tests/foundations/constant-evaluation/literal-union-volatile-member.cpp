// tidal-rules: CE-23
// tidal-expect: accept
union Register {
  volatile int hardware;
  int shadow;
  constexpr Register() : shadow(0) {}
};
constexpr Register reg;
static_assert(reg.shadow == 0);
