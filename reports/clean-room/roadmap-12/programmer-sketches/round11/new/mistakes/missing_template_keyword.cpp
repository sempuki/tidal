// A member template of a dependent object called without the template keyword.
struct Tuple { template <int I> int get() const { return I; } };
template <class T> int second(const T& t) { return t.get<1>(); }
int main() { return second(Tuple{}) - 1; }
