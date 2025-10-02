use algebra::impls::{CSR2D, SquareCSR2D};
use functional_properties::prelude::ScalarSimilarity;
use graph::{
    prelude::{GenericGraph, WuPalmer},
    traits::{MonopartiteGraph, MonoplexMonopartiteGraph},
};
use num_traits::{ConstOne, ConstZero};
use numeric_common_traits::prelude::{IntoUsize, PositiveInteger, TryFromUsize};

use crate::mock_procedure::MockProcedure;

pub type MockDag<N> = GenericGraph<N, SquareCSR2D<CSR2D<u64, N, N>>>;

pub struct MockProcedureWuPalmer<T, AM>
where
    T: PositiveInteger + IntoUsize + TryFromUsize,
    AM: PositiveInteger + IntoUsize + TryFromUsize,
{
    taxonomy_dag: MockDag<T>,
    asset_model_dag: MockDag<AM>,
}

impl<T, AM> MockProcedureWuPalmer<T, AM>
where
    T: PositiveInteger + IntoUsize + TryFromUsize,
    AM: PositiveInteger + IntoUsize + TryFromUsize,
{
    pub fn new(taxonomy_dag: MockDag<T>, asset_model_dag: MockDag<AM>) -> Self {
        Self {
            taxonomy_dag,
            asset_model_dag,
        }
    }
}

impl<T, AM> ScalarSimilarity<MockProcedure<T, AM>, MockProcedure<T, AM>>
    for MockProcedureWuPalmer<T, AM>
where
    MockDag<T>: WuPalmer + MonopartiteGraph<NodeId = T>,
    MockDag<AM>: WuPalmer + MonopartiteGraph<NodeId = AM>,
    T: PositiveInteger + IntoUsize + TryFromUsize,
    AM: PositiveInteger + IntoUsize + TryFromUsize,
{
    type Similarity = f64;
    fn similarity(
        &self,
        left: &MockProcedure<T, AM>,
        right: &MockProcedure<T, AM>,
    ) -> Self::Similarity {
        let taxonomy_wu_palmer = self.taxonomy_dag.wu_palmer().unwrap();
        let asset_model_wu_palmer = self.asset_model_dag.wu_palmer().unwrap();
        let mut total_similarity = 0.0;
        let mut number_of_similar_elements = 0.0;
        for (l,r) in left.taxonomies().iter().zip(right.taxonomies()) {
            total_similarity += taxonomy_wu_palmer.similarity(l,r);
            number_of_similar_elements += 1.0;
        }
        for (l,r) in left.asset_models().iter().zip(right.asset_models()) {
            total_similarity += asset_model_wu_palmer.similarity(l,r);
            number_of_similar_elements += 1.0;
        }
        total_similarity / number_of_similar_elements
    }
}
