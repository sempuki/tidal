// Y lvalue, step 1 does not bind, step 3 converts: ToValue against B lvalue.
struct B { int m; };
struct ToValue { operator B() { return B{4}; } };
int main() { ToValue v; B bl{1}; bool c = true; B r = c ? v : bl; return r.m == 4 ? 0 : 1; }
