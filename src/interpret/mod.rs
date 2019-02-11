mod call_stack;

use std::iter;

use datum::Datum;
use spell::Instruction;
use spell::Local;
use spell::SpellId;

pub use self::call_stack::*;

/// Interpret a single instruction and return what should happen to the call
/// stack.
#[inline(always)]
pub fn interpret_instruction<'a>(
    program_counter: ProgramCounter<'a>,
    local_variables: &'a mut [Datum],
) -> CallStackMutation<'a> {
    macro_rules! local {
        ($l:expr) => {{
            local_variables.get($l.0 as usize)
                .expect("Local variable out of bounds")
                .clone()
        }};
        ($l:expr, $v:expr) => {{
            *local_variables.get_mut($l.0 as usize)
                .expect("Local variable out of bounds")
                = $v;
        }};
    }

    match program_counter.get() {

        Instruction::Copy{from, to} => {
            let value = local!(from);
            local!(to, value);
            CallStackMutation{
                jump: program_counter.advance(),
                exit: None,
                call: None,
            }
        },

        Instruction::InvokeStatic{result, spellbook, spell, arguments} => {
            let argument_values: Box<[Datum]> =
                    arguments.iter().map(|l| local!(l)).collect();

            let callee = SpellId{
                spellbook: *spellbook,
                spell:     *spell,
                arity:     argument_values.len(),
            };
            let call = Call{
                callee:      callee,
                arguments:   argument_values,
                return_into: *result,
            };

            CallStackMutation{
                jump: program_counter.advance(),
                exit: None,
                call: Some(call),
            }
        },

        Instruction::InvokeDynamic{result, spell, receiver, arguments} => {
            let receiver_value = local!(receiver);
            let argument_values: Box<[Datum]> =
                iter::once(receiver_value.clone())
                    .chain(arguments.iter().map(|l| local!(l)))
                    .collect();

            let callee = SpellId{
                spellbook: receiver_value.enchantment(),
                spell:     *spell,
                arity:     argument_values.len(),
            };
            let call = Call{
                callee:      callee,
                arguments:   argument_values,
                return_into: *result,
            };

            CallStackMutation{
                jump: program_counter.advance(),
                exit: None,
                call: Some(call),
            }
        },

        Instruction::Return{result} => {
            let value = local!(result);
            CallStackMutation{
                jump: program_counter,
                exit: Some(value),
                call: None,
            }
        },

    }
}

/// A description of what must happen to the call stack after interpreting an
/// instruction.
///
/// Both exit and call may be set. In that case, a tail call must occur.
#[derive(Debug)]
pub struct CallStackMutation<'a> {
    /// Which instruction to jump to before changing the active stack
    /// frame. Not relevant if exit is set.
    pub jump: ProgramCounter<'a>,

    /// Exit the stack frame with a return value.
    pub exit: Option<Datum<'a>>,

    /// Create a new stack frame, invoking a spell with some arguments.
    pub call: Option<Call<'a>>,
}

/// A call to be performed.
#[derive(Debug)]
pub struct Call<'a> {
    pub callee:      SpellId,
    pub arguments:   Box<[Datum<'a>]>,
    pub return_into: Local,
}
