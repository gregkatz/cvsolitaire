#[derive(Debug)]
pub enum Error{
    NothingInUtl,
    StackCantParent,
    StackOutOfOrder,
    JacksNotVisible,
    MoveJacks,
    MustClickCard,
    MultipleToSlot,
    CardNotNumeric,
    NoCardClicked,
    NoOpenUtility,
    OrdCantParent,
    BadSourceOrDest,
    UtlNotOpen,
    InvalidConv,
}
