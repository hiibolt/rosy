
use mpi::{traits::*, topology::SimpleCommunicator, environment::Universe};
use bincode::{Encode, Decode, config::Configuration};
use anyhow::{Result, Context, ensure, bail};

use crate::RE;


pub struct RosyMPIContext {
	pub universe: Universe,
	pub world: SimpleCommunicator,
	pub size: i32,
	pub rank: i32,
	pub bincode_config: Configuration,
}
impl RosyMPIContext {
    pub fn new () -> Result<Self> {
        let universe = mpi::initialize()
            .context("Failed to initialize MPI")?;
        let world = universe.world();
        Ok(RosyMPIContext {
            universe,
            size: world.size(),
            rank: world.rank(),
            world,
            bincode_config: bincode::config::standard()
        })
    }
    // Coordinates a value array between all different processes
    //  according to the specified communication standard.
    pub fn coordinate<T: Encode + Decode<()>> (
        &self,

        value: &mut Vec<T>,
        communication_standard: u8,
        num_groups: &mut RE 
    ) -> Result<()> {
        match communication_standard {
            1 => {
                // In this standard, each process sends to all other processes 
                //  in its group.
                //
                // For example, for 8 processes and 2 groups,
                // - Process 0 sends to 1, 2, 3
                // - ...
                // - Process 3 sends to 0, 1, 2
                // - Process 4 sends to 5, 6, 7
                // - ...
                // - Process 7 sends to 4, 5, 6 
                let num_groups = *num_groups as i32;

                ensure!(self.size % num_groups == 0, "Total number of processes ({}) must be divisible by the number of processes per group ({})!", self.size, num_groups);
                let processes_per_group = self.size / num_groups;
                let group_id = self.rank / processes_per_group;
                let group_start = group_id * processes_per_group;
                let group_end = group_start + processes_per_group;

                let mut other_nodes = Vec::new();
                for r in group_start..group_end {
                    if r != self.rank {
                        other_nodes.push(r);
                    }
                }

                // Get the value we're going to be sending,
                //  which is the group_id'th element of the array
                let binary_value: Vec<u8> = if (group_id as usize) < value.len() {
                    bincode::encode_to_vec(&value[group_id as usize], self.bincode_config)
                        .context("Failed to serialize value for communication")?
                } else {
                    bail!("Not enough elements in value array to coordinate! Expected at least {} elements, found {}", group_id + 1, value.len());
                };

                // Send this value to all other nodes in the group
                for to_send in other_nodes.iter() {
                    self.world.process_at_rank(*to_send)
                        .send(&binary_value);
                }

                // Now receive values from all other nodes in the group
                for _ in other_nodes.iter() {
                    let (msg, status) = self.world.any_process().receive_vec::<u8>();

                    let (decoded_value, _): (T, usize) = bincode::decode_from_slice(
                        &msg,
                        self.bincode_config
                    ).context("Failed to decode received value")?;

                    // Place this value in the array at the index of the sender's rank
                    let sender_rank = status.source_rank();
                    if (sender_rank as usize) < value.len() {
                        value[sender_rank as usize] = decoded_value;
                    } else {
                        bail!("Not enough elements in value array to coordinate! Expected at least {} elements, found {}", sender_rank + 1, value.len());
                    }
                }

                Ok(())
            },
            _ => bail!( "Unsupported communication standard: {}", communication_standard )
        }
    }
    pub fn get_rank ( 
        &self,
        num_groups: &mut RE
    ) -> Result<RE> {
        let num_groups = *num_groups as i32;

        ensure!(self.size % num_groups == 0, "Total number of processes ({}) must be divisible by the number of processes per group ({})!", self.size, num_groups);
        let processes_per_group = self.size / num_groups;
        let rank_in_group = self.rank % processes_per_group;

        Ok(rank_in_group as RE)
    }
    fn _get_root_rank ( 
        &self,
        num_groups: &mut RE,
    ) -> Result<RE> {
        let num_groups = *num_groups as i32;

        ensure!(self.size % num_groups == 0, "Total number of processes ({}) must be divisible by the number of processes per group ({})!", self.size, num_groups);
        let processes_per_group = self.size / num_groups;
        let root_rank = self.rank - (self.rank % processes_per_group);

        Ok(root_rank as RE)
    }
}