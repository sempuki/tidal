// Two classes that convert to int&: COND-11, an int prvalue.
int s1 = 1, s2 = 2;
struct A { operator int&() { return s1; } };
struct B { operator int&() { return s2; } };
int main() { A a; B b; int x = true ? a : b; int y = false ? a : b; return x == 1 && y == 2 ? 0 : 1; }
