// DED-15 with a private base: deduction finds Base<int>, and the conversion is not accessible.
template <class T> struct Base {};
struct D : private Base<int> {};
template <class T> int up(Base<T>&) { return 1; }
int main() { D d; return up(d); }
