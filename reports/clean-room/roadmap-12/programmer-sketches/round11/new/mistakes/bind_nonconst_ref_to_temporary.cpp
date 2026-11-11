// A reference to non-const bound to a temporary.
int twice(int& v) { return v *= 2; }
int main() { return twice(3) - 6; }
