// Probe: pointer to a protected mixin member formed with the base name inside the derived class (ACC-4).
class Counter { protected: int count = 0; };
class W : public Counter { public: static int W::* f() { return &Counter::count; } };
int main() { W w; return w.*W::f(); }
