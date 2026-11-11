// A non-const lvalue reference is bound to a temporary.
int twice(int& x) { x *= 2; return x; }
int main() { return twice(3) - 6; }
