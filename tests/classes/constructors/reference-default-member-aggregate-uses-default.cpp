// tidal-rules: CTOR-17
// tidal-expect: run
// tidal-note: A is an aggregate. The braces give tag a value and leave r to its default member initializer, which binds r to a temporary. No constructor runs, so CTOR-17 does not reject the program, and the temporary lives as long as a.
struct A { int tag; const int& r = 5; };
int main() { A a{1}; return a.tag == 1 && a.r == 5 ? 0 : 1; }
