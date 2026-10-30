// tidal-rules: JMP-11
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int copies = 0;
struct C {
  C() {}
  C(const C&) { ++copies; }
  C(C&&) { ++copies; }
};
C make() { C local; return local; }
int main() { C c = make(); return copies <= 1 ? 0 : 1; }
