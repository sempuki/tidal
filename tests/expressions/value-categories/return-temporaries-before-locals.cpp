// tidal-rules: VC-9
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }

struct L { ~L() { mark(3); } };
struct T { int v; ~T() { mark(2); } };
int value(const T& t) { mark(1); return t.v; }
int f() { L local; return value(T{4}); }
int main() { int r = f(); return r == 4 && trace == 123 ? 0 : 1; }
