# psst-rs
Partially-Sorted-Search-Table

This serves as a reference implementation of the PSST structure. This is data
structure that works similarly to a HashMap/Set or BTreeMap/Set.

The main idea is to have a compact data structure that has zero memory overhead
compared to pushing to a contiguous vector. The structure sits in between two
extremes; a unordered vector, and a sorted vector.

An unordered vector has amortized push time O(1), but a search time of O(n).
For a set-like structure, one must search when inserting in order to not insert
duplicate elements, so both insert and search will be O(n). Collecting n items
into an empty structure then takes O(n^2) time.

A sorted vector can be binary searched in O(log(n)) time, inserting requires
shifting the entire array to make space for the new element (O(n) time).
Collecting n items will then take O(n log(n)) search time and O(n^2) insertion
time.

The partially sorted search table is a compromise between these extremes by not
sorting the structure after every element is inserted. Instead the tail end of
the array is left unsorted until the cost of linear search becomes large enough
to warrent sorting the entire array.

## The structure

The structure will look something like this:

------------------------------------- -----
            S                           U
-------------------------------------------
                    C

The entire collection, C, consists of a large sorted block, S, and a smaller
unsorted block, S. We will be using the number of items in the sorted section,
ns, the number of items in the unsorted block, nu, and the total number of items
being n = ns + nu.

Typically we will assume the number of unsorted items being less than some
threshold, k, and assume the sorted block makes up asymptotically most of the
items such that O(ns) = O(n) and O(nu) = O(k / 2)

### Searching

We first binary search through the sorted block, then if not yet found we do a
linear search through the unsorted block. Worst case runtime occurs when the
item is not present, which is a common occurance, and will be the basis for
optimizing the algorithm. This runtime is O(log(ns) + nu) = O(log(n) + k / 2)

### Inserting

Insetion will always first search to determine if the element already exists.
If the element does not exist, we will just append the element to the end of the
unsorted block. For all complexity analysis we will assume the buffer is always
pre-allocated, and disregard any time spent reallocating. When the insertion
leads to the unsorted block overflowing, we will sort the entire array, leaving
the unsorted block empty.

For a unsorted capacity of k, the probability of an insertion triggering a sort
is 1 / k. The cost of insertion is then O(search) + O(sort / k).

Since this structure is meant to use zero memory overhead, we will use a sorting
algorithm with O(1) additional space complexity. The simplest is running a
in-place quicksort (or similar) with complexity O(n log(n)) for the entire
array.

#### Notes on more optimal sorting complexity

Since most of the array is already sorted, a more efficient algorithm could be
to sort the unsorted block in O(k log(k)) time, then merge the two sorted blocks
in O(n) time, however this would require additional memory. One could probably
be smart in using the additional capacity of the buffer, but for the current
analysis will assume O(n log(n)) sorting complexity

#### Optimizing sorting threshold

The total (amortized) runtime of insertion is then
O(log(n) + k / 2 + n log(n) / k). We can differentiate with respect to k and
set equal to zero to minimize:
1/2 - n log(n) / k^2 = 0
k^2 = 2 n log(n)
k = \sqrt{2 n log(n)}

The insertion complexity is then
O(log(n) + \sqrt{2 n log(n)} / 2 + n log(n) / \sqrt{2 n log(n)})
=
O(log(n) + \sqrt{2 n log(n)})
