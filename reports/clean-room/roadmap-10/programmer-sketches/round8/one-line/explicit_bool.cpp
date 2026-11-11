template <class T> struct S { template <class U> explicit(sizeof(U) > sizeof(T)) S(U) {} };
void take(S<long>) {}
int main() { take(1); S<char> s(1); (void)s; }
