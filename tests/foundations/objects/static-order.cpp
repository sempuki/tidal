// tidal-rules: OBJ-4
// tidal-expect: run
// tidal-note: The object declared first is destroyed last. Its destructor checks the final order. The write through a null pointer runs only when the order is wrong, so a passing run has no undefined behavior.
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };

struct Last { ~Last() { if (trace != 123321) *(volatile int*)0 = 0; } } last;
T first(1);
T second(2);
int at_main = 0;
int main() { at_main = trace; T local(3); return at_main == 12 ? 0 : 1; }
