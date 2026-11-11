// auto(x) makes a prvalue copy (C++23).
struct C { int v; };
int bump(C&& c) { return ++c.v; }
int main() { C c{1}; int r = bump(auto(c)); return (r == 2 && c.v == 1) ? 0 : 1; }
