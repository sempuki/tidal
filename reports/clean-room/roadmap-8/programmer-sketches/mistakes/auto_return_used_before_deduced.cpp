auto fact(int n);
int use() { return fact(3); }
auto fact(int n) { return n < 2 ? 1 : n * fact(n - 1); }
int main() { return use(); }
