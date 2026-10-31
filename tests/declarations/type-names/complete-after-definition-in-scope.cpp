// tidal-rules: TYN-6
// tidal-expect: run
struct S;
int size_later();
struct S { int a, b; };
int size_later() { return sizeof(S); }
int main() { return size_later() == 2 * sizeof(int) ? 0 : 1; }
