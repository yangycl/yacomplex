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
}