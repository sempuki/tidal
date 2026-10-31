// tidal-rules: STMT-6
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
struct Box { int v = 0; Box* operator->() { return this; } };
struct Wrap { Box b; Wrap(int) {} operator Box&() { return b; } };
Box global;
struct Maker { Maker(int) {} Box* operator->() { return &global; } };
int main() { int x = 1; Maker(x)->v = 5; return global.v == 5 ? 0 : 1; }
