struct G { template <class T> static int operator()(T a) { return a; } };
int main() { int (*p)(int) = &G::operator()<int>; return p(0); }
