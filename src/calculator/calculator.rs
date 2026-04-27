use unity2::system::collections::Dictionary;
use unity2::system::collections::IDictionary;
use unity2::system::List;
use unity2::{Array, ClassIdentity, Il2CppString};
use unity2::system::collections::Stack;
use unity2::system::collections::IStack;

use crate::data::StructData;
use crate::data::IStructData;


#[unity2::class(namespace = "App", name = "StructCalculatorData`1")]
#[parent(StructData<T>)]
pub struct StructCalculatorData<T: ClassIdentity> {
    #[rename(name = "m_Commands")]
    pub commands: List<CalculatorCommand>,
}

#[unity2::class(namespace = "App")]
#[parent(StructCalculatorData<CalculatorData>)]
pub struct CalculatorData {
    #[backing]
    pub name: Il2CppString,
    #[backing]
    pub condition: Array<Il2CppString>,
    #[backing]
    pub function: Array<Il2CppString>,
}

#[unity2::class(namespace = "App")]
pub struct CalculatorCommand {
    #[backing]
    pub manager: CalculatorManager,
}

#[unity2::class(namespace = "App")]
pub struct CalculatorUtil {
    #[static_field]
    pub null_args: List<f32>,
}

#[unity2::class(namespace = "App")]
#[parent(CalculatorUtil)]
pub struct CalculatorManager {
    #[rename(name = "m_CommandList")]
    pub command_list: CalculatorManagerCommandList,
    #[rename(name = "m_Pool")]
    pub pool: CalculatorManagerStackPool,
}

impl CalculatorManager {
    pub const STACK_MAX: i32 = 32;
    pub const ARGS_MAX: i32 = 16;
}

#[unity2::class(namespace = "App")]
#[parent(CalculatorUtil)]
pub struct StringCalculator {
    #[rename(name = "m_Entitys")]
    pub entitys: List<CalculatorUtilEntity>,
    #[rename(name = "m_Polishs")]
    pub polishs: List<i32>,
}

#[unity2::class(namespace = "", name = "CalculatorUtil.Entity")]
pub struct CalculatorUtilEntity {
    #[rename(name = "m_Type")]
    pub ty: CalculatorUtilType,
    #[rename(name = "m_Name")]
    pub name: Il2CppString,
    #[rename(name = "m_Value")]
    pub value: f32,
    #[rename(name = "m_Code")]
    pub code: i32,
}

#[unity2::class(namespace = "", name = "CalculatorManager.CommandList")]
#[parent(unity2::system::collections::Dictionary<i32, CalculatorCommand>)]
pub struct CalculatorManagerCommandList {}

#[unity2::class(namespace = "", name = "CalculatorManager.CommandStack")]
pub struct CalculatorManagerCommandStack {
    pub value_stack: Stack<f32>,
    pub index_stack: Stack<i32>,
    pub stack_args: List<f32>,
    pub local_args: List<f32>,
    pub temp_args: List<f32>,
}

#[unity2::class(namespace = "", name = "CalculatorManager.StackPool")]
#[parent(unity2::system::collections::Stack<CalculatorManagerCommandStack>)]
pub struct CalculatorManagerStackPool {}

#[unity2::enumeration(namespace = "", name = "CalculatorUtil.Type")]
#[repr(i32)]
pub enum CalculatorUtilType {
    None = 0,
    Add = 1,
    Sub = 2,
    Mul = 3,
    Div = 4,
    Per = 5,
    Or = 6,
    And = 7,
    Xor = 8,
    LShift = 9,
    RShift = 10,
    Equal = 11,
    Nequal = 12,
    Less = 13,
    Lequal = 14,
    Greater = 15,
    Gequal = 16,
    Assign = 17,
    OrAssign = 18,
    AndAssign = 19,
    XorAssign = 20,
    AddAssign = 21,
    SubAssign = 22,
    MulAssign = 23,
    DivAssign = 24,
    PerAssign = 25,
    LSAssign = 26,
    RSAssign = 27,
    Open = 28,
    Close = 29,
    Comma = 30,
    LogOr = 31,
    LogAnd = 32,
    Negative = 33,
    Number = 34,
    Variable = 35,
    Function = 36,
    Args = 37,
    String = 38,
}