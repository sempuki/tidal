extern int start;
// Reads `start` from another unit during start-up. SINIT-3: gets 100 or 0.
int seen = start;
int seen_value() { return seen; }
