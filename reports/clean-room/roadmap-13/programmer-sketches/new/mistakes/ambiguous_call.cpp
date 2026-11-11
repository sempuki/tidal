// A call matches two overloads equally well.
int f(long) { return 1; }
int f(unsigned) { return 2; }
int main() { return f(5); }
