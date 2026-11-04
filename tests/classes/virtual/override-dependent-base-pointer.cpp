// tidal-rules: VIRT-3 INST-2
// tidal-expect: accept
// tidal-note: A pointer to Wrapper<NoRun> does not instantiate the class, so the override that would fail is never checked.
struct NoRun { virtual int stop() { return 1; } };
template <class Base> struct Wrapper : Base { int run() override { return 2; } };
Wrapper<NoRun>* p = nullptr;
