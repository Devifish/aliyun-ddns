use std::error::Error;

/// 公共错误处理
pub type CommonError = Box<dyn Error>;
