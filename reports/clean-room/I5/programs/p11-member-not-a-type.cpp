// DED-24: T::type names a static data member, so typename T::type is not a type.
struct S { static const int type = 0; };
template <class T> int f(T, typename T::type* = nullptr) { return 1; }
int f(...) { return 2; }
int main() { return f(S{}) == 2 ? 0 : 1; }
