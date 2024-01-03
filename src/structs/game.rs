use std::process;


use super::player::Player;
use super::player::Morality;
use super::player::Class;
use super::player::Resource;
use super::player::Enemy;
use dialoguer::Select;



pub struct Game {
    pub current_round: u32,
   
}

#[derive(Debug, PartialEq, Clone, Copy)]

pub enum Potion {
   Health(u32),
} 



//  player: Player,
// enemy: Enemy,
impl Game {
    
    pub fn new() -> Self {
        Self {
            current_round: 1
        }
    }



    pub fn main_menu(&mut self){


        let class_select: Vec<&str> = vec!["Start", "Quit"];

        let selection = Select::new()
            .with_prompt("Welcome to console fighting")
            .items(&class_select)
            .interact()
            .unwrap();


        if class_select[selection] == "Start"{
            self.start();
        } else {
            process::exit(0);
        }



    }

    pub fn fight_menu(&mut self, player: &mut Player, enemy: &mut Enemy){
        let class_select: Vec<&str> = vec!["Attack", "Use Item", "Give up"];

        let selection = Select::new()
            .with_prompt("Select Move")
            .items(&class_select)
            .interact()
            .unwrap();


        if class_select[selection] == "Attack"{
            self.attack(enemy, player);
        } else if class_select[selection] == "Use Item" {
            self.inventory_select(player);
        } else {
            process::exit(0);
        }
    }

    pub fn start(&mut self){
        

        let class_select: Vec<&str> = vec!["Warrior", "Mage", "Rogue"];

        let selection = Select::new()
            .with_prompt("Choose your class")
            .items(&class_select)
            .interact()
            .unwrap();
    
        println!("You chose: {}", class_select[selection]);

        let mut player: Player = Player::new(Class::Warrior, Morality::Good, 500, Resource::Rage(100), "Default".to_owned(), 100, vec![Potion::Health(100)]);

        player.stats();


        if class_select[selection] == "Warrior" {
            println!("You chose warrior class!");
            player.set_class(Class::Warrior);
            player.set_health(350);
            player.set_damage(40);
            player.set_resource(Resource::Rage(100));
        } else if class_select[selection] == "Mage" {
            println!("You chose mage!");
            player.set_class(Class::Mage);
            player.set_health(250);
            player.set_damage(80);
            player.set_resource(Resource::Mana(100));

        } else {
           
            println!("You chose rogue");
            player.set_class(Class::Rogue);
            player.set_health(300);
            player.set_damage(45);
            player.set_resource(Resource::Energy(100));
        }

        let mut enemy = Enemy::new("vilian".to_owned(), Resource::Rage(100), 100, 15);



        player.stats();
        let story = player.player_story();

        println!("{}", story);
        println!("Round 1");

     
        self.fight_loop(&mut enemy, &mut player, self.current_round);
        

       

    }




    pub fn inventory_select(&mut self, player: &mut Player) {



        let potion_select: Vec<&str> = vec!["Use Health Potion", "Use Damage Booster Potion"];

        let selection = Select::new()
            .with_prompt("Select a potion")
            .items(&potion_select)
            .interact()
            .unwrap();


        if potion_select[selection] == "Use Health Potion"{
            player.use_health_potion();
            
        } else {
            println!("You have no damage boosters!");
        }
    }


    pub fn fight_loop(&mut self, enemy: &mut Enemy, player: &mut Player, round: u32) {
        loop {

            self.fight_menu(player, enemy);
            self.enemy_attack(enemy, player);

            if player.get_health() <= 0 {
                println!("You died");
                self.main_menu();
                break;
            }

            if enemy.health <= 0{
                println!("You beat the challenger!");

                // enenmy drops a potion.
                player.set_potion(); 
                
                if self.current_round > 5 {
                    self.current_round = 2;
                    

                } else {
                    self.current_round += 1;


                }

                self.round(player);
                break;
            }
        }
    }

    pub fn get_round(&self) -> u32 {
        self.current_round
    }



    pub fn round(&mut self, player: &mut Player) {

        let mut enemy_two: Enemy = Enemy::new("Jayve".to_owned(), Resource::Rage(100), 200, 15);
        let mut enemy_three: Enemy = Enemy::new("frewf".to_owned(), Resource::Rage(100), 200, 15);
        let mut enemy_four: Enemy = Enemy::new("f".to_owned(), Resource::Rage(100), 200, 15);
        let mut enemy_five: Enemy = Enemy::new("Jayve".to_owned(), Resource::Rage(100), 200, 15);
        let mut cloned_player = player.clone();

        match self.get_round() {
            1 => 
            {
                println!("Round 1");
            },
            2 =>  {
                println!("Round 2");
                self.fight_loop(&mut enemy_two, &mut cloned_player, 2)
            },
            3 =>   {
                println!("Round 3");
                self.fight_loop(&mut enemy_three, &mut cloned_player, 3)
            },
            4 =>   {
                println!("Round 4");
                self.fight_loop(&mut enemy_four, &mut cloned_player, 4)
            },
            5 =>   {
                println!("Round 5");
                self.fight_loop(&mut enemy_five, &mut cloned_player, 5);
            },
            _ => {println!("You won the Arena Championship!!! Congratulations!");
            self.main_menu();
        }
        }

        
    }

   



    pub fn attack(&self, enemy: &mut Enemy, player: &mut Player) {

       println!("Enemy current hp: {}", enemy.health); 

        
        // execute
        if  enemy.health < 0 || enemy.health < player.damage {
            enemy.health = 0;       
           } else {
            enemy.health -= player.damage;

           }

      
        println!("Player attacked enemy! - their current hp. {}", enemy.health);

        
    }

    pub fn enemy_attack(&self, enemy: &mut Enemy, player: &mut Player) {
        let mut current_health = player.get_health();
        println!("Your current hp {}", current_health);

     

        // execute
        if current_health < enemy.damage {
         current_health = 0;       
        } else {
            current_health -= enemy.damage;

        }


        if current_health < 0 {
            current_health = 0
        }

        player.set_health(current_health);        
        
       

        println!("Enemy attacked you! new hp {}", current_health);

    }


}