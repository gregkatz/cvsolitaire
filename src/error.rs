#[derive(Debug)]
pub enum SolError{
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
}
