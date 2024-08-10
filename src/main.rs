mod plan;
mod run;

use plan::Plan;
use run::run_in_docker;

fn main() -> anyhow::Result<()> {
    let recipe = Plan::from_file("cio.yaml")?;
    println!("{:#?}", recipe);
    let mut absolute_success = true;

    for (name, mission) in recipe.missions {
        println!("Launching '{}'", name);
        let status = run_in_docker(mission)?;

        if !status.success() {
            eprintln!("Task failed with status: {:?}", status.code());
            absolute_success = false;
        }
    }

    if absolute_success {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Some missions have failed."))
    }
}
