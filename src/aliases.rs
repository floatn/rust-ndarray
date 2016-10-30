//! Type aliases for common array sizes
//!

use ::{Ix, Array, ArrayView, ArrayViewMut};

#[allow(non_snake_case)]
#[inline(always)]
pub fn Ix0() -> Ix0 { [] }
#[allow(non_snake_case)]
#[inline(always)]
pub fn Ix1(i0: Ix) -> Ix1 { [i0] }
#[allow(non_snake_case)]
#[inline(always)]
pub fn Ix2(i0: Ix, i1: Ix) -> Ix2 { [i0, i1] }

/// zero-dimensionial
pub type Ix0 = [Ix; 0];
/// one-dimensional
pub type Ix1 = [Ix; 1];
/// two-dimensional
pub type Ix2 = [Ix; 2];
/// three-dimensional
pub type Ix3 = [Ix; 3];
/// four-dimensional
pub type Ix4 = [Ix; 4];
/// five-dimensional
pub type Ix5 = [Ix; 5];
/// dynamic-dimensional
pub type IxDyn = Vec<Ix>;

/// zero-dimensional array
pub type Array0<A> = Array<A, Ix0>;
/// one-dimensional array
pub type Array1<A> = Array<A, Ix1>;
/// two-dimensional array
pub type Array2<A> = Array<A, Ix2>;
/// three-dimensional array
pub type Array3<A> = Array<A, Ix3>;
/// four-dimensional array
pub type Array4<A> = Array<A, Ix4>;
/// dynamic-dimensional array
pub type ArrayD<A> = Array<A, IxDyn>;

/// zero-dimensional array view
pub type ArrayView0<'a, A> = ArrayView<'a, A, Ix0>;
/// one-dimensional array view
pub type ArrayView1<'a, A> = ArrayView<'a, A, Ix1>;
/// two-dimensional array view
pub type ArrayView2<'a, A> = ArrayView<'a, A, Ix2>;
/// three-dimensional array view
pub type ArrayView3<'a, A> = ArrayView<'a, A, Ix3>;
/// four-dimensional array view
pub type ArrayView4<'a, A> = ArrayView<'a, A, Ix4>;
/// dynamic-dimensional array view
pub type ArrayViewD<'a, A> = ArrayView<'a, A, IxDyn>;

/// zero-dimensional read-write array view
pub type ArrayViewMut0<'a, A> = ArrayViewMut<'a, A, Ix0>;
/// one-dimensional read-write array view
pub type ArrayViewMut1<'a, A> = ArrayViewMut<'a, A, Ix1>;
/// two-dimensional read-write array view
pub type ArrayViewMut2<'a, A> = ArrayViewMut<'a, A, Ix2>;
/// three-dimensional read-write array view
pub type ArrayViewMut3<'a, A> = ArrayViewMut<'a, A, Ix3>;
/// four-dimensional read-write array view
pub type ArrayViewMut4<'a, A> = ArrayViewMut<'a, A, Ix4>;
/// dynamic-dimensional read-write array view
pub type ArrayViewMutD<'a, A> = ArrayViewMut<'a, A, IxDyn>;
