use k256::{elliptic_curve::CurveArithmetic, Secp256k1};

pub type PublicKey = <Secp256k1 as CurveArithmetic>::AffinePoint;