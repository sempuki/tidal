// tidal-rules: REF-1
// tidal-expect: accept
// tidal-note: Decision 0059, item RV5, point RV5-A-5. A function lvalue whose type has noexcept binds directly a reference to the function type without noexcept, because a pointer to the one type converts to a pointer to the other.
void tick() noexcept;
void (&plain)() = tick;
void (*const& pointer_ref)() = plain;
