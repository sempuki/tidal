// tidal-rules: LOOP-3
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { ~T() { mark(9); } };
int main() { for (T t; ; ) { mark(1); break; } mark(2); return trace == 192 ? 0 : 1; }
