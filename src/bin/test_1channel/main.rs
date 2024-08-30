use dam::simulation::ProgramBuilder;
use kvcachesim::nodes::worker::Worker;
use kvcachesim::nodes::gpu_l1::GPU_L1;

fn main() {
    // define the program builder
    let mut program_builder = ProgramBuilder::default();
    let capacity = 1000;
    let (cu_sender, cu_receiver) = program_builder.bounded(capacity);

    // define the compute unit and the L1 cache
    let compute_unit = Worker::init(cu_sender);
    let l1_cache: GPU_L1 = GPU_L1::init(cu_receiver);

    program_builder.add_child(compute_unit);
    program_builder.add_child(l1_cache);

    // start running
    program_builder.initialize(Default::default()).unwrap().run(Default::default());
}