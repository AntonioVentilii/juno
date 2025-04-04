mod assert_delete_doc;
mod assert_set_doc;
mod on_delete_doc;
mod on_delete_filtered_docs;
mod on_delete_many_docs;
mod on_set_doc;
mod on_set_many_docs;

use crate::hooks::js::loaders::db::assert_delete_doc::init_assert_delete_doc_loader;
use crate::hooks::js::loaders::db::assert_set_doc::init_assert_set_doc_loader;
use crate::hooks::js::loaders::db::on_delete_doc::init_on_delete_doc_loader;
use crate::hooks::js::loaders::db::on_delete_filtered_docs::init_on_delete_filtered_docs_loader;
use crate::hooks::js::loaders::db::on_delete_many_docs::init_on_delete_many_docs_loader;
use crate::hooks::js::loaders::db::on_set_doc::init_on_set_doc_loader;
use crate::hooks::js::loaders::db::on_set_many_docs::init_on_set_many_docs_loader;
use rquickjs::{Ctx, Error as JsError};

pub fn init_db_loaders(ctx: &Ctx) -> Result<(), JsError> {
    init_assert_set_doc_loader(ctx)?;
    init_assert_delete_doc_loader(ctx)?;

    init_on_set_doc_loader(ctx)?;
    init_on_set_many_docs_loader(ctx)?;
    init_on_delete_doc_loader(ctx)?;
    init_on_delete_many_docs_loader(ctx)?;
    init_on_delete_filtered_docs_loader(ctx)?;

    Ok(())
}
