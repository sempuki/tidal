// tidal-rules: ODR-4 ODR-8
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .Member::~Member
// tidal-error-gcc: undefined reference to .Member::~Member
// tidal-sources: new-uses-deallocation-function.tu.cpp
// tidal-note: The constructor of Whole can throw after m is built, so it uses the destructor of Member, which has no definition. No object is destroyed.
int start();
struct Member { ~Member(); };
struct Whole { Member m; int v; Whole() : v(start()) {} };
int main() { Whole* whole = new Whole; return whole->v - 1; }
