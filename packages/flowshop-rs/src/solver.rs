use grb::prelude::*;

/// Solve the exact Flow-Shop scheduling problem via Gurobi.
///
/// # Arguments
/// * `times`      – flat row-major list: job j on machine k is times[j * n_machines + k].
/// * `n_jobs`     – number of jobs.
/// * `n_machines` – number of machines.
///
/// # Returns
/// The optimal makespan ⌈C_max⌉ as a `usize`.
pub fn solve_flow_shop_exact(
    times: &[usize],
    n_jobs: usize,
    n_machines: usize,
) -> Result<usize, Error> {
    // 1) Model creation
    let mut model = Model::new("flowshop")?;

    // 2) Big-M constant
    let big_m = 1000.0_f64;

    // 3) Reshape input times into p[j][k]
    let mut p = vec![vec![0.0_f64; n_machines]; n_jobs];
    for j in 0..n_jobs {
        for k in 0..n_machines {
            p[j][k] = times[j * n_machines + k] as f64;
        }
    }

    // 4) Decision variables
    // 4a) Start times S[j][k] ≥ 0
    let mut s = vec![vec![Var::new(); n_machines]; n_jobs];
    for j in 0..n_jobs {
        for k in 0..n_machines {
            s[j][k] = add_ctsvar!(model, name: format!("S_{}_{}", j, k), bounds: 0.0..)?;
        }
    }

    // 4b) Binary precedence x[i][j] ∈ {0,1}
    let mut x = vec![vec![Var::new(); n_jobs]; n_jobs];
    for i in 0..n_jobs {
        for j in 0..n_jobs {
            x[i][j] = add_binvar!(model, name: format!("x_{}_{}", i, j))?;
        }
    }

    // 4c) Makespan C_max ≥ 0
    let c_max = add_ctsvar!(model, name: "C_max", bounds: 0.0..)?;

    // 5) Constraints
    // 5a) Every job j must finish on last machine by C_max
    for j in 0..n_jobs {
        model.add_constr(
            format!("finish_by_{}", j).as_str(),
            c!(s[j][n_machines - 1] + p[j][n_machines - 1] <= c_max),
        )?;
    }

    // 5b) Machine‐sequence for each job j: operation k+1 starts after k finishes
    for j in 0..n_jobs {
        for k in 0..(n_machines - 1) {
            model.add_constr(
                format!("seq_{}_{}", j, k).as_str(),
                c!(s[j][k + 1] >= s[j][k] + p[j][k]),
            )?;
        }
    }

    // 5c) No‐overlap on machine k via big-M and x-vars
    for i in 0..n_jobs {
        for j in 0..n_jobs {
            if i == j {
                continue;
            }
            for k in 0..n_machines {
                let pi = p[i][k];
                let pj = p[j][k];

                // If x[i][j]=1 ⇒ i after j
                model.add_constr(
                    format!("noover_{}_{}_{}", i, j, k).as_str(),
                    c!(s[i][k] >= s[j][k] + pj - big_m * (1.0 - x[i][j])),
                )?;

                // If x[i][j]=0 ⇒ j after i
                model.add_constr(
                    format!("noover_{}_{}_{}'", i, j, k).as_str(),
                    c!(s[j][k] >= s[i][k] + pi - big_m * x[i][j]),
                )?;
            }
        }
    }

    // 5d) Symmetry breaking: for each i<j, x[i][j] + x[j][i] == 1
    for i in 0..n_jobs {
        for j in (i + 1)..n_jobs {
            model.add_constr(
                format!("sym_{}_{}", i, j).as_str(),
                c!(x[i][j] + x[j][i] == 1),
            )?;
        }
    }

    // 6) Objective: minimize C_max
    model.set_objective(c_max.clone(), ModelSense::Minimize)?;

    // 7) Optimize
    model.optimize()?;
    assert_eq!(model.status()?, Status::Optimal);

    // 8) Retrieve makespan value
    let c_val = c_max.get(attr::VarDoubleAttr_X)?;

    Ok(c_val.ceil() as usize)
}
