// tidal-rules: UNI-3 CPY-7
// tidal-expect: run
struct Part {
  Part() = default;
  Part(const Part&) = default;
  Part(Part&&) {}
  Part& operator=(const Part&) = default;
  Part& operator=(Part&&) { return *this; }
};
union Slot { Part part; int count; };
int main() {
  Slot a;
  a.count = 4;
  Slot b(static_cast<Slot&&>(a));
  Slot c;
  c = static_cast<Slot&&>(b);
  return c.count == 4 ? 0 : 1;
}
