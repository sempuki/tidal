// tidal-rules: VC-10
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }

struct P { ~P() { mark(9); } };
int take(P) { mark(1); return 0; }
int main() { int r = (take(P{}), (mark(2), 0)); return (trace == 192 || trace == 129) ? 0 : 1; }
