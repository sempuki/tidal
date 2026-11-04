// tidal-rules: TR-3
// tidal-expect: run
// tidal-note: A function named main in a namespace is an ordinary function. Only main at global scope is the entry point.
namespace app { int main() { return 1; } }
int main() { return app::main() - 1; }
