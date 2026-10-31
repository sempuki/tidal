// tidal-rules: UNEV-2
// tidal-expect: run
int copies = 0;
struct C { C() {} C(const C&) { ++copies; } };
C obj;
C& get() { return obj; }
int main() { get(); return copies == 0 ? 0 : 1; }
