#[derive(Debug)]
struct Character {
    class: String,
    hp: u16,
    mana: u16
}

impl Character {
    // Create a new Char struct
    fn new(class: &str, hp: u16, mana: u16) -> Character {
        Character {
            class: class.to_string(),
            hp: hp,
            mana: mana
        }
    }

    // calculate total product of hp and mana
    fn total_power(&self) -> u16 {
        self.hp * self.mana
    }

    // How many hits char can take base on opponents attack
    fn maximum_hits(&self, attack: u16) -> u16 {
        self.hp / attack
    }

    fn take_hit(&mut self, damage: u16) {
        if damage >= self.hp {
            self.hp = 0;
        } else if damage < self.hp {
            self.hp -= damage;
        }
    }

    fn cast_magic(&mut self, spell: u16) {
        if spell == self.mana {
            self.mana = 0;
        } else if spell < self.mana {
            self.mana -= spell
        } else {
            println!("Not enough mana to cast that spell!");
        }
    }

    fn drink_health_potion(&mut self, potion: u16) {
        self.hp += potion;
        if self.hp > 100 {
            self.hp = 100;
        }
    }

    fn drink_mana_potion(&mut self, potion: u16) {
        self.mana += potion;
        if self.mana > 100 {
            self.mana = 100;
        }
    }


}

pub fn run() {
    // Create Char
    let mut npc = Character::new("Rogue", 100, 80);
    println!("Character: {:?}", npc);
    println!("Class: {} with HP: {} and MANA: {}", npc.class, npc.hp, npc.mana);

    // Hit npc
    let hit: u16 = 25;
    npc.take_hit(hit);
    println!("npc HP after taking {} hit is {}", hit, npc.hp);

    // Drink health potion
    let small_health_potion: u16 = 20;
    npc.drink_health_potion(small_health_potion);
    println!("HP after drinka a potion: {}", npc.hp);

    // Npc hits back with magic spell and then drinks potion
    let spell: u16 = 30;
    npc.cast_magic(spell);
    println!("Mana after casting spell: {}", npc.mana);

    // Power of a npc
    let total = npc.total_power();
    println!("total power of an npc is: {}", total);

    npc.drink_mana_potion(55);
    println!("Refilled mana and we have: {} mana", npc.mana);

}