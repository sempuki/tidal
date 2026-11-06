template <int N> int first(const int (&a)[N]) { return a[0]; }
int main() { int a[2] = {0, 1}; return first(a); }
