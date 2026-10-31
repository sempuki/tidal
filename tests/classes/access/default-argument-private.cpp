// tidal-rules: ACC-16
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
class C { static int hidden() { return 5; } public: static int get(int v = hidden()) { return v; } };
int main() { return C::get() == 5 ? 0 : 1; }
