template <class T> int f(T&) { return 1; }
namespace n { struct C {}; int f(C&) { return 0; } }
int main() { n::C c; return f(c); }
