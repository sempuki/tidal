// ICS-34 before ICS-28 (Q-0152). Tidal: take(const short&) wins.
enum Level : short { low };
int take(const short&) { return 1; }
int take(int&&) { return 2; }
int main() { return take(low) == 1 ? 0 : 1; }
