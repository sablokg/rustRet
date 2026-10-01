use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
mod args;
mod machine;
mod retentionindex;
mod retentiontime;
mod retentiontimeadj;
use figlet_rs::FIGfont;
use machine::regression;
use retentionindex::retentionindexcal;
use retentiontime::retentionindex;
use retentiontimeadj::retentionadjust;

/*
Gaurav Sablok
gsablok@proton.me
*/

fn main() {
    let fontgenerate = FIGfont::standard().unwrap();
    let repgenerate = fontgenerate.convert("rustRET");
    println!("{}", repgenerate.unwrap());
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::RetentionIndex { csvfile, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = retentionindexcal(csvfile).unwrap();
                println!("The merged alignment have been written: {:?}", command);
            });
        }
        Commands::TimeRetention { filepath, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = retentionindex(filepath).unwrap();
                println!("The retention time has been estimated{:?}", command);
            });
        }
        Commands::RetentionTimeAdjust {
            filepath,
            retentionfactor,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = retentionadjust(filepath, retentionfactor).unwrap();
                println!("The value has been estimated:{:?}", command);
            });
        }
        Commands::Machinelearning {
            fastafile,
            predictionfile,
            threshold,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = regression(fastafile, predictionfile, threshold).unwrap();
                println!(
                    "The machine learing for the regression has completed:{}",
                    command
                );
            });
        }
    }
}
