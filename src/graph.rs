#[derive(Clone, Debug)]
enum Dfs<Idx> {
    Pre(Idx),
    Post(Idx),
}

pub trait Graph {
    type Idx: Clone;
    type Cost;

    fn len(&self) -> usize;
    fn edges(&self, i: Self::Idx) -> Vec<(Self::Idx, Self::Cost)>;
    fn idx_to_usize(&self, i: Self::Idx) -> usize;

    fn dfs<D: Clone, F, G>(&self, init: (Self::Idx, D), mut pre_order: F, mut post_order: G)
    where
        F: FnMut(&Self::Idx, &Self::Idx, &Self::Cost, &D) -> Option<D>,
        G: FnMut(&Self::Idx, &D),
    {
        let mut done = vec![false; self.len()];
        let mut stk = vec![
            (Dfs::Post(init.0.clone()), init.1.clone()),
            (Dfs::Pre(init.0.clone()), init.1.clone()),
        ];
        while !stk.is_empty() {
            let (idx, data) = stk.pop().unwrap();
            match idx {
                Dfs::Pre(idx) => {
                    done[self.idx_to_usize(idx.clone())] = true;
                    for (nidx, cost) in self.edges(idx.clone()) {
                        if done[self.idx_to_usize(nidx.clone())] {
                            continue;
                        }
                        let ndata = pre_order(&idx, &nidx, &cost, &data);
                        if let Some(ndata) = ndata {
                            stk.push((Dfs::Post(nidx.clone()), ndata.clone()));
                            stk.push((Dfs::Pre(nidx.clone()), ndata.clone()));
                        }
                    }
                }
                Dfs::Post(idx) => {
                    post_order(&idx, &data);
                }
            }
        }
    }
}
#[derive(Debug)]
pub struct UnweightedGraph {
    pub n: usize,
    pub g: Vec<Vec<usize>>,
}

impl Graph for UnweightedGraph {
    type Idx = usize;
    type Cost = ();

    fn len(&self) -> usize {
        self.n
    }

    fn edges(&self, i: Self::Idx) -> Vec<(Self::Idx, Self::Cost)> {
        self.g[i].iter().map(|&i| (i, ())).collect()
    }

    fn idx_to_usize(&self, i: Self::Idx) -> usize {
        i
    }
}
#[derive(Debug)]
pub struct WeightedGraph<Cost> {
    pub n: usize,
    pub g: Vec<Vec<(usize, Cost)>>,
}

impl<Cost: Clone> Graph for WeightedGraph<Cost> {
    type Idx = usize;
    type Cost = Cost;

    fn len(&self) -> usize {
        self.n
    }

    fn edges(&self, i: Self::Idx) -> Vec<(Self::Idx, Self::Cost)> {
        self.g[i].clone()
    }

    fn idx_to_usize(&self, i: Self::Idx) -> usize {
        i
    }
}
