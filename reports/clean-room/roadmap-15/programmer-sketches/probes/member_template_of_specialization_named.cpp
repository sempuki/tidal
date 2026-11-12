template <class R> struct Wrap { template <class S> struct skill { int v = 1; }; };
int main() { Wrap<char>::skill<int> s; return s.v - 1; }
