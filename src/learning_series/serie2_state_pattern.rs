

#[derive(Clone, Copy)]
pub enum EntityState {
    InReview,
    Published,
    NonVisible,
    Invalid,
}

pub trait StateTransition {
    fn need_review(&self) -> Box<dyn StateTransition>;
    fn pushish(&self) -> Box<dyn StateTransition>;
    fn non_visible(&self) -> Box<dyn StateTransition>;
    fn invalid(&self) -> Box<dyn StateTransition>;
    fn status(&self) -> String;
}

impl StateTransition for EntityState {
    fn need_review(&self) -> Box<dyn StateTransition> {
        match self {
            EntityState::InReview => {
                println!("Entity is already in review!");
                Box::new(self.clone())
            }
            EntityState::Invalid => { 
                println!("Entity is invalid can't be changed!");
                Box::new(self.clone())
            }
            _ => Box::new(EntityState::InReview)

        }
    }

    fn pushish(&self) -> Box<dyn StateTransition> {
        match self {
            EntityState::InReview | EntityState::NonVisible => {
                println!("Entity has been made published!");
                Box::new(EntityState::Published)
            },
            _ => {
                println!("Entity need to be in review to be published!");
                Box::new(self.clone())
            }
        }
    }

    fn non_visible(&self) -> Box<dyn StateTransition> {
        match self {
            EntityState::Published => {
                println!("Entity has been hidden!");
                Box::new(EntityState::NonVisible)
            },
            _ => {
                println!("Entity needs to be published to be hidden!");
                Box::new(self.clone())
            }
        }
    }

    fn invalid(&self) -> Box<dyn StateTransition> {
        println!("Entity has been invalidated!");

        Box::new(EntityState::Invalid)
    }

    fn status(&self) -> String {
        match self  {
            EntityState::InReview => String::from("InReview"),
            EntityState::NonVisible => String::from("NonVisible"),
            EntityState::Published =>  String::from("Published"),
            EntityState::Invalid => String::from("Invalid")
        }
    }
}

pub struct Specie {
    pub kingdom: String,
    pub phylum: String,
    pub class: String,
    pub order: String,
    pub family: String,
    pub genus: String,
    pub specie: String, 
    pub state: Box<dyn StateTransition>,
}

impl Specie {
    pub fn new(specie: &str, genus: &str, family: &str, order: &str, class: &str, phylum: &str, kingdom: &str) -> Self {
        Specie {
            kingdom: String::from(kingdom),
            phylum: String::from(phylum),
            class: String::from(class),
            order: String::from(order),
            family: String::from(family),
            genus: String::from(genus),
            specie: String::from(specie),
            state: Box::new(EntityState::InReview),
        }
    }

    pub fn publish(&mut self) {
        let state = self.state.pushish();
        self.state = state;
    }

    pub fn invalidate(&mut self) {
        let state = self.state.invalid();
        self.state = state;
    }

    pub fn hide(&mut self) {
        let state = self.state.non_visible();
        self.state = state;
    }

    pub fn status(self) -> String {
        self.state.status()
    }
}

#[cfg(test)]
mod tests {
    use super::Specie;

    #[test]
    fn spacie_transition() {
        let mut specie = Specie::new(
            "P. tigris",
            "Panthera",
            "Felidae",
            "Carnivora",
            "Mammalia",
            "Chordata",
            "Animalia"
        );

        specie.hide();

        specie.publish();

        assert_eq!(specie.status(), "Published")
    }
}