template <class A, class B> struct Pair { A first; B second; };
constexpr Pair p{1, 2.5};
static_assert(p.first == 1 && p.second == 2.5);
int main() { Pair q{'a', 3L}; return q.second == 3 ? 0 : 1; }
