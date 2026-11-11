template <class A, class B> concept Same = sizeof(A) == sizeof(B);
template <class Q, class... Qs>
constexpr bool all() { return (Same<Q, Qs> && ...); }
int main() { return all<int, int, int>() ? 0 : 1; }
