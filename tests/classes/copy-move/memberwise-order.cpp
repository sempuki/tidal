// tidal-rules: CPY-8
// tidal-expect: run
int trace = 0;
struct T { int id; T(int i) : id(i) {} T(const T& o) : id(o.id) { trace = trace * 10 + id; } };
struct B1 { T t{1}; };
struct B2 { T t{2}; };
struct S : B1, B2 { T a{3}; T b{4}; };
int main() { S x; S y(x); return trace == 1234 ? 0 : 1; }
