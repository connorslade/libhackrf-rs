use num_complex::Complex;

pub trait ToComplexI8 {
    fn to_i8(self) -> Complex<i8>;
}

impl ToComplexI8 for Complex<f32> {
    fn to_i8(self) -> Complex<i8> {
        Complex::new((self.re * 127.0) as i8, (self.im * 127.0) as i8)
    }
}
