// tidal-rules: LIST-10
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
struct A;
struct Holder { int origin; Holder(int o) : origin(o) {} Holder(const A&); };
struct A { Holder h; };
Holder::Holder(const A&) : origin(2) {}
int main() {
  A a{Holder{1}};
  A b{a};
  return b.h.origin == 1 ? 0 : 1;
}
