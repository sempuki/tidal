struct C { int a[3] = {}; int n{}; };
int main() { C c; return c.a[2] + c.n; }
