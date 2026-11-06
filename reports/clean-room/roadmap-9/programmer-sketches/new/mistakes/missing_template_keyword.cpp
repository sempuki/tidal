struct T { template <int I> int get() const { return I; } };
template <class U> int first(const U& u) { return u.get<0>(); }
int main() { return first(T{}); }
