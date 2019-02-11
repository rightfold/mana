mod code;

use std::collections::HashMap;

use sigil::Sigil;

pub use spell::code::*;

/// A spell is identified by the name of the spellbook it is defined in, the name
/// of the spell, and the arity of the spell.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpellId {
    pub spellbook: Sigil,
    pub spell: Sigil,
    pub arity: usize,
}

/// A spell is a sequence of instructions that can be executed using a single
/// stack frame.
#[derive(Debug)]
pub struct Spell {
    /// The sequence of instructions that are to be interpreted when the spell
    /// is invoked.
    pub instructions: Box<[Instruction]>,

    /// How many local variable slots must be allocated when the spell is
    /// invoked.
    ///
    /// For _n_ the arity of the spell spell, the first _n_ local variables are
    /// filled with the values of the arguments when the spell is invoked.
    pub local_variables: usize,
}

/// A spell database is a collection of spells.
#[derive(Debug)]
pub struct Spells {
    spells: HashMap<SpellId, Spell>,
}

impl Spells {
    /// Create an empty spell database.
    pub fn new() -> Self {
        Spells{spells: HashMap::new()}
    }

    /// Get a spell by its spellbook name, spell name, and arity.
    pub fn get(&self, id: SpellId) -> Option<&Spell> {
        self.spells.get(&id)
    }

    /// Insert a spell into the database, or return an error if the spell
    /// already exists.
    pub fn insert(&mut self,
                  id: SpellId,
                  spell: Spell,
                  ) -> Result<(), RedefinitionError> {
        if self.spells.contains_key(&id) {
            Err(RedefinitionError)
        } else {
            self.spells.insert(id, spell);
            Ok(())
        }
    }
}

/// This error is returned when attempting to define a spell that was already
/// defined.
pub struct RedefinitionError;
