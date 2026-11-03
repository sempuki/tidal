// tidal-rules: LNK-8
// tidal-expect: run
// tidal-sources: static-then-extern-function.tu.cpp
// tidal-note: A function declared static and then declared and defined with extern. The other unit defines a function with the same name and external linkage, and the program links.
static int g();
extern int g() { return 10; }
int other_g();
int main() { return g() == 10 && other_g() == 20 ? 0 : 1; }
