use unity2::Il2CppString;
use unity2::system::collections::List;

use crate::calculator::calculator::CalculatorCommand;
use crate::calculator::calculator::ICalculatorCommand;
use crate::calculator::calculator::StringCalculator;

// CalculatorCommand children
#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct ConditionGetterCommand {
    #[rename(name = "m_Commands")]
    pub commands: List<ConditionGetterCommandCommand>,
    #[rename(name = "m_Name")]
    pub name: Il2CppString,
}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct RandomCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct SinCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct CosCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct TanCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct AbsCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct SqrtCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct LogCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct ExpCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct RoundCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct IntCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct MinCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct MaxCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct ClampCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct LerpCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct PowCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct StrlenCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct CondCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct CompCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct BitCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct ScaleCommand {}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct FunctionCommand {
    #[rename(name = "m_Name")]
    pub name: Il2CppString,
    #[rename(name = "m_Function")]
    pub function: StringCalculator,
}

#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct VariableCommand {
    #[rename(name = "m_Name")]
    pub name: Il2CppString,
    #[rename(name = "m_Value")]
    pub value: f32,
}

#[unity2::class(namespace = "", name = "ConditionGetterCommand.Command")]
pub struct ConditionGetterCommandCommand {
    pub condition: StringCalculator,
    pub function: StringCalculator,
}