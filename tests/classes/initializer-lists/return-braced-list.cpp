// tidal-rules: ILC-6
// tidal-expect: run
int which = 0;
struct S { S(int, int) { which = 1; } };
S make() { return {1, 2}; }
int main() { make(); return which == 1 ? 0 : 1; }
