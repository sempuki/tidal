// tidal-rules: DTOR-2
// tidal-expect: run
int count = 0;
struct M { ~M() { ++count; } };
struct S { M a; M b; };
int main() { { S s; } return count == 2 ? 0 : 1; }
