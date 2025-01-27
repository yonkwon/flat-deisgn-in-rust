use std::fs;
use std::path::Path;
use std::thread::available_parallelism;
pub mod params;
pub mod scenario;
pub mod network_analyzer;
pub mod experiment_manager;
pub mod hdf5_manager;

fn main() -> std::io::Result<()> {
    let mut num_thread = params::MAX_THREAD;
    if num_thread > available_parallelism().unwrap().get() {
        num_thread = available_parallelism().unwrap().get()
    }
    rayon::ThreadPoolBuilder::new().num_threads(num_thread).build_global().unwrap();
    println!("This simulation will run on {} threads", num_thread);
    
    params::initialize_once_cells();
    let mut experiment_manager = experiment_manager::ExperimentManager::new();

    if params::GET_GRAPH {
        let folder_name = (*params::PARAM_STRING).clone();
        if !Path::new(&folder_name).exists() {
            fs::create_dir(&folder_name)?;
        }
        experiment_manager.sample_network_csv();
    }

    if params::GET_MAT {
        let tic = std::time::Instant::now().elapsed().as_secs();
        experiment_manager.run_experiments();
        let toc = std::time::Instant::now().elapsed().as_secs();
        let hdf5_manager = hdf5_manager::HDF5Manager::new(experiment_manager, toc-tic);
        hdf5_manager.write_to_file();
    }

    Ok(())
}
