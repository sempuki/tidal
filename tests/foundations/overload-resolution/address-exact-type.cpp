// tidal-rules: OVL-18
// tidal-expect: accept
// tidal-note: A noexcept function matches a target without noexcept. The target selects by the exact function type.
void pick(int) noexcept;
void pick(long);
void (*p)(int) = pick;
void (*q)(long) = pick;
