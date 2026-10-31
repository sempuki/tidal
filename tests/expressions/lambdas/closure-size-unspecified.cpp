// tidal-rules: LAM-20
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int main() { int a = 1; auto byref = [&a] { return a; }; auto bycopy = [a] { return a; }; return sizeof(byref) >= 1 && sizeof(bycopy) >= sizeof(int) ? 0 : 1; }
