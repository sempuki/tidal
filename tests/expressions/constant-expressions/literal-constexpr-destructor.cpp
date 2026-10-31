// tidal-rules: CE-23
// tidal-expect: accept
struct Counted {
  int value;
  constexpr ~Counted() {}
};
constexpr Counted counted{1};
static_assert(counted.value == 1);
