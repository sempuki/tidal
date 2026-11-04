template <class T> struct M { using vt = T; int put(); };
template <class T> int M<T>::put() { vt x = 1; return x; }
int main() { M<int> m; return m.put() == 1 ? 0 : 1; }
