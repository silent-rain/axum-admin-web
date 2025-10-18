//! 组织相关表
pub mod department;
pub mod department_role_rel;
pub mod position;
pub mod rank;

pub use department::Entity as DepartmentEntity;
pub use department_role_rel::Entity as DepartmentRoleRelEntity;
pub use position::Entity as PositionEntity;
pub use rank::Entity as RankEntity;
