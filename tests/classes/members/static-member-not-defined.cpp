// tidal-rules: CLS-3
// tidal-expect: link-fail
// tidal-sources: static-member-not-defined.tu.cpp
struct Counter { static int count; };
int read();
int main() { return Counter::count + read(); }
