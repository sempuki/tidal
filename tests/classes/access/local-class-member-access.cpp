// tidal-rules: ACC-10
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
class C { int secret = 6; public: int peek() const { struct L { int get(const C& c) { return c.secret; } }; return L{}.get(*this); } };
int main() { return C{}.peek() == 6 ? 0 : 1; }
