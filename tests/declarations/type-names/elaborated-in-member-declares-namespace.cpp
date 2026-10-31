// tidal-rules: TYN-4
// tidal-expect: accept
struct Holder { struct Later* p; };
struct Later { int v; };
Later obj{1};
int get(Holder h) { return h.p == &obj ? obj.v : 0; }
