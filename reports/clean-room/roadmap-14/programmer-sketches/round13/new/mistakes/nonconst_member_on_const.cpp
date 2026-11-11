// A non-const member function is called on a const object.
struct Counter { int n = 0; int bump() { return ++n; } };
int use(const Counter& c) { return c.bump(); }
int main() { Counter c; return use(c) - 1; }
