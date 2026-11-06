// Two classes that convert to int&: COND-11 gives a prvalue, so & is rejected.
int s1 = 1, s2 = 2;
struct A { operator int&() { return s1; } };
struct B { operator int&() { return s2; } };
int main() { A a; B b; bool c = true; int* p = &(c ? a : b); return *p == 1 ? 0 : 1; }
