// tidal-rules: CPY-18
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int assigns = 0;
struct V { V& operator=(const V&) { ++assigns; return *this; } };
struct A : virtual V {};
struct B : virtual V {};
struct D : A, B {};
int main() { D x, y; x = y; return (assigns == 1 || assigns == 2) ? 0 : 1; }
