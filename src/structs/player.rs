use super::game::Potion;



#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Class {
    Warrior,
    Mage,
    Rogue
}
#[derive(Debug, PartialEq, Clone)]
pub enum Morality {
    Evil,
    Good
}
#[derive(Debug, PartialEq, Clone)]
pub enum Resource {
    Mana(u32),
    Energy(u32),
    Rage(u32)
}

#[derive(Debug, PartialEq, Clone)]

pub struct Player {
    class: Class,
    path: Morality,
    health: u32,
    resource: Resource,
    story: String,
    pub damage: u32,
    pub inventory: Vec<Potion>
}



#[derive(Debug, PartialEq, Clone)]

pub struct Enemy {
    name: String, 
    pub resource: Resource,
   pub health: u32,
   pub damage: u32
}


impl Enemy {
    pub fn new(name: String, resource: Resource, health: u32, damage: u32)  -> Self {
        Self {
            name,
            resource,
            health,
            damage
        }
    }



}


impl Player {
    pub fn new(class: Class, path: Morality, health: u32, resource: Resource, story: String, damage: u32, inventory: Vec<Potion>) -> Self {
        Self {
            class,
            path,
            health,
            resource, 
            story,
            damage,
            inventory
        }
    }

    pub fn stats(&self) {
        println!("Current stats are as follows: Class- {:#?}, Path- {:#?}, Current Health- {}, Class resource- {:#?}, ", self.class, self.path, self.health, self.resource)
    }


    pub fn get_health(&self) -> u32 {
        self.health
    }

    pub fn get_class(&self) -> Class {
        self.class
    }

    pub fn set_class(&mut self, new_class: Class){
        self.class =  new_class;
    }

    pub fn set_health(&mut self, new_health: u32) {
        self.health = new_health;
    }

    
    pub fn set_damage(&mut self, new_damage: u32) {
        self.damage = new_damage;
    }

    pub fn set_resource(&mut self, new_resource: Resource){
        self.resource = new_resource;
    }

    

    pub fn set_potion(&mut self){

        println!("The enemy dropped a health potion!");
        self.inventory.push(Potion::Health(100));
    

    }


    

    // thanks chatgpt cause im noob with vecs prob rework this but i needed to heal the player cause they cannot survive 5 rounds without heals 
    pub fn use_health_potion(&mut self) {
        // Remove the first item from the inventory
        if let Some(potion) = self.inventory.pop() {
            // Handle the removed potion (you can adjust this part based on your needs)
            match potion {
                Potion::Health(value) => {
                    self.health += value;
                    println!("Used a health potion. Current health: {}", self.health);
                }
                _ => {
                    self.damage += 100;
                    println!("Used a potion of damage booster. current damange: {}", self.damage);
                }
            }
        } else {
            println!("No potion found in inventory.");
        }
    }

    

    pub fn player_story(&mut self) -> String {
        if self.class == Class::Warrior {
            self.story = "You are a great warrior. from birth you have had strong genetics physically and excelled in sword fighting, hand to hand while growing up".to_owned();
            format!("{}", self.story)
        } else if self.class == Class::Mage {
            self.story = "You are a great mage. from birth you have had a lot of mana and excelled in school".to_owned();
            format!("{}", self.story)

        } else {
            self.story = "classified".to_owned();
            format!("{}", self.story)

        }
    }

}