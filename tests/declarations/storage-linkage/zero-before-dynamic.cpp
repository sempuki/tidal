// tidal-rules: STO-3
// tidal-expect: run
// tidal-note: The initializer of early reads late before the initializer of late runs. The storage of late is already zero at that point.
int compute() { return 42; }
int read_late();
int early = read_late() + 1;
int late = compute();
int read_late() { return late; }
int main() { return early == 1 && late == 42 ? 0 : 1; }
