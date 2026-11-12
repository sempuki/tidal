struct S { int f() const { return 0; } };
using Getter = int (S::*)() const;
int main() { Getter g = &S::f; return (S{}.*g)(); }
