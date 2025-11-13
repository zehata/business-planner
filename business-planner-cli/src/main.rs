use business_planner::cli_api::init::init_shell::init;

pub mod linear;

fn main() {
    let estimator = crate::linear::estimator::LinearRegression{};

    init();
}

