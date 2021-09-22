# *Powierża distance*

Given a sequence `S` and its subsequence `P`, *Powierża distance* is the least number of disjoint subsequences of consecutive elements in a sequence `S` — `s_i` — that, when concatenated in their original order, are equal to `P`, less 1. Alternatively, it is the number of gaps between subsequences `s_i`.


## Examples

Let `P` be `abcjkl` and `S` be `abcjkl`. The *Powierża distance* between them is 0, since only one subsequence is needed to 'cover' entire `P` — `S`.

Let `P` be `abcjkl` and `S` be `abc_jkl` (`_` functions as a normal element in this sequence). The *Powierża distance* between them is 1. That's because there are two subsequences of consecutive elements of `S`, `s_1`: `abc` and `s_2`: `jkl`, so there's one gap.

Let `P` be `abcjkl` and `S` be `__abc_jk_abcj_l__`. The *Powierża distance* between them is 2. The subsequences are `abc`, `jk` and `l`.


# *Powierża algorithm*

The algorithm was inspired by [Wagner–Fischer algorithm
](https://en.wikipedia.org/wiki/Wagner%E2%80%93Fischer_algorithm). Whereas in Wagner-Fischer algorithm (*WF*) there are 3 types of moves (horizontal, diagonal and vertical) in my algorithm there are only two — horizontal and diagonal. The main idea is that the 'cost' of a gap is always 1, despite its length. In *WF* the cost of a gap is it's length.

That means the algorithm must differentiate between cells that were filled in horizontal moves and the ones that were filled in diagonal moves. The first type of cells are cells containing `Gap(score)`; the second type — `Continuation(score)`. A horizontal move results in `Gap(score)` if the original cell contains `Gap(score)` and in `Gap(score + 1)` if the original cell contains `Continuation(score)`. The algorithm prefers moves that result in lower score and a diagonal move over horizontal move if they result in the same score.

1. Create a matrix `m` rows by `n` cols where `m` is the length of `S` and `n` is the length of `P`. `n` must be less or equal to `m`. Each cell can either be empty (that's the initial state) or contain either `Gap(score)` or `Continuation(score)`.
2. Begin filling the matrix from left to right and from top to bottom. The first row is special — `xth`, `yth` cell is set to `Continuation(0)` if the `xth` element of `S` and the `yth` element of `P` are equal. Otherwise, is set to `Gap(score + cost)` where `score` is the score of its left neighbor. If its left neighbor is empty, the cell is left empty as well.
3. Other cells are filled according to these rules:

   Let `x` be `a`'s upper-left neighbor and `y` be its left neighbor:

   ```
   x _
   y a
   ```
   
   The cost of a diagonal move is 0 but such move is only possible if the `xth` element of `S` and the `yth` element of `P` are equal and if `x` isn't empty. After the move `a` is set to `Continuation(score)` where `score` is `x`'s score.

   The cost of a horizontal move is 0 if `y` contains `Gap` and 1 if `y` contains `Continuation`. Such move is only possible if `y` isn't empty. After the move `a` is set to `Gap(score + cost)` where `score` is `y`'s score.

   * If there are no available moves, leave `a` empty.
   * If there's only one available move, make it.
   * If there are two available moves and their scores are equal, make the diagonal move.
   * If there are two available moves and their scores aren't equal, make the move with the least score.
4. *Powierża distance* is the least value in the last row. In some cases there are no values in the last row and the distance is not defined.


## Illustration

Cells with B's were filled in horizontal moves and those with G's were filled in diagonal moves. The numbers next to the letters are cells' scores. The distance is 2.

![image](https://user-images.githubusercontent.com/20628866/134387055-24dfec18-159e-42cc-8d1b-c4ef15ce7046.png)

