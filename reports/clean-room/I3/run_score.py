"""Write score-output.txt: grids (score.py) and individual tests (score_tests.py, batch*.py)."""
import sys
sys.path.insert(0, __file__.rsplit('/', 1)[0])
import score
import batch2, batch3, batch4, batch5, batch6  # noqa: F401  (each appends its batch)
import score_tests

g = score.main()
t = score_tests.main()
open(__file__.rsplit('/', 1)[0] + '/score-output.txt', 'w').write(g + '\n\n' + t + '\n')
print(g.split('\n\n')[0]); print(t.split('\n')[0])
