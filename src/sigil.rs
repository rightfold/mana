use std::collections::HashMap;
use std::sync::Arc;

/// A sigil is some sort of identifier.
///
/// Internally, sigils are used for referring to spellbooks and spells, but
/// user code may use them for any purpose.
///
/// Sigils are implemented using a technique known as _interning_. This means
/// that any particular sigil only exists in one place in memory. As a
/// consequence, hashing sigils and comparing them for equality is very
/// efficient.
///
/// To create a new sigil, you need a sigil database. See [Sigils] for more
/// information.
///
/// [Sigils]: struct.Sigils.html
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Sigil(pub u32);

/// A sigil database maps sigils to their names and vice versa.
///
/// A sigil database automatically creates new sigils that were previously
/// unknown. Sigils are not released until the entire sigil database is
/// released, so you should not create them from untrusted input.
#[derive(Debug)]
pub struct Sigils {
    by_id:   Vec<Arc<[u8]>>,
    by_name: HashMap<Arc<[u8]>, Sigil>,
}

impl Sigils {
    /// Create an empty sigil database.
    ///
    /// The returned database is distinct from other databases, and sigils
    /// created using one database should not be queried using another
    /// database.
    pub fn new() -> Self {
        Sigils{by_id: Vec::new(), by_name: HashMap::new()}
    }

    /// Get the name of a sigil in the database.
    pub fn name(&self, sigil: Sigil) -> Option<&Arc<[u8]>> {
        self.by_id.get(sigil.0 as usize)
    }

    /// Get a sigil by its name. If the sigil does not yet exist in the
    /// database, it is first created.
    pub fn intern(&mut self, name: &Arc<[u8]>) -> Sigil {
        if let Some(&sigil) = self.by_name.get(name) {
            sigil
        } else {
            let sigil = Sigil(self.by_id.len() as u32);
            self.by_id.push(name.clone());
            self.by_name.insert(name.clone(), sigil);
            sigil
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let mut sigils = Sigils::new();
        let name = Arc::from("foo".as_bytes());
        let sigil = sigils.intern(&name);
        assert_eq!(sigils.name(sigil), Some(&name));
    }

    #[test]
    fn test_identity() {
        let mut sigils = Sigils::new();

        let name_a = Arc::from("foo".as_bytes());
        let name_b = Arc::from("bar".as_bytes());

        let sigil_a_1 = sigils.intern(&name_a);
        let sigil_a_2 = sigils.intern(&name_a);
        let sigil_b   = sigils.intern(&name_b);

        assert_eq!(sigil_a_1, sigil_a_2);
        assert_ne!(sigil_a_1, sigil_b  );
        assert_ne!(sigil_a_2, sigil_b  );
    }
}
