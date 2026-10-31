// tidal-rules: LAM-6
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct S { int v = 0; void f() { auto a = [&, this] { return v; }; auto b = [=, this] { return v; }; auto c = [=, *this] { return v; }; a(); b(); c(); } };
