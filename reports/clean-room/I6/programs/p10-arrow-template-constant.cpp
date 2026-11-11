// DEP-24: the template keyword after -> with a constant parameter as the argument.
struct Table { template <int N> int at() const { return N * 2; } };
template <class T, int N> int read(const T* t) { return t->template at<N>(); }
int main() { Table t; return read<Table, 5>(&t) == 10 ? 0 : 1; }
