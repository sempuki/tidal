// tidal-rules: VC-9
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }

struct T { bool ok = true; ~T() { mark(9); } };
int main() { if (T{}.ok) mark(1); return trace == 91 ? 0 : 1; }
