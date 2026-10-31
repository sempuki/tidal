// tidal-rules: VC-9
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }

struct T { int v; ~T() { mark(9); } };
int read(const T& t) { mark(t.v); return t.v; }
int main() { int a = read(T{1}), b = read(T{2}); return trace == 1929 ? 0 : 1; }
