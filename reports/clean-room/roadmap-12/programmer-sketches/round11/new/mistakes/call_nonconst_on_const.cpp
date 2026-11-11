// A non-const member function called on a const object.
struct Counter { int n = 0; int next() { return ++n; } };
int use(const Counter& c) { return c.next(); }
int main() { Counter c; return use(c); }
