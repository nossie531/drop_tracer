use crate::for_test::*;
use drop_tracer::prelude::*;
use rand::prelude::*;
use rand_pcg::Pcg32;
use std::collections::BTreeMap;

pub(crate) struct TracerBuilder {
    target: Option<DropTracer>,
    rng: Pcg32,
    living_vals: Vec<TraceVal<ts::Val>>,
    expected_logs: Vec<TraceLog>,
    expected_states: BTreeMap<TraceTag, bool>,
}

impl TracerBuilder {
    pub fn new() -> Self {
        Self {
            target: Some(DropTracer::new()),
            rng: Pcg32::seed_from_u64(0),
            living_vals: Vec::new(),
            expected_logs: Vec::new(),
            expected_states: BTreeMap::new(),
        }
    }

    pub fn new_normal() -> Self {
        Self::new().with_defaults()
    }

    pub fn with_defaults(self) -> Self {
        assert!(self.is_initial_state());
        self.with_log_simple()
            .with_log_drop()
            .with_log_cloned()
            .with_log_cloned_drop()
            .with_log_origin_drop()
            .with_log_cloned_siblings()
            .with_log_cloned_seriese()
    }

    pub fn with_log_simple(mut self) -> Self {
        assert!(self.is_now_setting());
        let val = self.make_val();
        self.save_living_vals([val]);
        self
    }

    pub fn with_log_drop(mut self) -> Self {
        assert!(self.is_now_setting());
        let val = self.make_val();
        self.add_drop_vals([val]);
        self
    }

    pub fn with_log_cloned(mut self) -> Self {
        assert!(self.is_now_setting());
        let val = self.make_val();
        let clone = self.clone_val(&val);
        self.save_living_vals([val, clone]);
        self
    }

    pub fn with_log_cloned_drop(mut self) -> Self {
        assert!(self.is_now_setting());
        let val = self.make_val();
        let clone = self.clone_val(&val);
        self.add_drop_vals([clone]);
        self.save_living_vals([val]);
        self
    }

    pub fn with_log_origin_drop(mut self) -> Self {
        assert!(self.is_now_setting());
        let val = self.make_val();
        let clone = self.clone_val(&val);
        self.add_drop_vals([val]);
        self.save_living_vals([clone]);
        self
    }

    pub fn with_log_cloned_siblings(mut self) -> Self {
        assert!(self.is_now_setting());
        let val = self.make_val();
        let sibs = &mut self.clone_branches(&val, 3);
        self.add_drop_vals([sibs.remove(&1).unwrap()]);
        self.save_living_vals([val]);
        self.save_living_vals(tu::extract_vals(sibs));
        self
    }

    pub fn with_log_cloned_seriese(mut self) -> Self {
        assert!(self.is_now_setting());
        let val = self.make_val();
        let fst_gens = &mut self.clone_branches(&val, 2);
        let snd_gens = &mut self.clone_branches_x2(fst_gens, 3);
        self.add_drop_vals([fst_gens.remove(&1).unwrap()]);
        self.add_drop_vals([snd_gens.get_mut(&0).unwrap().remove(&1).unwrap()]);
        self.save_living_vals([val]);
        self.save_living_vals(tu::extract_vals(fst_gens));
        self.save_living_vals(tu::extract_flat_vals(snd_gens));
        self
    }

    pub fn is_now_setting(&self) -> bool {
        self.target.is_some()
    }

    pub fn is_initial_state(&self) -> bool {
        self.is_now_setting() && self.expected_states.len() == 0
    }

    pub fn living_count(&self) -> usize {
        self.expected_states.values().filter(|&x| *x).count()
    }

    pub fn expected_logs(&self) -> &Vec<TraceLog> {
        &self.expected_logs
    }

    pub fn expected_states(&self) -> &BTreeMap<TraceTag, bool> {
        &self.expected_states
    }

    pub fn take(&mut self) -> DropTracer {
        self.target.take().unwrap()
    }

    fn make_val(&mut self) -> TraceVal<ts::Val> {
        let ret = self.target.as_ref().unwrap().trace(self.rng.random());
        self.add_expected_changes(&ret, true);
        ret
    }

    fn clone_val(&mut self, val: &TraceVal<ts::Val>) -> TraceVal<ts::Val> {
        let ret = val.clone();
        self.add_expected_changes(&ret, true);
        ret
    }

    fn clone_branches(
        &mut self,
        val: &TraceVal<ts::Val>,
        n: usize,
    ) -> BTreeMap<usize, TraceVal<ts::Val>> {
        (0..n).map(|i| (i, self.clone_val(val))).collect()
    }

    fn clone_branches_x2(
        &mut self,
        map: &BTreeMap<usize, TraceVal<ts::Val>>,
        n: usize,
    ) -> BTreeMap<usize, BTreeMap<usize, TraceVal<ts::Val>>> {
        map.iter()
            .map(|(k, v)| (*k, self.clone_branches(v, n)))
            .collect()
    }

    fn save_living_vals(&mut self, vals: impl IntoIterator<Item = TraceVal<ts::Val>>) {
        self.living_vals.extend(vals);
    }

    fn add_drop_vals(&mut self, vals: impl IntoIterator<Item = TraceVal<ts::Val>>) {
        for val in vals {
            self.add_expected_changes(&val, false);
            self.expected_states.insert(TraceVal::tag(&val), false);
        }
    }

    fn add_expected_changes(&mut self, val: &TraceVal<ts::Val>, alloc: bool) {
        self.expected_logs.push(TraceLog::new_by_val(val, alloc));
        self.expected_states.insert(TraceVal::tag(val), alloc);
    }
}
