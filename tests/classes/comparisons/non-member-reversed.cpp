// tidal-rules: DCMP-6
// tidal-expect: run
struct A { int v; };
struct B { int w; };
bool operator==(const A& a, const B& b) { return a.v == b.w; }
int main() { A a{1}; B b{1}; return a == b && b == a ? 0 : 1; }
