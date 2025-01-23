use crate::Unit;

#[inline(always)]
pub fn is_empty<C>(start: &C, end: &C) -> bool
where
    C: Unit,
{
    (*end - *start).is_zero()
}

#[inline(always)]
pub fn overlaps<C>(a_start: &C, a_end: &C, b_start: &C, b_end: &C) -> bool
where
    C: Unit,
{
    if is_empty(a_start, a_end) && is_empty(b_start, b_end) {
        a_start == b_end && b_start == a_end
    } else {
        a_start < b_end && b_start < a_end
    }
}

#[inline(always)]
pub fn contains<C>(a_start: &C, a_end: &C, b_start: &C, b_end: &C) -> bool
where
    C: Unit,
{
    a_start <= b_start && b_end <= a_end
}

// #[inline(always)]
// fn distance<C>(a_start: &C, a_end: &C, b_start: &C, b_end: &C) -> DirectedDistance<C>
// where
//     C: Unit, 
// {
//     if overlaps_with(a_start, a_end, b_start, b_end) {
//         DirectedDistance::Overlaps
//     } else {
//         match a_end.cmp(b_start) {
//             Ordering::Less => {
//                 // a is upstream of b.
//                 DirectedDistance::Upstream(*b_start - *a_end)
//             }
//             Ordering::Equal => {
//                 match a_start.cmp(b_end) {
//                     Ordering::Less => DirectedDistance::Upstream(C::zero()),
//                     Ordering::Equal => panic!(
//                         "Both coordinates are equal should have been caught with the previous `overlaps_with` test!"
//                     ),
//                     Ordering::Greater => DirectedDistance::Downstream(C::zero()),
//                 }
//             },
//             Ordering::Greater => {
//                 // a is downstream of b.
//                 DirectedDistance::Downstream(*a_start - *b_end)
//             }
//         }
//     }
// }