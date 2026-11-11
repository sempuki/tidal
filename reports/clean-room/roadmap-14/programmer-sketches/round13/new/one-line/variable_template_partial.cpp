// A partial specialization of a variable template.
template <class T> constexpr int rank = 0;
template <class T> constexpr int rank<T*> = 1 + rank<T>;
template <class T, int N> constexpr int rank<T[N]> = 10 + rank<T>;
static_assert(rank<int**> == 2 && rank<int*[3]> == 11);
int main() { return rank<char***> - 3; }
