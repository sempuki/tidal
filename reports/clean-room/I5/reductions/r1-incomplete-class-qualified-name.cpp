// I5-a. T::type with T an incomplete class. GCC and Clang drop the template and call f(...), exit 0. The front end stops: unsupported, parameter of unknown type.
struct X;
template <class T> int f(T*, typename T::type = 0) { return 1; }
int f(...) { return 2; }
int main() { X* p = nullptr; return f(p) - 2; }
