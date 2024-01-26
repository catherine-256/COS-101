fn main() {
    let name_of_commisioner =  vec! ["Aigbobun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etiye"];
    let ministry = vec! ["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let geopolitical_zone = vec! ["South West", "North East", "South South", "South West", "South East"];

    println!("\nMinisters from different geopolitical zones in the country:\n");
    for i in 0..geopoloticalzone.len() {
        println!("{} in the {} ministry located in the {} geopolitical zone\n", name_of_commisioner[i], ministry[i], geopolitical_zone[i]);
    }
}