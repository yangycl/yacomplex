use crate::types::types::Complex;

impl Complex{
    pub fn new(r:f64, i:f64) -> Self{
        Self{
            r,
            i
        }
    }
    
    pub fn add(a:Complex, b:Complex) -> Complex{
        Complex::new(a.r + b.r, a.i + b.i)
    }

    pub fn sub(a:Complex, b:Complex) -> Complex{
        Complex::new(a.r - b.r, a.i - b.i)
    }
    
    pub fn mul(a:Complex, b:Complex) -> Complex{
        Complex::new(a.r * b.r - a.i * b.i, a.r * b.i + a.i * b.r)
    }

    pub fn div(a:Complex, b:Complex) -> Complex{
        let denominator = b.r * b.r + b.i * b.i;
        Complex::new(
            (a.r * b.r + a.i * b.i) / denominator,
            (a.i * b.r - a.r * b.i) / denominator,
        )
    }
}