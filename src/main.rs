use skani::chain;
use skani::file_io;
use skani::params::*;
use skani::regression;

fn default_params(mode: Mode) -> (CommandParams, SketchParams) {
    let cmd_params = CommandParams {
        screen: false,
        screen_val: 0.00,
        mode: mode,
        out_file_name: "".to_string(),
        ref_files: vec![],
        query_files: vec![],
        refs_are_sketch: false,
        queries_are_sketch: false,
        robust: false,
        median: false,
        sparse: false,
        full_matrix: false,
        max_results: 10000000,
        individual_contig_q: false,
        individual_contig_r: false,
        min_aligned_frac: 0.15,
        keep_refs: false,
        est_ci: false,
        learned_ani: true,
        learned_ani_cmd: false,
        detailed_out: false,
    };

    let m = 1000;
    let c = 125;
    let k = 15;
    let sketch_params = SketchParams::new(m, c, k, false, false);
    return (cmd_params, sketch_params);
}

fn main() {
    //Vector of Strings
    let refs = vec!["./test_files/e.coli-W.fasta.gz".to_string()];
    let queries = vec!["./test_files/e.coli-K12.fasta".to_string()];

    let (command_params, mut sketch_params) = default_params(Mode::Dist);

    //Change c as needed
    sketch_params.c = 200;
//    sketch_params.c = 30;

    //Learned regression model only when c >= 70 by default. 
    //skani uses learned ani debiasing via regression only when c >= 70 by default
    let use_learned_ani = true;
//    let use_learned_ani = false;

    let model_opt = regression::get_model(sketch_params.c, use_learned_ani);
    //The booleans don't need to be changed unless you want to do screening
    //with markers.
    let ref_sketches = file_io::fastx_to_sketches(&refs, &sketch_params, true);
    let query_sketches = file_io::fastx_to_sketches(&queries, &sketch_params, true);
    for ref_sketch in ref_sketches.iter() {
        for query_sketch in query_sketches.iter() {
            let map_params = chain::map_params_from_sketch(ref_sketch, false, &command_params);
            let mut ani_result = chain::chain_seeds(ref_sketch, query_sketch, map_params);
            dbg!(&ani_result);
            //let ani = ani_result.ani
            //let af_q = ani_result.align_fraction_query
            //let af_r = ani_result.align_fraction_ref

            if model_opt.is_some() {
                let model = model_opt.as_ref().unwrap();
                regression::predict_from_ani_res(&mut ani_result, &model);
                dbg!(&ani_result);
                //let debiased_ani = ani_result.ani;
            }
        }
    }
}
