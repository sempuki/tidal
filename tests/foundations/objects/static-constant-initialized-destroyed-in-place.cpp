// tidal-rules: OBJ-4
// tidal-expect: run
// tidal-note: b is constant-initialized, so its construction completes before a's. b still takes the place of its definition, after a, and is destroyed first. Last is destroyed last and checks the order. The write through a null pointer runs only when the order is wrong.
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
int dynamic_one() { return 1; }
struct Last { ~Last() { if (trace != 21) *(volatile int*)0 = 0; } } last;
struct Dynamic { int id; Dynamic(int i) : id(i) {} ~Dynamic() { mark(id); } };
struct Constant { int id; constexpr Constant(int i) : id(i) {} ~Constant() { mark(id); } };
Dynamic a(dynamic_one());
Constant b(2);
int main() { return 0; }
