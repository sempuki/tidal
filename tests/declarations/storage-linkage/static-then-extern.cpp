// tidal-rules: LNK-8
// tidal-expect: run
// tidal-sources: static-then-extern.tu.cpp
static int x = 1;
extern int x;
static int f();
int f() { return 10; }
int other_x();
int other_f();
int main() { return x == 1 && f() == 10 && other_x() == 2 && other_f() == 20 ? 0 : 1; }
