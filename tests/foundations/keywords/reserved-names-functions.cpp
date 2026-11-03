// tidal-rules: KW-4
// tidal-expect: accept
// tidal-note: A function with two underscores in a row, a global variable that starts with an underscore, and a class that starts with an underscore and an uppercase letter.
int count__all() { return 0; }
int _global = 1;
struct _Widget { int v; };
