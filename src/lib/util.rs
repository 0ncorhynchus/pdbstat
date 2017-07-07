use lib::error::Result;
use chemfiles::Topology;

pub fn calc_distance(pos1: &[f64; 3], pos2: &[f64; 3]) -> f64 {
    ((pos2[0] - pos1[0]).powi(2)
     + (pos2[1] - pos1[1]).powi(2)
     + (pos2[2] - pos1[2]).powi(2)).sqrt()
}

pub fn list_residue_and_name(topology: &Topology) -> Result<(Vec<Option<usize>>, Vec<String>)> {
    let natoms = topology.natoms()?;
    let mut ids = vec![None; natoms as usize];
    let mut names = vec![String::new(); natoms as usize];

    for i in 0..natoms {
        ids[i as usize] = match topology.residue_for_atom(i)? {
            Some(res) => Some(res.id()? as usize),
            None => None
        };
        names[i as usize] = topology.atom(i)?.name()?;
    }

    Ok((ids, names))
}
