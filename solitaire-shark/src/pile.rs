
// "PILE.ISEMPTY"

// /// Pops the top of the Pile stack to determine which Pile to check. Pushes the top card of the specified Pile onto the
// /// Card stack. Checking the Draw Pile if it is empty will flip three more cards if they are available.
// "PILE.TOPCARD"

// /// Pops the Pile stack to determine which Pile to check. Pushes the bottom card of the face-up cards of the Pile 
// /// specified. If the Draw or any of the Finished piles are requested, this is equivalent to PILE.TOPCARD.
// "PILE.BOTTOMCARD"

// /// Pops the Pile stack to determine the source and destination of the moves. The Pile stack is popped even if the move
// /// is not a legal move (i.e. source of FinishedSpades or destination of Draw). If a Work Pile is the source and another
// /// Work Pile is the destination, the bottom card of the source pile is checked for legal moves. Otherwise the top card
// /// of the source is checked against the top card of the destination for a legal move.
// /// 
// /// Pushes TRUE on the Bool stack if a move was completed
// "PILE.MOVE"

// "PILE.COUNT"

// "PILE.FACEDOWNCOUNT"

// "PILE.LITERALPILE"

// "PILE.RAND"

// "PILE.DEFINE"

// "PILE.DUP"

// "PILE.EQUAL"

// "PILE.FLUSH"

// "PILE.FROMINT"

// "PILE.POP"

// "PILE.ROT"

// "PILE.SHOVE"

// "PILE.STACKDEPTH"

// "PILE.SWAP"

// "PILE.YANKDUP"

// "PILE.YANK"