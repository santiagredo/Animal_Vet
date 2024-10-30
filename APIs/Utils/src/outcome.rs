pub enum Outcome<S, F, E> {
    Success(S),
    Failure(F),
    Error(E),
}