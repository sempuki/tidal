// tidal-rules: ACC-4
// tidal-expect: reject
class B { protected: int x = 1; };
int f(B& b) { return b.x; }
