use algebra::{
    bls12_381::{Fr, G1Projective},
    mnt6_753::{Fr as MNT6Fr, G1Projective as MNT6G1Projective},
};

use rcmath::{fft::domain::*, test_rng, PrimeField, UniformRand};

#[cfg(test)]
mod tests {
    use crate::{EvaluationDomain, Radix2EvaluationDomain};
    use algebra::bls12_381::Fr;
    use algebra_core::{test_rng, Field, Zero};
    use rand::Rng;

    #[test]
    fn vanishing_polynomial_evaluation() {
        let rng = &mut test_rng();
        for coeffs in 0..10 {
            let domain = Radix2EvaluationDomain::<Fr>::new(coeffs).unwrap();
            let z = domain.vanishing_polynomial();
            for _ in 0..100 {
                let point = rng.gen();
                assert_eq!(
                    z.evaluate(point),
                    domain.evaluate_vanishing_polynomial(point)
                )
            }
        }
    }

    #[test]
    fn vanishing_polynomial_vanishes_on_domain() {
        for coeffs in 0..1000 {
            let domain = Radix2EvaluationDomain::<Fr>::new(coeffs).unwrap();
            let z = domain.vanishing_polynomial();
            for point in domain.elements() {
                assert!(z.evaluate(point).is_zero())
            }
        }
    }

    #[test]
    fn size_of_elements() {
        for coeffs in 1..10 {
            let size = 1 << coeffs;
            let domain = Radix2EvaluationDomain::<Fr>::new(size).unwrap();
            let domain_size = domain.size();
            assert_eq!(domain_size, domain.elements().count());
        }
    }

    #[test]
    fn elements_contents() {
        for coeffs in 1..10 {
            let size = 1 << coeffs;
            let domain = Radix2EvaluationDomain::<Fr>::new(size).unwrap();
            for (i, element) in domain.elements().enumerate() {
                assert_eq!(element, domain.group_gen.pow([i as u64]));
            }
        }
    }

    #[test]
    #[cfg(feature = "parallel")]
    fn parallel_fft_consistency() {
        use super::serial_radix2_fft;
        use crate::domain::utils::parallel_fft;
        use crate::Vec;
        use algebra::bls12_381::Bls12_381;
        use algebra_core::{test_rng, PairingEngine, UniformRand};
        use core::cmp::min;

        fn test_consistency<E: PairingEngine, R: Rng>(rng: &mut R, max_coeffs: u32) {
            for _ in 0..5 {
                for log_d in 0..max_coeffs {
                    let d = 1 << log_d;

                    let mut v1 = (0..d).map(|_| E::Fr::rand(rng)).collect::<Vec<_>>();
                    let mut v2 = v1.clone();

                    let domain = Radix2EvaluationDomain::new(v1.len()).unwrap();

                    for log_cpus in log_d..min(log_d + 1, 3) {
                        parallel_fft::<E::Fr, E::Fr>(
                            &mut v1,
                            domain.group_gen,
                            log_d,
                            log_cpus,
                            serial_radix2_fft::<E::Fr, E::Fr>,
                        );
                        serial_radix2_fft::<E::Fr, E::Fr>(&mut v2, domain.group_gen, log_d);

                        assert_eq!(v1, v2);
                    }
                }
            }
        }

        let rng = &mut test_rng();

        test_consistency::<Bls12_381, _>(rng, 10);
    }
}

#[cfg(test)]
mod tests {
    use crate::{EvaluationDomain, MixedRadixEvaluationDomain};
    use algebra::mnt6_753::Fr;
    use algebra_core::{test_rng, Field, Zero};
    use rand::Rng;

    #[test]
    fn vanishing_polynomial_evaluation() {
        let rng = &mut test_rng();
        for coeffs in 0..17 {
            let domain = MixedRadixEvaluationDomain::<Fr>::new(coeffs).unwrap();
            let z = domain.vanishing_polynomial();
            for _ in 0..100 {
                let point = rng.gen();
                assert_eq!(
                    z.evaluate(point),
                    domain.evaluate_vanishing_polynomial(point)
                )
            }
        }
    }

    #[test]
    fn vanishing_polynomial_vanishes_on_domain() {
        for coeffs in 0..1000 {
            let domain = MixedRadixEvaluationDomain::<Fr>::new(coeffs).unwrap();
            let z = domain.vanishing_polynomial();
            for point in domain.elements() {
                assert!(z.evaluate(point).is_zero())
            }
        }
    }

    #[test]
    fn size_of_elements() {
        for coeffs in 1..17 {
            let size = 1 << coeffs;
            let domain = MixedRadixEvaluationDomain::<Fr>::new(size).unwrap();
            let domain_size = domain.size();
            assert_eq!(domain_size, domain.elements().count());
        }
    }

    #[test]
    fn elements_contents() {
        for coeffs in 1..17 {
            let size = 1 << coeffs;
            let domain = MixedRadixEvaluationDomain::<Fr>::new(size).unwrap();
            for (i, element) in domain.elements().enumerate() {
                assert_eq!(element, domain.group_gen.pow([i as u64]));
            }
        }
    }

    #[test]
    #[cfg(feature = "parallel")]
    fn parallel_fft_consistency() {
        use super::serial_mixed_radix_fft;
        use crate::domain::utils::parallel_fft;
        use crate::Vec;
        use algebra::mnt6_753::MNT6_753;
        use algebra_core::{test_rng, PairingEngine, UniformRand};
        use core::cmp::min;

        fn test_consistency<E: PairingEngine, R: Rng>(rng: &mut R, max_coeffs: u32) {
            for _ in 0..5 {
                for log_d in 0..max_coeffs {
                    let d = 1 << log_d;

                    let mut v1 = (0..d).map(|_| E::Fr::rand(rng)).collect::<Vec<_>>();
                    let mut v2 = v1.clone();

                    let domain = MixedRadixEvaluationDomain::new(v1.len()).unwrap();

                    for log_cpus in log_d..min(log_d + 1, 3) {
                        parallel_fft::<E::Fr, E::Fr>(
                            &mut v1,
                            domain.group_gen,
                            log_d,
                            log_cpus,
                            serial_mixed_radix_fft::<E::Fr, E::Fr>,
                        );
                        serial_mixed_radix_fft::<E::Fr, E::Fr>(&mut v2, domain.group_gen, log_d);

                        assert_eq!(v1, v2);
                    }
                }
            }
        }

        let rng = &mut test_rng();

        test_consistency::<MNT6_753, _>(rng, 16);
    }
}

// Test multiplying various (low degree) polynomials together and
// comparing with naive evaluations.
#[test]
fn fft_composition() {
    fn test_fft_composition<
        F: PrimeField,
        T: DomainCoeff<F> + UniformRand + core::fmt::Debug + Eq,
        R: rand::Rng,
        D: EvaluationDomain<F>,
    >(
        rng: &mut R,
        max_coeffs: usize,
    ) {
        for coeffs in 0..max_coeffs {
            let coeffs = 1 << coeffs;

            let domain = D::new(coeffs).unwrap();

            let mut v = vec![];
            for _ in 0..coeffs {
                v.push(T::rand(rng));
            }
            // Fill up with zeros.
            v.resize(domain.size(), T::zero());
            let mut v2 = v.clone();

            domain.ifft_in_place(&mut v2);
            domain.fft_in_place(&mut v2);
            assert_eq!(v, v2, "ifft(fft(.)) != iden");

            domain.fft_in_place(&mut v2);
            domain.ifft_in_place(&mut v2);
            assert_eq!(v, v2, "fft(ifft(.)) != iden");

            domain.coset_ifft_in_place(&mut v2);
            domain.coset_fft_in_place(&mut v2);
            assert_eq!(v, v2, "coset_fft(coset_ifft(.)) != iden");

            domain.coset_fft_in_place(&mut v2);
            domain.coset_ifft_in_place(&mut v2);
            assert_eq!(v, v2, "coset_ifft(coset_fft(.)) != iden");
        }
    }

    let rng = &mut test_rng();

    test_fft_composition::<Fr, Fr, _, GeneralEvaluationDomain<Fr>>(rng, 10);
    test_fft_composition::<Fr, G1Projective, _, GeneralEvaluationDomain<Fr>>(rng, 10);
    // This will result in a mixed-radix domain being used.
    test_fft_composition::<MNT6Fr, MNT6Fr, _, MixedRadixEvaluationDomain<MNT6Fr>>(rng, 17);
    test_fft_composition::<MNT6Fr, MNT6G1Projective, _, MixedRadixEvaluationDomain<MNT6Fr>>(rng, 5);
}
