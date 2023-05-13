# Scarab Example

Example binary for using the "scarab-engine" library to run a game




Proc Macros to write:

Derive: HasUuid for anything with a field that HasUuid
Derive: RegisteredEnity for an enum with all variants that are RegisteredEntity
    when deriving it should mark the player type and require that "PlayerType: Into\<Self\>"
Derive: MaybeToAction/InputBinding for Enums whose variants all do it
Derive: HasBox for enums where each variant HasBox
Derive: HasBox for tuples where one of the elements HasBox
Derive: HasBox for structs where one of the fields HasBox
