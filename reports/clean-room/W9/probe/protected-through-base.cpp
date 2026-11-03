// Probe: protected state of a mixin read through a reference to the mixin (ACC-4).
class Counter { protected: int count = 0; };
class W : public Counter { public: int peek(const Counter& c) const { return c.count; } };
int main() { W w; return w.peek(w); }
