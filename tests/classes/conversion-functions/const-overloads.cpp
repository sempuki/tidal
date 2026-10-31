// tidal-rules: CVF-4
// tidal-expect: run
struct S { operator int() & { return 1; } operator int() && { return 2; } };
int main() { S s; int a = s; int b = S{}; return a == 1 && b == 2 ? 0 : 1; }
