Ideas for solving part 3
========================

All three parts today are the same, so we can look at the structure of the first two parts (which
are solvable by BFS or A*) to get an idea of what might work for part 3. The mazes seem to be
solvable by the left/right-hand rule, "of course", but that does not get the shortest path possible.

The map given is also not a "_true_" maze, since the cells do not form a tree: if that were the case,
this would be a _very_ easy problem. However, we can leverage the fact that it's "kinda sorta" a
maze probably?

My idea is as follows:

1. generate **a** path by using the left/right-hand follow rule (depending on how we start
   constructing the maze, or just try out both and take the minimum length lmao);
2. try to shorten it via heuristics, determined by observing the A* solutions of part1/2.

The path is also always a sequence of horizontal/vertical lines; this fact might give us some
heuristics. Let's try to think of some of them:

* Say we have segment A and segment B, where B occurs later than A in the sequence. If prolonging
  segment B causes it to intersect with A, then we can directly join them and skip any intermediate
  segments. This might not be tractable if we have to check any previous segments, so we might have
  to think of an early exit or otherwise see if just doing this pairwise (so just windows of size
  three) is enough. 
* Same as before but we prolong A and see if it intersects B.
* I think maybe both segments should be prolonged?
* There are cases in which prolongation is not enough, sometimes you can switch directions and save
  steps: perhaps these cases could be handled by a second pass, where we check if we go horizontal,
  vertical and then horizontal in the opposite direction?
