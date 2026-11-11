// A member template of a dependent object is called without template.
struct Tuple { template <int I> int get() const { return I; } };
template <class T> int first(const T& t) { return t.get<0>(); }
int main() { return first(Tuple{}); }
