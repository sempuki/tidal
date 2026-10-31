// tidal-rules: TYN-6
// tidal-expect: accept
struct Inc;
Inc make() = delete;
void take(Inc) = delete;
