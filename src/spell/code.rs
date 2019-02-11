use sigil::Sigil;

/// An instruction is the smallest unit of executable code.
#[derive(Clone, Debug)]
pub enum Instruction {
    /// Copy the datum from one variable into another.
    Copy{
        from: Local,
        to:   Local,
    },

    /// Invoke a spell using static dispatch.
    InvokeStatic{
        result:    Local,
        spellbook: Sigil,
        spell:     Sigil,
        arguments: Box<[Local]>,
    },

    /// Invoke a spell using dynamic dispatch on the enchantment of the
    /// receiver.
    InvokeDynamic{
        result:    Local,
        spell:     Sigil,
        receiver:  Local,
        arguments: Box<[Local]>,
    },

    /// Return to the caller, giving it a datum.
    Return{
        result: Local,
    },
}

/// A local variable indexes into the array of local variables on the stack
/// frame.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Local(pub u32);
