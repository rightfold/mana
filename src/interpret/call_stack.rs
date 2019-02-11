use datum::Datum;
use spell::Instruction;
use spell::Local;

/// A call stack is a sequence of stack frames.
#[derive(Debug)]
pub struct CallStack<'a> {
    pub stack_frames: Vec<StackFrame<'a>>,
}

/// A stack frame consists of a program counter and local variables.
///
/// A stack frame represents an active spell invocation.
#[derive(Debug)]
pub struct StackFrame<'a> {
    pub program_counter: ProgramCounter<'a>,
    pub local_variables: Box<[Datum<'a>]>,

    /// The local variable to store the result into when the callee returns. If
    /// this is the active stack frame, the value of this field is irrelevant.
    pub return_into: Local,
}

/// A program counter points into the instructions of a spell, and tells the
/// interpreter which instruction comes next.
#[derive(Clone, Copy, Debug)]
pub struct ProgramCounter<'a> {
    pub instructions:     &'a [Instruction],
    pub next_instruction: usize,
}

impl ProgramCounter<'_> {
    /// Get the next instruction or panic.
    #[inline(always)]
    pub fn get(&self) -> &Instruction {
        self.instructions.get(self.next_instruction)
            .expect("Program counter out of bounds")
    }

    /// Jump to the next instruction.
    #[inline(always)]
    pub fn advance(&self) -> Self {
        self.jump(self.next_instruction + 1)
    }

    /// Jump to an arbitrary instruction.
    #[inline(always)]
    pub fn jump(&self, target: usize) -> Self {
        ProgramCounter{
            instructions:     self.instructions,
            next_instruction: target,
        }
    }
}
