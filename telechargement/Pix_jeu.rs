fn main() {
	let liste_recompenses: Vec<&str> = vec!["Aetherium","Brimstone","Celestium","Draconium","Emberite","Froststone","Glimmerium","Hallowedium","Ignatium","Jadeite","Kyanite","Lumina","Mystrium","Necrostone","Oblivium","Pyrestone","Quicksilver","Runeium","Stardust","Terraflux","Umbraflux","Vitralis","Wraithium","Xenium","Yggdrasilium","Zephyrstone","Aurorium","Blazium","Crysolite","Doomstone","Etherealite","Frostfall","Galaxium","Havocite","Infernite","Jadestone","Krakenium","Luminite","Moltenium","Almanite","Obsidianium","Phoenixium","Quartzite","Solarium","Thunderium","Umbrastone","Vortexite","Wispstone","Xenonite","Aqualite"];


	let mut score = 0;

	let mut vampire: Vec<(i32, &str)> = vec![
	(90, "points de vie"),
	(5, "points de puissance"),
	(24, "points de défense")];

	// Boucle pour surveiller les points de vie du vampire
	while vampire[0].0 != 0 {
		for i in 0..=vampire.len() {
			println!("Le vampire a {} {}.", vampire[i].0, vampire[i].1);
			score += i;
		}
	
		// Simulation d'un combat où le vampire perd des points de vie
		let attaque_en_cours = 20;
		vampire[0] -= attaque_en_cours;
		score += 1;

		// Simulation de la pause entre chaque tour de combat
		std::thread::sleep(std::time::Duration::from_secs(1));
	}

	println!("Vous avez vaincu le vampire ! Vous recevez {} x1.", liste_recompenses[score%50]);
}