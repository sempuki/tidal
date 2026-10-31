// tidal-rules: VC-8
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }

struct S { int v = 4; int get() const { mark(1); return v; } ~S() { mark(2); } };
int main() { int x = S{}.get(); mark(3); return x == 4 && trace == 123 ? 0 : 1; }
