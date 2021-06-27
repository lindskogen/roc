/// Low-level operations that get translated directly into e.g. LLVM instructions.
/// These are always wrapped when exposed to end users, and can only make it
/// into an Expr when added directly by can::builtins
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LowLevel {
    StrConcat,
    StrJoinWith,
    StrIsEmpty,
    StrStartsWith,
    StrStartsWithCodePoint,
    StrEndsWith,
    StrSplit,
    StrCountGraphemes,
    StrFromInt,
    StrFromUtf8,
    StrToBytes,
    StrFromFloat,
    ListLen,
    ListGetUnsafe,
    ListSet,
    ListSingle,
    ListRepeat,
    ListReverse,
    ListConcat,
    ListContains,
    ListAppend,
    ListPrepend,
    ListJoin,
    ListRange,
    ListMap,
    ListMap2,
    ListMap3,
    ListMapWithIndex,
    ListKeepIf,
    ListWalk,
    ListWalkUntil,
    ListWalkBackwards,
    ListKeepOks,
    ListKeepErrs,
    ListSortWith,
    ListDrop,
    ListSwap,
    DictSize,
    DictEmpty,
    DictInsert,
    DictRemove,
    DictContains,
    DictGetUnsafe,
    DictKeys,
    DictValues,
    DictUnion,
    DictIntersection,
    DictDifference,
    DictWalk,
    SetFromList,
    NumAdd,
    NumAddWrap,
    NumAddChecked,
    NumSub,
    NumSubWrap,
    NumSubChecked,
    NumMul,
    NumMulWrap,
    NumMulChecked,
    NumGt,
    NumGte,
    NumLt,
    NumLte,
    NumCompare,
    NumDivUnchecked,
    NumRemUnchecked,
    NumIsMultipleOf,
    NumAbs,
    NumNeg,
    NumSin,
    NumCos,
    NumSqrtUnchecked,
    NumLogUnchecked,
    NumRound,
    NumToFloat,
    NumPow,
    NumCeiling,
    NumPowInt,
    NumFloor,
    NumIsFinite,
    NumAtan,
    NumAcos,
    NumAsin,
    NumBitwiseAnd,
    NumBitwiseXor,
    NumBitwiseOr,
    NumShiftLeftBy,
    NumShiftRightBy,
    NumShiftRightZfBy,
    NumIntCast,
    Eq,
    NotEq,
    And,
    Or,
    Not,
    Hash,
    ExpectTrue,
}

impl LowLevel {
    /// is one of the arguments always a function?
    /// An example is List.map.
    pub fn is_higher_order(&self) -> bool {
        use LowLevel::*;

        match self {
            StrConcat
            | StrJoinWith
            | StrIsEmpty
            | StrStartsWith
            | StrStartsWithCodePoint
            | StrEndsWith
            | StrSplit
            | StrCountGraphemes
            | StrFromInt
            | StrFromUtf8
            | StrToBytes
            | StrFromFloat
            | ListLen
            | ListGetUnsafe
            | ListSet
            | ListDrop
            | ListSingle
            | ListRepeat
            | ListReverse
            | ListConcat
            | ListContains
            | ListAppend
            | ListPrepend
            | ListJoin
            | ListRange
            | ListSwap
            | DictSize
            | DictEmpty
            | DictInsert
            | DictRemove
            | DictContains
            | DictGetUnsafe
            | DictKeys
            | DictValues
            | DictUnion
            | DictIntersection
            | DictDifference
            | SetFromList
            | NumAdd
            | NumAddWrap
            | NumAddChecked
            | NumSub
            | NumSubWrap
            | NumSubChecked
            | NumMul
            | NumMulWrap
            | NumMulChecked
            | NumGt
            | NumGte
            | NumLt
            | NumLte
            | NumCompare
            | NumDivUnchecked
            | NumRemUnchecked
            | NumIsMultipleOf
            | NumAbs
            | NumNeg
            | NumSin
            | NumCos
            | NumSqrtUnchecked
            | NumLogUnchecked
            | NumRound
            | NumToFloat
            | NumPow
            | NumCeiling
            | NumPowInt
            | NumFloor
            | NumIsFinite
            | NumAtan
            | NumAcos
            | NumAsin
            | NumBitwiseAnd
            | NumBitwiseXor
            | NumBitwiseOr
            | NumShiftLeftBy
            | NumShiftRightBy
            | NumShiftRightZfBy
            | NumIntCast
            | Eq
            | NotEq
            | And
            | Or
            | Not
            | Hash
            | ExpectTrue => false,

            ListMap | ListMap2 | ListMap3 | ListMapWithIndex | ListKeepIf | ListWalk
            | ListWalkUntil | ListWalkBackwards | ListKeepOks | ListKeepErrs | ListSortWith
            | DictWalk => true,
        }
    }
}
