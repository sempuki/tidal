// tidal-rules: MPTR-4
// tidal-expect: run
struct B { int x = 0; };
struct D : B { int y = 6; };
int main() { int B::* pm = static_cast<int B::*>(&D::y); D d; B& b = d; return b.*pm == 6 ? 0 : 1; }
