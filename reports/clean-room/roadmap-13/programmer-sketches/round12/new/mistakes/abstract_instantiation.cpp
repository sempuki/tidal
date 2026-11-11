// Planted mistake: an object of an abstract class.
struct Codec { virtual int encode(int) = 0; virtual ~Codec() = default; };
int main() { Codec c; return 0; }
