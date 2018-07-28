use matrix::Matrix;

macro_rules! impl_from {
	($from:ident, $to:ident) => {
		impl From<Matrix<$from>> for Matrix<$to> {
    		fn from(f: Matrix<$from>) -> Self {
    			Matrix {
    				rows: f.rows,
    				cols: f.cols,
    				data: f.into_iter().map($to::from).collect(),
    			}
    		}
    	}

    	impl<'a> From<&'a Matrix<$from>> for Matrix<$to> {
    		fn from(f: &'a Matrix<$from>) -> Self {
    			Matrix {
    				rows: f.rows,
    				cols: f.cols,
    				data: f.into_iter().map(|n| $to::from(*n)).collect(),
    			}
    		}
    	}
	};

    ($from:ident, $to:ident, $($more:ident),* ) => {
    	impl_from!($from, $to);
    	impl_from!($from, $($more),*);
    };
}

// `impl<T, U> From<Matrix<U>> for Matrix<T>` doesn't work,
// so we'll have to manually tell the compiler

// impl_from!(T, U1, U2, ...);
// Checked (T, U) couples where U implement From<T> from:
// https://doc.rust-lang.org/src/core/num/mod.rs.html#4510-4581
// (that may be macro-able but I'm not too familiar with them for that)

// Unsigned -> Unsigned
impl_from!(u8, u16, u32, u64, u128, usize);
impl_from!(u16, u32, u64, u128);
impl_from!(u32, u64, u128);
impl_from!(u64, u128);

// Signed -> Signed
impl_from!(i8, i16, i32, i64, i128, isize);
impl_from!(i16, i32, i64, i128);
impl_from!(i32, i64, i128);
impl_from!(i64, i128);

// Unsigned -> Signed
impl_from!(u8, i16, i32, i64, i128);
impl_from!(u16, i32, i64, i128);
impl_from!(u32, i64, i128);
impl_from!(u64, i128);

// (from C99 standard)
impl_from!(u16, usize);
impl_from!(u8, isize);
impl_from!(i16, isize);

// Signed -> Float
impl_from!(i8, f32, f64);
impl_from!(i16, f32, f64);
impl_from!(i32, f64);

// Unsigned -> Float
impl_from!(u8, f32, f64);
impl_from!(u16, f32, f64);
impl_from!(u32, f64);

// Float -> Float
impl_from!(f32, f64);
