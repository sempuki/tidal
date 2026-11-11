// tidal-fe stops: "unsupported at line 6: class assignment". g++ and clang++ accept, exit 0.
// The same operator= with an int parameter, or a named member function
// in place of operator=, runs on tidal-fe and returns 0.
struct B { int k; };
struct A { int v = 0; A& operator=(B b) { v = b.k; return *this; } };
int main() { A a; a = B{3}; return a.v - 3; }
