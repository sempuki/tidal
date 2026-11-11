// Assignment to a class object from an int: the implicit copy or move
// assignment takes a temporary that S(int) makes.
struct S { int n = 0; S(int x) : n(x) {} };
int main() { S s(1); s = 5; return s.n == 5 ? 0 : 1; }
