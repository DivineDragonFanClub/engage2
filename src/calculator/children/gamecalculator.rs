use unity2::Il2CppString;

use crate::calculator::calculator::CalculatorCommand;
use crate::calculator::calculator::ICalculatorCommand;


#[unity2::class(namespace = "App")]
#[parent(CalculatorCommand)]
pub struct GameCalculatorCommand {
    #[rename(name = "m_Index")]
    pub index: i32,
    #[rename(name = "m_Header")]
    pub header: Il2CppString,
    #[rename(name = "m_IsReverse")]
    pub is_reverse: bool,
}