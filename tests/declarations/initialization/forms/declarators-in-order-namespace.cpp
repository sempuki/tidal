// tidal-rules: INIT-6
// tidal-expect: run
// tidal-note: Three declarators at namespace scope with dynamic initialization.
int trace = 0;
int mark(int d) { trace = trace * 10 + d; return d; }
int a = mark(1), b = mark(2), c = mark(3);
int main() { return trace == 123 && a + b + c == 6 ? 0 : 1; }
