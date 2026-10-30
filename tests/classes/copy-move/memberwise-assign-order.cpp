// tidal-rules: CPY-8
// tidal-expect: run
int trace = 0;
struct T { int id; T(int i) : id(i) {} T& operator=(const T&) { trace = trace * 10 + id; return *this; } };
struct B { T t{1}; };
struct S : B { T a{2}; T b{3}; };
int main() { S x, y; y = x; return trace == 123 ? 0 : 1; }
