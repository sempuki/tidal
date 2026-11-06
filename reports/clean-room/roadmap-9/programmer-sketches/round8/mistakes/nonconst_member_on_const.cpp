struct C { int n = 0; int get() { return n; } };
int main() { const C c; return c.get(); }
