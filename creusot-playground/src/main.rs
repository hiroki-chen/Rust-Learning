extern crate creusot_contracts;

use creusot_contracts::*;

// Prove that this function implements the sum of the first n natural numbers.
// Hint: there exists a closed form of this sum.
// The prover cannot 100% be sure that the following function is valid;
// however, it cannot give any counter-example. So we admit it.
#[requires(@n < 1000)]
#[ensures(@result == @n * (@n + 1) / 2)]
pub fn sum_first_n(n: u32) -> u32 {
    let mut sum = 0;
    let mut i = 0;

    #[invariant(bound, i <= n)]
    #[invariant(sum_correct, @sum == @i * (@i + 1) / 2)]
    while i < n {
        i += 1;
        sum += i;
    }
    sum
}

#[ensures(@result == (@rhs, @lhs))]
fn pair(lhs: u32, rhs: u32) -> (u32, u32) {
    (rhs, lhs)
}

#[requires((@vec).len() < @usize::MAX)]
#[ensures(forall<i: Int> 0 <= i && i < (@^vec).len() ==> (@^vec)[i] == 0u32)]
#[ensures((@*vec).len() == (@^vec).len())]
fn zeroize_vector(vec: &mut Vec<u32>) {
    let old_v = ghost! { vec };

    let mut i = 0;

    #[invariant(proph_const, ^vec == ^old_v.inner())]
    #[invariant(in_bounds, (@*vec).len() == (@*old_v.inner()).len())]
    // Do not use as to cast between types. You need to call `into`.
    #[invariant(all_zero, forall<j : Int> 0 <= j && j < i.into() ==> (@*vec)[j] == 0u32)]
    while i < vec.len() {
        vec[i] = 0;
        i += 1;
    }
}

// Prove:
// 1. If `search` returns None, then none of element matches `target`.
// 2. If `search` returns Some, then if must be the first index.
#[requires((@vec).len() < @usize::MAX)]
// #[ensures(@result == None ==> forall<x : Int> 0 <= x && x <= (@vec).len() ==> @(@vec)[x] != @target)]
#[ensures(forall<x : Int> @result == Some(x) ==> @(@vec)[x] == @target)]
fn search<T>(vec: &Vec<T>, target: &T) -> Option<usize>
where
    T: Ord + Model,
{
    let mut i = 0 as usize;
    let mut ans: Option<usize> = None;

    #[invariant(in_bounds, @i < (@vec).len())]
    #[invariant(correct, @target == @(@vec)[@i] ==> @ans == Some(@i))]
    while i < vec.len() {
        if target == &vec[i] {
            ans = Some(i);
            return ans;
        }

        i += 1;
    }

    ans
}

fn main() {}
