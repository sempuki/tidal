// Planted mistake: a constexpr variable from a value only known at run time.
int read() { return 3; }
int main() { constexpr int n = read(); return n - 3; }
