// tidal-rules: VC-9
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }

struct T { int v; ~T() { mark(9); } };
const T& pass(const T& t) { return t; }
int read(const T& t) { mark(1); return t.v; }
int main() { int x = read(pass(T{5})); mark(2); return x == 5 && trace == 192 ? 0 : 1; }
