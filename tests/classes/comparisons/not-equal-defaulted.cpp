// tidal-rules: DCMP-3
// tidal-expect: run
struct S { int x; bool operator==(const S&) const = default; bool operator!=(const S&) const = default; };
int main() { return S{1} != S{2} && !(S{1} != S{1}) ? 0 : 1; }
