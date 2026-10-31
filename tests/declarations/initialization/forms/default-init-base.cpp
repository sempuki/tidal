// tidal-rules: INIT-9
// tidal-expect: accept
struct Base {
  int id;
  constexpr Base() : id(7) {}
};
struct Derived : Base {
  int extra = 1;
};
constexpr int read() {
  Derived d;
  return d.id + d.extra;
}
static_assert(read() == 8);
