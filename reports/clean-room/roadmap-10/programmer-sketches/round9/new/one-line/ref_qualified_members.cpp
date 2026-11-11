struct S { int v = 1; int get() & { return v; } int get() && { return v + 100; } };
int main() { S s; return s.get() == 1 && S{}.get() == 101 ? 0 : 1; }
