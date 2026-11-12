struct Vec { using value_type = int; };
template <class C> int first() { C::value_type x = 0; return x; }
int main() { return first<Vec>(); }
