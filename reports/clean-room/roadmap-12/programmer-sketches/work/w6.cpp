int f(int x) { return x; }
template <class T> int f(T, T) { return 1; }
int main() { return f(0); }
