// tidal-rules: LAM-8
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct T { int v = 5; int got = [this] { return v; }(); int all = [=, this] { return v * 2; }(); };
int main() { T t; return t.got == 5 && t.all == 10 ? 0 : 1; }
