//! Benchmarking setup for pallet-smart-agreement
use crate::Pallet as SmartAgreement;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::sp_runtime::traits::Hash;
use frame_system::RawOrigin;
