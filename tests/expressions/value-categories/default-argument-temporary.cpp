// tidal-rules: VC-9
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }

struct T { ~T() { mark(9); } };
int use(const T& = T{}) { mark(1); return 0; }
int main() { use(); mark(2); return trace == 192 ? 0 : 1; }
