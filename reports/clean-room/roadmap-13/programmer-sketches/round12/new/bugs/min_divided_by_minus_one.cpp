// Planted bug: the smallest int divided by -1 overflows.
int ratio(int a, int b) { return a / b; }
int main() { int m = -2147483647 - 1; volatile int d = -1; return ratio(m, d) == 0; }
