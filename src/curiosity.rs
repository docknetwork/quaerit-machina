use crate::store::Store;
use oxigraph::model as om;
use oxigraph::sparql::{algebra::Query, EvaluationError, QueryResults};

pub struct Curiosity {
    /// queries in this list must all be select statements
    curiosity: Vec<Query>,
}

impl Curiosity {
    pub fn create(curiosity: Vec<Query>) -> Result<Self, ()> {
        if !curiosity.iter().all(is_select) {
            return Err(());
        }
        Ok(Self { curiosity })
    }

    pub fn curious(
        &self,
        store: &impl Store,
        mut interesting: impl FnMut(&om::Term),
    ) -> Result<(), EvaluationError> {
        for cur in &self.curiosity {
            let q = store.query(cur.clone())?;
            match q {
                QueryResults::Solutions(solutions) => {
                    for s in solutions {
                        for (_name, term) in s?.iter() {
                            interesting(term);
                        }
                    }
                }
                QueryResults::Boolean(_) | QueryResults::Graph(_) => {
                    panic!("Expected SELECT statements only.");
                }
            }
        }
        Ok(())
    }
}

fn is_select(q: &Query) -> bool {
    match q {
        Query::Select { .. } => true,
        _ => false,
    }
}

