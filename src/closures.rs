#[derive(Debug)]

pub struct City {
   pub city: String,
   pub population: u64
}

pub fn sort_pop(city: &mut Vec<City>) {
    city.sort_by_key(pop_helper);
}

pub fn pop_helper(pop: &City) -> u64 {
    pop.population
}
