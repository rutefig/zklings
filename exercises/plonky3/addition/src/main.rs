use std::fmt::Debug;
use std::marker::PhantomData;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{AbstractField, Field};
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::Matrix;

use p3_challenger::{HashChallenger, SerializingChallenger32};
use p3_circle::CirclePcs;
use p3_commit::ExtensionMmcs;
use p3_field::extension::BinomialExtensionField;
use p3_fri::FriConfig;
use p3_keccak::Keccak256Hash;
use p3_merkle_tree::FieldMerkleTreeMmcs;
use p3_mersenne_31::Mersenne31;
use p3_symmetric::{CompressionFunctionFromHasher, SerializingHasher32};
use p3_uni_stark::{prove, verify, StarkConfig};
use tracing_forest::util::LevelFilter;
use tracing_forest::ForestLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

pub struct AdditionAir {
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

impl<F: Field> BaseAir<F> for AdditionAir {
    fn width(&self) -> usize {
        3
    }
}

impl<AB: AirBuilder> Air<AB> for AdditionAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);

        builder
            .when_first_row()
            .assert_eq(local[0] + local[1], local[2]);

        let final_value = AB::Expr::from_canonical_u32(self.c);
        builder.when_first_row().assert_eq(local[2], final_value);
    }
}

pub fn generate_addition_trace<F: Field>(a: u32, b: u32, c: u32) -> RowMajorMatrix<F> {
    let mut values = Vec::with_capacity(3 * 4);
    let a_f = F::from_canonical_u32(a);
    let b_f = F::from_canonical_u32(b);
    let c_f = F::from_canonical_u32(c);

    values.push(a_f);
    values.push(b_f);
    values.push(c_f);

    for _ in 0..3 {
        values.push(F::zero());
        values.push(F::zero());
        values.push(F::zero());
    }

    RowMajorMatrix::new(values, 3)
}

fn main() -> Result<(), impl Debug> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    Registry::default()
        .with(env_filter)
        .with(ForestLayer::default())
        .init();

    type Val = Mersenne31;
    type Challenge = BinomialExtensionField<Val, 3>;

    type ByteHash = Keccak256Hash;
    type FieldHash = SerializingHasher32<ByteHash>;
    let byte_hash = ByteHash {};
    let field_hash = FieldHash::new(Keccak256Hash {});

    type MyCompress = CompressionFunctionFromHasher<u8, ByteHash, 2, 32>;
    let compress = MyCompress::new(byte_hash);

    type ValMmcs = FieldMerkleTreeMmcs<Val, u8, FieldHash, MyCompress, 32>;
    let val_mmcs = ValMmcs::new(field_hash, compress);

    type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
    let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());

    type Challenger = SerializingChallenger32<Val, HashChallenger<u8, ByteHash, 32>>;

    let fri_config = FriConfig {
        log_blowup: 1,
        num_queries: 100,
        proof_of_work_bits: 16,
        mmcs: challenge_mmcs,
    };

    type Pcs = CirclePcs<Val, ValMmcs, ChallengeMmcs>;
    let pcs = Pcs {
        mmcs: val_mmcs,
        fri_config,
        _phantom: PhantomData,
    };

    type MyConfig = StarkConfig<Pcs, Challenge, Challenger>;
    let config = MyConfig::new(pcs);

    let a = 4;
    let b = 5;
    let c = 9;
    let air = AdditionAir { a, b, c };
    let trace = generate_addition_trace::<Val>(a, b, c);

    let mut challenger = Challenger::from_hasher(vec![], byte_hash);
    let proof = prove(&config, &air, &mut challenger, trace, &vec![]);

    let mut challenger = Challenger::from_hasher(vec![], byte_hash);
    verify(&config, &air, &mut challenger, &proof, &vec![])
}
