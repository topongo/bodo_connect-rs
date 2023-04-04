#![allow(dead_code)]

#[cfg(feature = "log")]
use log::debug;
#[cfg(not(feature = "log"))]
use crate::debug;

use subprocess::{ExitStatus, Popen, PopenConfig, PopenError, Redirection};

pub struct SSHProcess {
    args: Vec<String>
}

impl SSHProcess {
    pub fn new(args: Vec<String>) -> SSHProcess {
        SSHProcess {
            args
        }
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn run(&mut self, opts: Option<PopenConfig>) -> Result<ExitStatus, PopenError> {
        debug!("spawning new ssh process");
        match Popen::create(&self.args, match opts {
            Some(x) => x,
            None => PopenConfig::default()
        }) {
            Ok(mut p) => {
                debug!("waiting for ssh process to exit");
                p.wait()
            },
            Err(e) => Err(e)
        }
    }

    // pub fn join(&mut self) -> Result<ExitStatus, PopenError> {
    //     match &mut self.process {
    //         Some(ref mut p) => p.wait(),
    //         None => panic!("cannot wait unstarted process")
    //     }
    // }

    pub fn run_stdout_to_stderr(&mut self) -> Result<ExitStatus, PopenError> {
        debug!("passing redirecting options to ssh process");
        let mut opts = PopenConfig::default();
        opts.stdout = Redirection::Merge;
        self.run(Some(opts))
    }

    pub fn to_string(&self) -> String {
        return self.args.join(" ")
    }
}