// tidal-rules: OVL-42
// tidal-expect: run
// tidal-note: The member's move functions are deleted, so the defaulted move constructor and move assignment of Holder are deleted. They are not candidates, and the copy functions take the rvalues.
int copies = 0;
struct Part {
  Part() {}
  Part(const Part&) { ++copies; }
  Part(Part&&) = delete;
  Part& operator=(const Part&) { ++copies; return *this; }
  Part& operator=(Part&&) = delete;
};
struct Holder {
  Part part;
  Holder() {}
  Holder(const Holder&) = default;
  Holder(Holder&&) = default;
  Holder& operator=(const Holder&) = default;
  Holder& operator=(Holder&&) = default;
};
int main() {
  Holder a;
  Holder b(static_cast<Holder&&>(a));
  Holder c;
  c = static_cast<Holder&&>(a);
  return copies == 2 ? 0 : 1;
}
