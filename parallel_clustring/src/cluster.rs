use std::fmt;

#[derive(Debug)]
pub struct Cluster {
    fips_codes:         Vec<u64>,
    horiz_center:       f64,
    vert_center:        f64,
    total_population:   u64,
    averaged_risk:      f64,
}

impl Cluster {
    pub fn new(fips: u64, horiz: f64, vert: f64, population: u64, risk: f64)
           -> Self {
                Cluster {
                   fips_codes:          vec![fips],
                   horiz_center:        horiz,
                   vert_center:         vert,
                   total_population:    population,
                   averaged_risk:       risk,
                }
           }

    pub fn fips_codes(&self) -> Vec<u64> {
        self.fips_codes
    }

    pub fn horiz_center(&self) -> f64 {
        self.horiz_center
    }

    pub fn vert_center(&self) -> f64 {
        self.vert_center
    }

    pub fn total_population(&self) -> u64 {
        self.total_population
    }

    pub fn averaged_risk(&self) -> f64 {
        self.averaged_risk
    }

    pub fn distance(&self, other_cluster: &Cluster) -> f64 {
        // Compute the Euclidean distance between two clusters
        let vert_dist = self.vert_center - other_cluster.vert_center();
        let horiz_dist = self.horiz_center - other_cluster.horiz_center();
        (vert_dist * vert_dist + horiz_dist * horiz_dist).sqrt()
    }
    
    // take the owenership of the other_cluster
    pub fn merge_clusters(&mut self, other_cluster: Cluster) {
    }

}

impl fmt::Display for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut codes = String::new();
        for &code in &self.fips_codes {
            codes.push_str(&code.to_string());
        }
        write!(f, "cluster: {}\ncenter: ({}, {})\n",
               codes, self.horiz_center, self.vert_center)
    }
}