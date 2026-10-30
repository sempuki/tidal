# Q-0008: Are the similarity thresholds (8-word run, 0.25 overlap) right?

**Status:** provisional
**Affects:** `tools/similarity.py`, decision 0011

## Context

The similarity check flags a rule that shares a run of 8 or more words with
the reference corpus, or whose 5-word sequences overlap it by more than 25%.
Short shared phrases are expected, because Tidal reuses industry terms
(decision 0004).

## Options

1. Keep the current thresholds.
2. Tighten them (for example, a 6-word run) and accept more rewrites.
3. Loosen them and rely on review.

## Provisional answer

Option 1 until reports from more sections show how many flags each setting
produces.

## Opinions
