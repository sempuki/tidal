struct S { int twice(int v) const { return v * 2; } };
int other() { return S{}.twice(3); }
