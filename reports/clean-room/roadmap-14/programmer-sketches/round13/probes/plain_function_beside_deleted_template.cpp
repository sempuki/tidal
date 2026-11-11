template <class T> int f(T&) = delete;
int f(int&) { return 0; }
int main() { int c = 0; return f(c); }
