use paired::bls12_381::{Bls12, Fr, FrRepr};
use neptune::poseidon::{Poseidon, PoseidonConstants};
use neptune::{round_numbers, scalar_from_u64, Error, Scalar};
use generic_array::typenum::{U0, U11, U16, U2, U24, U36, U4, U8};
use ff::{Field, PrimeField, ScalarEngine};

fn main() {
    println!("hello codeface!");

    let constants = PoseidonConstants::<Bls12, U8>::new();

    println!("half_full_rounds {}", constants.half_full_rounds);
    println!("partial_rounds {}", constants.partial_rounds);
    //println!("mds_matrices.m {} {:?}", constants.mds_matrices.m.len(), constants.mds_matrices.m);
    //println!("compressed_round_constants {} {:?}", constants.compressed_round_constants.len(), constants.compressed_round_constants);
    //println!("pre_sparse_matrix {:?}", constants.pre_sparse_matrix);

    // println!("sparse_matrixes_w_hat {}", constants.sparse_matrixes[0].w_hat.len());
    // for e in constants.sparse_matrixes.iter() {
    //     println!("{:?},", &e.w_hat[1..]);
    // }

    // println!("sparse_matrixes_v_rest {}", constants.sparse_matrixes[0].v_rest.len());
    // for e in constants.sparse_matrixes.iter() {
    //     println!("{:?},", &e.v_rest);
    // }

    let mut p = Poseidon::<Bls12, U8>::new(&constants);

    let test_arity = constants.arity();
    let mut preimage = vec![Scalar::zero(); test_arity];
    for n in 0..test_arity {
        let scalar = scalar_from_u64::<Fr>(n as u64);
        p.input(scalar).unwrap();
        preimage[n] = scalar;
    }
    println!("result: {:?}", p.hash())
}