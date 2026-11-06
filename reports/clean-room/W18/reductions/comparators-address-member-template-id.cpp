struct G { template <class T> static int f(T a) { return a; } };
int main() { int (*p)(int) = &G::f<int>; return p(0); }
