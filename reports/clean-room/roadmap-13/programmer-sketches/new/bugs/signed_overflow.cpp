// A sum of int values overflows.
int total(int a, int b) { return a + b; }
int main() { int big = 2147483000; return total(big, 1000) < 0 ? 1 : 0; }
