// tidal-rules: NAM-21
// tidal-expect: run
// tidal-note: The body of a class defined with the qualified name shapes::Box sees the members of shapes (Q-0377).
namespace shapes {
using length = int;
struct Box;
}
struct shapes::Box { length width = 2; };
int main() { shapes::Box b; return b.width == 2 ? 0 : 1; }
