use crate::utils::{FSort, Mean};
use serde::Serialize;
use std::collections::HashMap;

use crate::json::{ExperimentStore, LevelInformation, SolveInformation};
#[derive(Serialize)]
pub struct PlotExperimentStoreView<'a> {
    store: HashMap<&'a str, HashMap<&'a str, Box<PlotConfigView<'a>>>>,
}
#[derive(Serialize)]
pub struct PlotConfigView<'a> {
    nb_data_points: u8,
    total_solver_time_mean: Option<f64>,
    total_solver_time_best: Option<f64>,
    total_sr_time_mean: Option<f64>,
    total_sr_time_best: Option<f64>,
    total_nodes_mean: Option<f64>,
    total_nodes_best: Option<f64>,
    nb_solutions: Option<u64>,
    nb_levels: Option<u16>,
    levels_best_solver_time: &'a Option<HashMap<String, f64>>,
    levels_best_sat_clauses: &'a Option<HashMap<String, u64>>,
    levels_best_satv: &'a Option<HashMap<String, u64>>,
    levels_best_sat_learnt_clauses: &'a Option<HashMap<String, u64>>,
    levels_best_nodes: &'a Option<HashMap<String, u64>>,
    levels_best_nb_sols: &'a Option<HashMap<String, u64>>,
}

impl<'a> PlotConfigView<'a> {
    fn new_from_temp(t: &TempConfigView<'a>) -> Self {
        let mut levels_best_solver_time: &Option<HashMap<String, f64>> = &None;
        let mut levels_best_sat_clauses: &Option<HashMap<String, u64>> = &None;
        let mut levels_best_satv: &Option<HashMap<String, u64>> = &None;
        let mut levels_best_sat_learnt_clauses: &Option<HashMap<String, u64>> = &None;
        let mut levels_best_nodes: &Option<HashMap<String, u64>> = &None;
        let mut levels_best_nb_sols: &Option<HashMap<String, u64>> = &None;
        if let Some(level_info) = t.best_level_info {
            levels_best_solver_time = &level_info.solver_time;
            levels_best_sat_clauses = &level_info.nb_clauses;
            levels_best_satv = &level_info.nb_vars;
            levels_best_sat_learnt_clauses = &level_info.nb_learnt_clauses;
            levels_best_nodes = &level_info.nodes;
            levels_best_nb_sols = &level_info.cumulative_nb_solutions;
        }

        if !t.sr_times.is_empty() {
            PlotConfigView {
                nb_data_points: t.nb_data_points,
                total_solver_time_mean: Some(t.solver_times.mean()),
                total_solver_time_best: Some(t.solver_times[0]),
                total_sr_time_mean: Some(t.sr_times.mean()),
                total_sr_time_best: Some(t.sr_times[0]),
                total_nodes_mean: Some(t.solver_nodes.mean()),
                total_nodes_best: Some(t.solver_nodes[0]),
                nb_solutions: Some(t.nb_solutions),
                nb_levels: Some(t.nb_levels),
                levels_best_solver_time,
                levels_best_sat_clauses,
                levels_best_satv,
                levels_best_sat_learnt_clauses,
                levels_best_nodes,
                levels_best_nb_sols,
            }
        } else {
            PlotConfigView {
                nb_data_points: t.nb_data_points,
                total_solver_time_mean: None,
                total_solver_time_best: None,
                total_sr_time_mean: None,
                total_sr_time_best: None,
                total_nodes_mean: None,
                total_nodes_best: None,
                nb_solutions: None,
                nb_levels: None,
                levels_best_solver_time,
                levels_best_sat_clauses,
                levels_best_satv,
                levels_best_sat_learnt_clauses,
                levels_best_nodes,
                levels_best_nb_sols,
            }
        }
    }
}

struct TempConfigView<'a> {
    nb_data_points: u8,
    solver_times: Vec<f64>,
    sr_times: Vec<f64>,
    solver_nodes: Vec<f64>,
    nb_levels: u16,
    nb_solutions: u64,
    best_level_info: Option<&'a LevelInformation>,
}

impl<'a> TempConfigView<'a> {
    fn default() -> Self {
        TempConfigView {
            nb_data_points: 0,
            solver_times: vec![],
            sr_times: vec![],
            solver_nodes: vec![],
            nb_levels: 0,
            nb_solutions: 0,
            best_level_info: None,
        }
    }
    fn adjust(
        &mut self,
        total_sr_time: f64,
        total_solver_time: f64,
        nb_solutions: u64,
        total_nodes: f64,
        level_info: &'a LevelInformation,
    ) {
        self.nb_data_points += 1;
        if self.nb_solutions == 0 {
            self.nb_solutions = nb_solutions;
        }
        self.solver_times.push(total_solver_time);
        self.sr_times.push(total_sr_time);
        self.solver_nodes.push(total_nodes);
        // sort all
        self.sr_times.f_sort();
        self.solver_times.f_sort();
        self.solver_nodes.f_sort();
        if self.sr_times[0] == total_sr_time {
            self.best_level_info = Some(level_info);
            if let Some(s) = &level_info.solver_time {
                // dangerous but I know that my levels are short.
                self.nb_levels = s.keys().len() as u16;
            }
        }
    }
}

pub fn convert_store_for_plot(store: &ExperimentStore) -> PlotExperimentStoreView {
    let mut plot_store = HashMap::new();
    for (exp_key, exps) in &store.experiments {
        let mut inner_configs = HashMap::new();
        for (c_key, configs) in &exps.configs {
            let mut t = TempConfigView::default();
            let solves = &configs.solve_information;
            for i in solves {
                if let SolveInformation::Success {
                    total_sr_time,
                    total_solver_time,
                    nb_solutions,
                    total_nodes,
                    level_info,
                    ..
                } = i
                {
                    t.adjust(
                        *total_sr_time,
                        *total_solver_time,
                        *nb_solutions,
                        // some smt doesnt send node count. get 0.
                        total_nodes.unwrap_or(0) as f64,
                        level_info,
                    );
                }
            }
            let plot_config = PlotConfigView::new_from_temp(&t);
            inner_configs.insert(&c_key[..], Box::new(plot_config));
        }
        plot_store.insert(&exp_key[..], inner_configs);
    }
    PlotExperimentStoreView { store: plot_store }
}
