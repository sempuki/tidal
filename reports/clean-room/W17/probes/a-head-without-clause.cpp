// Probe: CNS-26 and TPL-23. The definition outside the class leaves out
// the class's requires clause. Expected from the specification: reject.
template <class T> concept Small = sizeof(T) <= 8;
template <Small T, int N> requires (N > 0)
struct Box { int size() const; };
template <Small T, int N>
int Box<T, N>::size() const { return N; }
int main() { return Box<int, 1>{}.size() - 1; }
