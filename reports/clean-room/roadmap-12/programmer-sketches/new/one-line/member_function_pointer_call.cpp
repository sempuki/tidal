// Calls through a pointer to member function with .* and ->*.
struct Acc { int v = 0; void add(int d) { v += d; } };
int main() { void (Acc::*f)(int) = &Acc::add; Acc a; Acc* p = &a; (a.*f)(2); (p->*f)(3); return a.v - 5; }
