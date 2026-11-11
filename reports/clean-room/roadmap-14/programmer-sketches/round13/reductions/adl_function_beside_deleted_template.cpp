namespace n { struct C {}; int f(C&) { return 0; } }
template <class T> int f(T&) = delete;
int main() { n::C c; return f(c); }
