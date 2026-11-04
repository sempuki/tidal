// tidal-rules: TRY-5
// tidal-expect: accept
// tidal-note: An attribute can stand at the start of the handler parameter and applies to the parameter.
void g() { try { } catch ([[maybe_unused]] int e) { } }
