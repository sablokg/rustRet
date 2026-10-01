use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "retentiontools for mass spectrometry",
    version = "1.0",
    about = "rustRET.
           ************************************************
           Author Gaurav Sablok,
           Email: gsablok@proton.me
          ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// retention index calculation
    RetentionIndex {
        /// provide the path to the retetnion file
        csvfile: String,
        /// threads for the analysis
        thread: String,
    },
    /// retention time calculate
    TimeRetention {
        /// provide the path to the file for the retention time calculation
        filepath: String,
        /// threads for the analysis
        thread: String,
    },
    /// retention time adjusted calculation
    RetentionTimeAdjust {
        /// provide the path to the file,
        filepath: String,
        /// retention factor
        retentionfactor: String,
        /// threads for the analysis
        thread: String,
    },
    /// machine learning metabolomics
    Machinelearning {
        /// fasta file for the sequences
        fastafile: String,
        /// threshold assoicated with the sequences a csv file with the assoicated peak for the classification
        threshold: String,
        /// fasta file for the prediction
        predictionfile: String,
        /// threads for the analysis
        thread: String,
    },
}
