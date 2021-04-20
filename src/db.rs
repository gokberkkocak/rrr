// use mysql::prelude::*;
// use mysql::*;
use mysql_async::{prelude::*, Conn, Pool};
use std::thread;
use std::time::Duration;

use crate::json::{ExperimentSingle, ExperimentStore, SolveInformation};
use crate::utils::Mode;

pub struct DBConfig<'a> {
    host: &'a str,
    username: &'a str,
    password: &'a str,
}

impl<'a> DBConfig<'a> {
    fn new(host: &'a str, username: &'a str, password: &'a str) -> DBConfig<'a> {
        DBConfig {
            host,
            username,
            password,
        }
    }

    pub fn from_config_file(file_contents: &'a str) -> Self {
        let mut host = None;
        let mut username = None;
        let mut password = None;
        for line in file_contents.lines() {
            if line.starts_with('#') {
                continue;
            } else if line.contains('=') {
                let split: Vec<&str> = line.split('=').collect();
                assert_eq!(split.len(), 2);
                if split[0] == "host" {
                    host = Some(split[1].trim_end());
                } else if split[0] == "user" {
                    username = Some(split[1].trim_end())
                } else if split[0] == "password" {
                    password = Some(split[1].trim_end())
                }
            }
        }
        Self::new(
            host.expect("host not found"),
            username.expect("username not found"),
            password.expect("password not found"),
        )
    }
}

pub async fn init_db_connection<'a>(db: DBConfig<'a>) -> Conn {
    loop {
        match try_init_db_connection(&db).await {
            Ok(conn) => return conn,
            Err(e) => println!("DB error: {}. Will try again soon.", e),
        }
        thread::sleep(Duration::from_millis(4000));
    }
}

async fn try_init_db_connection<'a>(db: &'a DBConfig<'a>) -> Result<Conn, mysql_async::Error> {
    let url = format!(
        "mysql://{}:{}@{}/{}_experiments",
        db.username, db.password, db.host, db.username
    );
    let pool = Pool::new(url);
    let conn = pool.get_conn();
    conn.await
}

pub async fn create_table(conn: &mut Conn) {
    // Let's create a table for payments.
    conn.query_drop(
        r"CREATE TABLE experiments (
            id SERIAL PRIMARY KEY,
            exp_id VARCHAR(255) NOT NULL,
            config_id VARCHAR(255) NOT NULL,
            result_type VARCHAR(255) NOT NULL,
            measured_time DOUBLE NOT NULL,
            nb_solutions INTEGER,
            machine_info VARCHAR(255) NOT NULL,
            memory_limit INTEGER NOT NULL,
            seed DOUBLE
          )
        ",
    )
    .await
    .unwrap();
}

pub async fn drop_table(conn: &mut Conn) {
    conn.query_drop(
        r"DROP TABLE IF EXISTS experiments
        ",
    )
    .await
    .unwrap();
}

pub struct DBRow {
    exp_id: String,
    config_id: String,
    result_type: String,
    measured_time: f64,
    nb_solutions: Option<u64>,
    machine_info: String,
    memory_limit: u64,
    seed: Option<f64>,
}

#[allow(clippy::too_many_arguments)]
impl DBRow {
    fn new(
        exp_id: String,
        config_id: String,
        result_type: String,
        measured_time: f64,
        nb_solutions: Option<u64>,
        machine_info: String,
        memory_limit: u64,
        seed: Option<f64>,
    ) -> Self {
        DBRow {
            exp_id,
            config_id,
            result_type,
            measured_time,
            nb_solutions,
            machine_info,
            memory_limit,
            seed,
        }
    }

    fn from_solve(exp_id: &str, config_id: &str, solve_information: &SolveInformation) -> Self {
        let measured_time;
        let mut nb: Option<u64> = None;
        let r_seed: Option<f64>;
        let r_machine_info;
        let result_type;
        let r_memory_limit;
        match solve_information {
            SolveInformation::Success {
                total_sr_time,
                nb_solutions,
                seed,
                machine_info,
                memory_limit,
                ..
            } => {
                nb = Some(*nb_solutions);
                measured_time = *total_sr_time;
                r_seed = *seed;
                r_machine_info = machine_info.as_str();
                r_memory_limit = *memory_limit;
                result_type = "SUCCESS"
            }
            SolveInformation::Timeout {
                seed,
                memory_limit,
                time_limit,
                machine_info,
            } => {
                r_seed = *seed;
                measured_time = *time_limit as f64;
                r_machine_info = machine_info.as_str();
                r_memory_limit = *memory_limit;
                result_type = "TIMEOUT"
            }
            SolveInformation::Memout {
                crash_time,
                seed,
                memory_limit,
                machine_info,
                ..
            } => {
                r_seed = *seed;
                measured_time = *crash_time;
                r_machine_info = machine_info.as_str();
                r_memory_limit = *memory_limit;
                result_type = "MEMOUT"
            }
            SolveInformation::Crash {
                crash_time,
                seed,
                memory_limit,
                machine_info,
                ..
            } => {
                r_seed = *seed;
                measured_time = *crash_time;
                r_machine_info = machine_info.as_str();
                r_memory_limit = *memory_limit;
                result_type = "CRASHED"
            }
            SolveInformation::Doubted {
                total_sr_time,
                nb_solutions,
                seed,
                machine_info,
                memory_limit,
                ..
            } => {
                nb = Some(*nb_solutions);
                measured_time = *total_sr_time;
                r_seed = *seed;
                r_machine_info = machine_info.as_str();
                r_memory_limit = *memory_limit;
                result_type = "DOUBTED"
            }
        }
        DBRow::new(
            exp_id.to_string(),
            config_id.to_string(),
            result_type.to_string(),
            measured_time,
            nb,
            r_machine_info.to_string(),
            r_memory_limit,
            r_seed,
        )
    }
}

pub async fn populate_db(conn: &mut Conn, store: ExperimentStore) {
    let mut rows: Vec<DBRow> = vec![];
    for (id, experiment) in store.experiments.into_iter() {
        for (c_id, config) in experiment.configs.into_iter() {
            for solve in config.solve_information.iter() {
                rows.push(DBRow::from_solve(&id, &c_id, solve));
            }
        }
    }
    conn.exec_batch(
        r"INSERT INTO experiments (exp_id, config_id, result_type, measured_time, nb_solutions, machine_info, memory_limit, seed)
          VALUES (:exp_id, :config_id, :result_type, :measured_time, :nb_solutions, :machine_info, :memory_limit, :seed)",
        rows.into_iter().map(|r| params! {
            "exp_id" => r.exp_id,
            "config_id" => r.config_id,
            "result_type" => r.result_type,
            "measured_time" => r.measured_time,
            "nb_solutions" => r.nb_solutions,
            "machine_info" => r.machine_info,
            "memory_limit" => r.memory_limit,
            "seed" => r.seed,
        })
    ).await.unwrap();
}

pub async fn check_mode(conn: &mut Conn, exp_id: &str, config_id: &str, mode: Mode, exact: bool) {
    match mode {
        Mode::NbSolutions => check_sol(conn, exp_id).await,
        _ => {
            if exact {
                check_exact_time(conn, exp_id, config_id).await;
            } else {
                check_best_time(conn, exp_id).await;
            }
        }
    }
}

async fn check_sol(conn: &mut Conn, exp_id: &str) {
    let res = get_experiment_successful_results(conn, exp_id).await;
    let mut nb_solutions: Option<u64> = None;
    let mut support: u16 = 0;
    for row in res.into_iter() {
        if let Some(nb) = row.nb_solutions {
            match nb_solutions {
                Some(n) => {
                    assert_eq!(n, nb);
                    support += 1;
                }
                None => {
                    nb_solutions = Some(nb);
                    support = 1;
                }
            }
        }
    }
    match nb_solutions {
        Some(nb) => println!("DB_NB_SOLS {} with support {}", nb, support),
        None => println!("EMPTY"),
    }
}

async fn check_best_time(conn: &mut Conn, exp_id: &str) {
    let res;
    if exp_id.contains("rel_sub") {
        let exps = get_rsd_extra_exp_ids(exp_id);
        // cannot use streams here since conn is &mut
        let mut in_res = vec![];
        for e in exps {
            in_res.extend(get_experiment_results(conn, &e).await);
        }
        res = in_res;
    } else {
        res = get_experiment_results(conn, exp_id).await;
    }
    check_time(res);
}

fn get_rsd_extra_exp_ids(exp_id: &str) -> Vec<String> {
    // for rsd different exp ids we need to add this extra exps for best
    let mut naked_str = exp_id.replace("rel_sub", "");
    naked_str = naked_str.replace("_complete", "");
    naked_str = naked_str.replace("_par_neg", "");
    naked_str = naked_str.replace("_par_pos", "");
    let mut vec = vec![];
    for i in 0..4 {
        let mut base = String::from("rel_sub");
        match i {
            1 => base.push_str("_complete"),
            2 => base.push_str("_par_neg"),
            3 => base.push_str("_par_pos"),
            _ => {}
        }
        base.push_str(&naked_str);
        vec.push(base);
    }
    vec
}

async fn check_exact_time(conn: &mut Conn, exp_id: &str, config_id: &str) {
    let res = get_experiment_config_results(conn, exp_id, config_id).await;
    check_time(res);
}

// I don't like this function. Want to rewrite it.
fn check_time(res: Vec<DBRow>) {
    let mut res_time: (Option<f64>, Option<&str>) = (None, None);
    for row in res.iter() {
        if let Some(result_type) = res_time.1 {
            if row.result_type == "SUCCESS" {
                if result_type == "SUCCESS" {
                    match res_time.0 {
                        Some(t) => {
                            if row.measured_time < t {
                                res_time = (Some(row.measured_time), Some("SUCCESS"))
                            }
                        }
                        None => res_time = (Some(row.measured_time), Some("SUCCESS")),
                    }
                } else {
                    res_time = (Some(row.measured_time), Some("SUCCESS"));
                }
            } else if row.result_type == "TIMEOUT" && result_type == "TIMEOUT" {
                match res_time.0 {
                    Some(t) => {
                        if row.measured_time > t {
                            res_time = (Some(row.measured_time), Some("TIMEOUT"))
                        }
                    }
                    None => res_time = (Some(row.measured_time), Some("TIMEOUT")),
                }
            } else if row.result_type == "MEMOUT" && result_type == "MEMOUT" {
                match res_time.0 {
                    Some(t) => {
                        if row.measured_time > t {
                            res_time = (Some(row.measured_time), Some("MEMOUT"))
                        }
                    }
                    None => res_time = (Some(row.measured_time), Some("MEMOUT")),
                }
            }
        } else {
            res_time = (Some(row.measured_time), Some(row.result_type.as_str()));
        }
    }
    if let Some(res_str) = res_time.1 {
        if res_str == "SUCCESS" {
            match res_time.0 {
                Some(t) => println!("DB_MIN_SR {}", t),
                None => println!("EMPTY"),
            }
        } else if res_str == "TIMEOUT" {
            match res_time.0 {
                Some(t) => println!("DB_MAX_TIMEOUT {}", t),
                None => println!("EMPTY"),
            }
        } else if res_str == "MEMOUT" {
            match res_time.0 {
                Some(t) => println!("DB_MAX_MEMOUT_TIME {}", t),
                None => println!("EMPTY"),
            }
        }
    } else {
        println!("EMPTY");
    }
}

pub async fn check_nb_successful(conn: &mut Conn, exp_id: &str, config_id: &str) -> usize {
    let res = get_experiment_config_successful_results(conn, exp_id, config_id).await;
    let mut seed_vec = vec![];
    for row in res.iter() {
        if let Some(seed) = row.seed {
            if !seed_vec.contains(&seed) {
                seed_vec.push(seed);
            }
        }
    }
    seed_vec.len()
}

async fn get_experiment_results(conn: &mut Conn, exp_id: &str) -> Vec<DBRow> {
    conn.exec_map(
        "SELECT config_id, result_type, measured_time, nb_solutions, memory_limit, machine_info, seed from experiments WHERE exp_id = ?",
        (exp_id, ),
        |(config_id, result_type, measured_time, nb_solutions, memory_limit, machine_info, seed)| {
            DBRow::new(
                exp_id.to_string(),
                config_id,
                result_type,
                measured_time,
                nb_solutions,
                machine_info,
                memory_limit,
                seed,
            )
        },
    ).await.unwrap()
}

async fn get_experiment_successful_results(conn: &mut Conn, exp_id: &str) -> Vec<DBRow> {
    conn.exec_map(
        "SELECT config_id, measured_time, nb_solutions, memory_limit, machine_info, seed from experiments WHERE exp_id = ? and result_type = ?",
        (exp_id, "SUCCESS"),
        |(config_id,  measured_time, nb_solutions, memory_limit, machine_info, seed)| {
            DBRow::new(
                exp_id.to_string(),
                config_id,
                "SUCCESS".to_string(),
                measured_time,
                nb_solutions,
                machine_info,
                memory_limit,
                seed,
            )
        },
    ).await.unwrap()
}

async fn get_experiment_config_results(
    conn: &mut Conn,
    exp_id: &str,
    config_id: &str,
) -> Vec<DBRow> {
    conn.exec_map(
        "SELECT result_type, measured_time, nb_solutions, memory_limit, machine_info, seed from experiments WHERE exp_id = ? and config_id = ?",
        (exp_id, config_id),
        |(result_type, measured_time, nb_solutions, memory_limit, machine_info, seed)| {
            DBRow::new(
                exp_id.to_string(),
                config_id.to_string(),
                result_type,
                measured_time,
                nb_solutions,
                machine_info,
                memory_limit,
                seed,
            )
        },
    ).await.unwrap()
}

async fn get_experiment_config_successful_results(
    conn: &mut Conn,
    exp_id: &str,
    config_id: &str,
) -> Vec<DBRow> {
    conn.exec_map(
        "SELECT measured_time, nb_solutions, memory_limit, machine_info, seed from experiments WHERE exp_id = ? and config_id = ? and result_type = ?",
        (exp_id, config_id, "SUCCESS"),
        |(measured_time, nb_solutions, memory_limit, machine_info, seed)| {
            DBRow::new(
                exp_id.to_string(),
                config_id.to_string(),
                "SUCCESS".to_string(),
                measured_time,
                nb_solutions,
                machine_info,
                memory_limit,
                seed,
            )
        },
    ).await.unwrap()
}

pub async fn commit_to_db(conn: &mut Conn, exp: ExperimentSingle) {
    let exp_id = exp.exp_id;
    let config_id = exp.config_id;
    assert_eq!(exp.config.solve_information.len(), 1);
    // I can consume the sol_info vec and move instead of borrow since sol info is not required later.
    let r = DBRow::from_solve(
        &exp_id,
        &config_id,
        exp.config.solve_information.iter().next().unwrap(),
    );
    conn.exec_drop(
        r"INSERT INTO experiments (exp_id, config_id, result_type, measured_time, nb_solutions, machine_info, memory_limit, seed)
          VALUES (:exp_id, :config_id, :result_type, :measured_time, :nb_solutions, :machine_info, :memory_limit, :seed)",
         params! {
            "exp_id" => r.exp_id,
            "config_id" => r.config_id,
            "result_type" => r.result_type,
            "measured_time" => r.measured_time,
            "nb_solutions" => r.nb_solutions,
            "machine_info" => r.machine_info,
            "memory_limit" => r.memory_limit,
            "seed" => r.seed,
        }
    ).await.unwrap();
}
