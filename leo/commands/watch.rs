use crate::{
    cli::*,
    cli_types::*,
    commands::BuildCommand,
    directories::SOURCE_DIRECTORY_NAME,
    errors::{CLIError, RunError},
    files::{Manifest, ProvingKeyFile, VerificationKeyFile, MAIN_FILE_NAME},
};
use leo_compiler::{compiler::Compiler, group::targets::edwards_bls12::EdwardsGroupType};

use snarkos_algorithms::snark::groth16::{Groth16, Parameters, PreparedVerifyingKey, VerifyingKey};
use snarkos_curves::bls12_377::{Bls12_377, Fr};
use snarkos_models::algorithms::snark::SNARK;

use clap::ArgMatches;
use rand::thread_rng;
use std::{convert::TryFrom, env::current_dir, time::Instant};

#[derive(Debug)]
pub struct WatchCommand;

impl CLI for WatchCommand {
    type Options = ();
    type Output = ();

    const ABOUT: AboutType = "Auto-compile the current package on file changes (*)";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "watch";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        // Get the package name
        let path = current_dir()?;
        let package_name = Manifest::try_from(&path)?.get_package_name();

        match BuildCommand::output(options)? {
            Some((program, checksum_differs)) => {
                log::info!("Program re-compiled.");

                Ok(())
            }
            None => {
                let mut main_file_path = path.clone();
                main_file_path.push(SOURCE_DIRECTORY_NAME);
                main_file_path.push(MAIN_FILE_NAME);

                Err(CLIError::RunError(RunError::MainFileDoesNotExist(
                    main_file_path.into_os_string(),
                )))
            }
        }
    }
}
