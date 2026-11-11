template <class T> struct Counter { static inline int n = 0; Counter() { ++n; } };
int main() { Counter<int> a, b; Counter<char> c; return Counter<int>::n == 2 && Counter<char>::n == 1 ? 0 : 1; }
