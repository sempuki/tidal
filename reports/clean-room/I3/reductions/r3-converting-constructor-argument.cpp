struct Source {};
struct Wrap { Wrap(Source) {} };
int take(Wrap) { return 0; }
int main() { return take(Source{}); }
