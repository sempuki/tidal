struct Acc { int sum = 0; void operator()(int x) { sum += x; } };
template <class It, class F> F for_each(It b, It e, F f) { for (; b != e; ++b) f(*b); return f; }
int main() { int a[] = {1, 2, 3, 4}; Acc r = for_each(a, a + 4, Acc{}); return r.sum == 10 ? 0 : 1; }
