// tidal-rules: DCMP-1
// tidal-expect: run
int trace = 0;
struct M { int id; bool operator==(const M&) const { trace = trace * 10 + id; return true; } };
struct B { M m{1}; bool operator==(const B&) const = default; };
struct D : B { M a{2}; M b{3}; bool operator==(const D&) const = default; };
int main() { D x, y; bool r = x == y; return r && trace == 123 ? 0 : 1; }
