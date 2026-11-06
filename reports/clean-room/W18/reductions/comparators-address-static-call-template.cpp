struct G { template <class T> static int operator()(T a) { return a; } };
int main() { int (*p)(int) = &G::operator(); return p(0); }
