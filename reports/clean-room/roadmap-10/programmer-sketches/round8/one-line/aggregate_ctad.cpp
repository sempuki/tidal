template <class A, class B> struct Pair { A first; B second; };
int main() { Pair p{1, 2.5}; return sizeof(p.second) == sizeof(double) ? 0 : 1; }
