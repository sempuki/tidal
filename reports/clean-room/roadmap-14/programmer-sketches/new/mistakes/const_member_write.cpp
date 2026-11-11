struct Counter { int n = 0; int bump() const { return ++n; } };
int main() { Counter c; return c.bump() - 1; }
